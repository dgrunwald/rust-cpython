// Copyright (c) 2015 Daniel Grunwald
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this
// software and associated documentation files (the "Software"), to deal in the Software
// without restriction, including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons
// to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
// PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
// FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use ffi;
use python::{Python, ToPythonPointer, PythonObject};
use conversion::ToPyObject;
use objects::{PyObject, PyList};
use err::{self, PyResult, PyErr};
use std::{mem, collections, hash, cmp};

/// Represents a Python `dict`.
pub struct PyDict<'p>(PyObject<'p>);

pyobject_newtype!(PyDict, PyDict_Check, PyDict_Type);

impl <'p> PyDict<'p> {
    /// Creates a new empty dictionary.
    ///
    /// May panic when running out of memory.
    pub fn new(py: Python<'p>) -> PyDict<'p> {
        unsafe {
            err::cast_from_owned_ptr_or_panic(py, ffi::PyDict_New())
        }
    }

    /// Return a new dictionary that contains the same key-value pairs as self.
    /// Corresponds to `dict(self)` in Python.
    pub fn copy(&self) -> PyResult<'p, PyDict<'p>> {
        let py = self.python();
        unsafe {
            err::result_cast_from_owned_ptr(py, ffi::PyDict_Copy(self.as_ptr()))
        }
    }

    /// Empty an existing dictionary of all key-value pairs.
    #[inline]
    pub fn clear(&self) {
        unsafe { ffi::PyDict_Clear(self.as_ptr()) }
    }

    /// Return the number of items in the dictionary.
    /// This is equivalent to len(p) on a dictionary.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { ffi::PyDict_Size(self.as_ptr()) as usize }
    }

    /// Determine if the dictionary contains the specified key.
    /// This is equivalent to the Python expression `key in self`.
    pub fn contains<K>(&self, key: K) -> PyResult<'p, bool> where K: ToPyObject<'p> {
        let py = self.python();
        key.with_borrowed_ptr(py, |key| unsafe {
            match ffi::PyDict_Contains(self.as_ptr(), key) {
                1 => Ok(true),
                0 => Ok(false),
                _ => Err(PyErr::fetch(py))
            }
        })
    }

    /// Gets an item from the dictionary.
    /// Returns None if the item is not present, or if an error occurs.
    pub fn get_item<K>(&self, key: K) -> Option<PyObject<'p>> where K: ToPyObject<'p> {
        let py = self.python();
        key.with_borrowed_ptr(py, |key| unsafe {
            PyObject::from_borrowed_ptr_opt(py,
                ffi::PyDict_GetItem(self.as_ptr(), key))
        })
    }

    /// Sets an item value.
    /// This is equivalent to the Python expression `self[key] = value`.
    pub fn set_item<K, V>(&self, key: K, value: V) -> PyResult<'p, ()> where K: ToPyObject<'p>, V: ToPyObject<'p> {
        let py = self.python();
        key.with_borrowed_ptr(py, move |key|
            value.with_borrowed_ptr(py, |value| unsafe {
                err::error_on_minusone(py,
                    ffi::PyDict_SetItem(self.as_ptr(), key, value))
            }))
    }

    /// Deletes an item.
    /// This is equivalent to the Python expression `del self[key]`.
    pub fn del_item<K>(&self, key: K) -> PyResult<'p, ()> where K: ToPyObject<'p> {
        let py = self.python();
        key.with_borrowed_ptr(py, |key| unsafe {
            err::error_on_minusone(py,
                ffi::PyDict_DelItem(self.as_ptr(), key))
        })
    }

    // List of dict items.
    // This is equivalent to the python expression `list(dict.items())`.
    pub fn items_list(&self) -> PyList<'p> {
        let py = self.python();
        unsafe {
            err::cast_from_owned_ptr_or_panic(py, ffi::PyDict_Items(self.as_ptr()))
        }
    }

    /// Returns the list of (key,value) pairs in this dictionary.
    pub fn items(&self) -> Vec<(PyObject<'p>, PyObject<'p>)> {
        // Note that we don't provide an iterator because
        // PyDict_Next() is unsafe to use when the dictionary might be changed
        // by other python code.
        let py = self.python();
        let mut vec = Vec::with_capacity(self.len());
        unsafe {
            let mut pos = 0;
            let mut key: *mut ffi::PyObject = mem::uninitialized();
            let mut value: *mut ffi::PyObject = mem::uninitialized();
            while ffi::PyDict_Next(self.as_ptr(), &mut pos, &mut key, &mut value) != 0 {
                vec.push((PyObject::from_borrowed_ptr(py, key),
                          PyObject::from_borrowed_ptr(py, value)));
            }
        }
        vec
    }
}

impl <'p, K, V> ToPyObject<'p> for collections::HashMap<K, V>
    where K: hash::Hash+cmp::Eq+ToPyObject<'p>,
          V: ToPyObject<'p>
{
    type ObjectType = PyDict<'p>;

    fn to_py_object(&self, py: Python<'p>) -> PyDict<'p> {
        let dict = PyDict::new(py);
        for (key, value) in self.iter() {
            dict.set_item(key, value).unwrap();
        };
        dict
    }
}

impl <'p, K, V> ToPyObject<'p> for collections::BTreeMap<K, V>
    where K: cmp::Eq+ToPyObject<'p>,
          V: ToPyObject<'p>
{
    type ObjectType = PyDict<'p>;

    fn to_py_object(&self, py: Python<'p>) -> PyDict<'p> {
        let dict = PyDict::new(py);
        for (key, value) in self.iter() {
            dict.set_item(key, value).unwrap();
        };
        dict
    }
}

#[cfg(test)]
mod test {
    use std;
    use python::{Python, PythonObject};
    use conversion::ToPyObject;
    use objects::{PyDict, PyTuple};
    use std::collections::HashMap;

    #[test]
    fn test_len() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut v = HashMap::new();
        let dict = v.to_py_object(py);
        assert_eq!(0, dict.len());
        v.insert(7, 32);
        let dict2 = v.to_py_object(py);
        assert_eq!(1, dict2.len());
    }

    #[test]
    fn test_contains() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut v = HashMap::new();
        v.insert(7, 32);
        let dict = v.to_py_object(py);
        assert_eq!(true, dict.contains(7i32).unwrap());
        assert_eq!(false, dict.contains(8i32).unwrap());
    }

    #[test]
    fn test_get_item() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut v = HashMap::new();
        v.insert(7, 32);
        let dict = v.to_py_object(py);
        assert_eq!(32, dict.get_item(7i32).unwrap().extract::<i32>().unwrap());
        assert_eq!(None, dict.get_item(8i32));
    }

    #[test]
    fn test_set_item() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut v = HashMap::new();
        v.insert(7, 32);
        let dict = v.to_py_object(py);
        assert!(dict.set_item(7i32, 42i32).is_ok()); // change
        assert!(dict.set_item(8i32, 123i32).is_ok()); // insert
        assert_eq!(42i32, dict.get_item(7i32).unwrap().extract::<i32>().unwrap());
        assert_eq!(123i32, dict.get_item(8i32).unwrap().extract::<i32>().unwrap());
    }

    #[test]
    fn test_set_item_does_not_update_original_object() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut v = HashMap::new();
        v.insert(7, 32);
        let dict = v.to_py_object(py);
        assert!(dict.set_item(7i32, 42i32).is_ok()); // change
        assert!(dict.set_item(8i32, 123i32).is_ok()); // insert
        assert_eq!(32i32, *v.get(&7i32).unwrap()); // not updated!
        assert_eq!(None, v.get(&8i32));
    }


    #[test]
    fn test_del_item() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut v = HashMap::new();
        v.insert(7, 32);
        let dict = v.to_py_object(py);
        assert!(dict.del_item(7i32).is_ok());
        assert_eq!(0, dict.len());
        assert_eq!(None, dict.get_item(7i32));
    }

    #[test]
    fn test_del_item_does_not_update_original_object() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut v = HashMap::new();
        v.insert(7, 32);
        let dict = v.to_py_object(py);
        assert!(dict.del_item(7i32).is_ok()); // change
        assert_eq!(32i32, *v.get(&7i32).unwrap()); // not updated!
    }

    #[test]
    fn test_items_list() {
    let gil = Python::acquire_gil();
        let py = gil.python();
        let mut v = HashMap::new();
        v.insert(7, 32);
        v.insert(8, 42);
        v.insert(9, 123);
        let dict = v.to_py_object(py);
        // Can't just compare against a vector of tuples since we don't have a guaranteed ordering.
        let mut key_sum = 0;
        let mut value_sum = 0;
        for el in dict.items_list().into_iter() {
            let tuple = el.cast_into::<PyTuple>().unwrap();
            key_sum += tuple.get_item(0).extract::<i32>().unwrap();
            value_sum += tuple.get_item(1).extract::<i32>().unwrap();
        }
        assert_eq!(7 + 8 + 9, key_sum);
        assert_eq!(32 + 42 + 123, value_sum);
    }

    #[test]
    fn test_items() {
    let gil = Python::acquire_gil();
        let py = gil.python();
        let mut v = HashMap::new();
        v.insert(7, 32);
        v.insert(8, 42);
        v.insert(9, 123);
        let dict = v.to_py_object(py);
        // Can't just compare against a vector of tuples since we don't have a guaranteed ordering.
        let mut key_sum = 0;
        let mut value_sum = 0;
        for (key, value)  in dict.items().into_iter() {
            key_sum += key.extract::<i32>().unwrap();
            value_sum += value.extract::<i32>().unwrap();
        }
        assert_eq!(7 + 8 + 9, key_sum);
        assert_eq!(32 + 42 + 123, value_sum);
    }

}

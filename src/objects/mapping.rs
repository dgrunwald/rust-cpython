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

use python::{Python, PythonObject, ToPythonPointer};
use err::{self, PyErr, PyResult, result_from_owned_ptr};
use super::object::PyObject;
use super::list::PyList;
use ffi::{self, Py_ssize_t};
use std::ffi::CString;
use libc::c_char;
use conversion::{ToPyObject, ExtractPyObject};

/// Represents a Python `Mapping`.
pub struct PyMapping<'p>(PyObject<'p>);

pyobject_newtype!(PyMapping, PyMapping_Check);

impl <'p> PyMapping<'p> {
    /// Gets the length of the mapping.
    #[inline]
    pub fn len(&self) -> PyResult<'p, usize> {
        let py = self.python();
        let v = unsafe {
            ffi::PyMapping_Size(self.as_ptr())
        };
        if v == -1 {
            Err(PyErr::fetch(py))
        } else {
            Ok(v as usize)
        }
    }

    /// Remove string item from the mapping. Equivalent to Python `del o[key]`
    #[inline]
    pub fn del_item_string(&self, key: &str) -> PyResult<'p, ()> {
        let py = self.python();
        let c_key = CString::new(key).unwrap();
        unsafe { 
            err::error_on_minusone(py,
                ffi::PyMapping_DelItemString(self.as_ptr(), c_key.as_ptr() as *mut c_char) )
        }
    }

    /// Remove item from the mapping. Equivalent to Python `del o[key]`
    #[inline]
    pub fn del_item<K>(&self, key: K) -> PyResult<'p, ()>  where K: ToPyObject<'p>{
        let py = self.python();
        key.with_borrowed_ptr(py, |key| unsafe { 
            err::error_on_minusone(py,
                ffi::PyMapping_DelItem(self.as_ptr(), key))
        })
    }

    /// Check if a string item is in the mapping. Equivalent to Python `o[key]`
    #[inline]
    pub fn has_key_string(&self, key : &str) -> bool {
        let c_key = CString::new(key).unwrap();
        let v = unsafe { 
            ffi::PyMapping_HasKeyString(self.as_ptr(), c_key.as_ptr())
        };
        return v == 1;
    }

    /// Check if an item is in the mapping. Equivalent to Python `o[key]`
    #[inline]
    pub fn has_key<K>(&self, key: K) -> bool where K: ToPyObject<'p> {
        let py = self.python();
        let v = key.with_borrowed_ptr(py, |key| unsafe { 
            ffi::PyMapping_HasKey(self.as_ptr(), key)
        });
        return v == 1;
    }

    /// Return the keys as a list
    #[inline]
    pub fn keys(&self) -> PyResult<'p, PyList> {
        let py = self.python();
        let v = try!(unsafe { 
            result_from_owned_ptr(py, ffi::PyMapping_Keys(self.as_ptr()))
        });
        Ok(unsafe { v.unchecked_cast_into::<PyList>() } ) 
    }

    /// Return the values as a list
    #[inline]
    pub fn values(&self) -> PyResult<'p, PyList> { 
        let py = self.python();
        let v = try!(unsafe { 
            result_from_owned_ptr(py, ffi::PyMapping_Values(self.as_ptr()))
        });
        Ok(unsafe { v.unchecked_cast_into::<PyList>() } ) 
    }

    /// Return the values as a list of tuples
    #[inline]
    pub fn items(&self) -> PyResult<'p, PyList> {
        let py = self.python();
        let v = try!(unsafe { 
            result_from_owned_ptr(py, ffi::PyMapping_Items(self.as_ptr()))
        });
        Ok(unsafe { v.unchecked_cast_into::<PyList>() } ) 
    }

    /// Return the string item. Equivalent to Python `o[key]`
    #[inline]
    pub fn get_item_string(&self, key: &str) -> PyResult<'p, PyObject<'p>> {
        let py = self.python();
        let v = unsafe {
            let c_key = CString::new(key).unwrap();
            ffi::PyMapping_GetItemString(self.as_ptr(), c_key.as_ptr())
        };
        if v.is_null() {
            Err(PyErr::fetch(py))
        } else {
            Ok(unsafe{ PyObject::from_owned_ptr(py, v)})
        }
    }

    /// Set the string item. Equivalent to Python `o[key] = value`
    #[inline]
    pub fn set_item_string<V>(&self, key: &str, value: V) -> PyResult<'p, ()> where V: ToPyObject<'p>{
        let py = self.python();
        let c_key = CString::new(key).unwrap();
        value.with_borrowed_ptr(py, |value| unsafe {
            err::error_on_minusone(py, 
                ffi::PyMapping_SetItemString(self.as_ptr(), c_key.as_ptr(), value))
        })
    }
}

#[cfg(test)]
mod test {
    use std;
    use std::collections::BTreeMap;
    use python::{Python, PythonObject};
    use conversion::ToPyObject;
    use objects::{PyMapping, PyTuple};

    #[test]
    fn test_dict_is_mapping() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let btree = BTreeMap::<i32, i32>::new();
        let res = btree.to_py_object(py).into_object().cast_into::<PyMapping>();
        assert!(res.is_ok());
    }

    // Lists can be cast into Mappings in Python3 but not in Python2.
    // http://bugs.python.org/issue5945
    /*
    #[test]
    fn test_list_is_not_mapping() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let list = vec![1, 2, 3];
        let res = list.to_py_object(py).into_object().cast_into::<PyMapping>();
        assert!(res.is_err());
    }
    */

    #[test]
    fn test_empty_mapping() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let btree = BTreeMap::<i32, i32>::new();
        let map = btree.to_py_object(py).into_object().cast_into::<PyMapping>().unwrap();
        assert_eq!(0, map.len().unwrap());
    }

    #[test]
    fn test_mapping_keys() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut btree = BTreeMap::<i32, i32>::new();
        btree.insert(2, 32);
        btree.insert(3, 123);
        btree.insert(5, 123);
        let map = btree.to_py_object(py).into_object().cast_into::<PyMapping>().unwrap();
        let keys = map.keys().unwrap();
        let expected = vec![2, 3, 5];
        for (res, exp) in keys.into_iter().zip(expected.into_iter()) {
            println!("{}", res.extract::<i32>().unwrap());
            assert_eq!(res.extract::<i32>().unwrap(), exp);
        }
    }

    #[test]
    fn test_mapping_values() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut btree = BTreeMap::<i32, i32>::new();
        btree.insert(2, 32);
        btree.insert(3, 123);
        btree.insert(5, 123);
        let map = btree.to_py_object(py).into_object().cast_into::<PyMapping>().unwrap();
        let expected = vec![32, 123, 123];
        let values = map.values().unwrap();
        for (res, exp) in values.into_iter().zip(expected.into_iter()) {
            assert_eq!(res.extract::<i32>().unwrap(), exp);
        }
    }

    #[test]
    fn test_mapping_items() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut btree = BTreeMap::<i32, i32>::new();
        btree.insert(2, 32);
        btree.insert(3, 123);
        btree.insert(5, 123);
        let map = btree.to_py_object(py).into_object().cast_into::<PyMapping>().unwrap();
        let items = map.items().unwrap();
        // BTrees are ordered so we can use a vector of tuples. We can't do this w/ HashMap.
        let expected = vec![(2, 32), (3, 123), (5, 123)]; 
        for (res, exp) in items.into_iter().zip(expected.iter()) {
            let tuple = res.cast_into::<PyTuple>().unwrap();
            let a = tuple.get_item(0).extract::<i32>().unwrap();
            let b = tuple.get_item(1).extract::<i32>().unwrap();
            assert_eq!(a, exp.0);
            assert_eq!(b, exp.1);
        }
    }

    #[test]
    fn test_mapping_has_item() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut btree = BTreeMap::<i32, i32>::new();
        btree.insert(7, 32);
        let map = btree.to_py_object(py).into_object().cast_into::<PyMapping>().unwrap();
        assert_eq!(true, map.has_key(7i32.to_py_object(py)));
        assert_eq!(false, map.has_key(32i32.to_py_object(py)));
    }


    #[test]
    fn test_mapping_has_item_string() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut btree = BTreeMap::<String, i32>::new();
        btree.insert("7".to_owned(), 32);
        let dict = btree.to_py_object(py);
        let map = dict.into_object().cast_into::<PyMapping>().unwrap();
        assert_eq!(true, map.has_key_string("7"));
        assert_eq!(false, map.has_key_string("32"));
    }


    #[test]
    fn test_mapping_get_item_string() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut btree = BTreeMap::<String, i32>::new();
        btree.insert("7".to_owned(), 32);
        btree.insert("8".to_owned(), 42);
        let map = btree.to_py_object(py).into_object().cast_into::<PyMapping>().unwrap();
        assert_eq!(42, map.get_item_string("8").unwrap().extract::<i32>().unwrap());
        assert_eq!(32, map.get_item_string("7").unwrap().extract::<i32>().unwrap());
    }

    #[test]
    fn test_mapping_set_item_string() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut btree = BTreeMap::<i32, i32>::new();
        btree.insert(7, 32);
        let map = btree.to_py_object(py).into_object().cast_into::<PyMapping>().unwrap();
        assert!(map.set_item_string("7", 42).is_ok());
        assert_eq!(42, map.get_item_string("7").unwrap().extract::<i32>().unwrap());
    }

    #[test]
    fn test_mapping_del_item_string() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut btree = BTreeMap::<String, i32>::new();
        btree.insert("7".to_owned(), 32);
        let map = btree.to_py_object(py).into_object().cast_into::<PyMapping>().unwrap();
        assert!(map.del_item_string("7").is_ok());
        assert_eq!(0, map.len().unwrap());
    }

}

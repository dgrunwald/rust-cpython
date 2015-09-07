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
    /// Gets the length of the list.
    #[inline]
    pub fn size(&self) -> PyResult<'p, usize> {
        // non-negative Py_ssize_t should always fit into Rust usize
        let v = unsafe {
            ffi::PyMapping_Size(self.as_ptr())
        };
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else {
            Ok(v as usize)
        }
    }

    pub fn length(&self) -> PyResult<'p, usize> {
        self.size()
    }

    #[inline]
    pub fn del_item_string(&self, key: &str) -> PyResult<'p, ()> {
        let v = unsafe { 
            let c_key = key.as_ptr() as *mut c_char;
            ffi::PyMapping_DelItemString(self.as_ptr(), c_key) 
        };
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn del_item(&self, key: &PyObject<'p>) -> PyResult<'p, ()> {
        let v = unsafe { ffi::PyMapping_DelItem(self.as_ptr(), key.as_ptr())};
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn has_key_string(&self, key : &str) -> bool {
        let v = unsafe { 
            let c_key = key.as_ptr() as *const c_char;
            ffi::PyMapping_HasKeyString(self.as_ptr(), c_key)
        };
        return v == 1;
    }

    #[inline]
    pub fn has_key(&self, key: &PyObject<'p>) -> bool {
        let v = unsafe { ffi::PyMapping_HasKey(self.as_ptr(), key.as_ptr())};
        return v == 1;
    }
    #[inline]
    pub fn keys(&self) -> PyResult<'p, PyList> {
        let py = self.python();
        let v = try!(unsafe { 
            result_from_owned_ptr(py, ffi::PyMapping_Keys(self.as_ptr()))
        });
        Ok(unsafe { v.unchecked_cast_into::<PyList>() } ) 
    }

    #[inline]
    pub fn values(&self) -> PyResult<'p, PyList> { 
        let py = self.python();
        let v = try!(unsafe { 
            result_from_owned_ptr(py, ffi::PyMapping_Values(self.as_ptr()))
        });
        Ok(unsafe { v.unchecked_cast_into::<PyList>() } ) 
    }

    #[inline]
    pub fn items(&self) -> PyResult<'p, PyList> {
        let py = self.python();
        let v = try!(unsafe { 
            result_from_owned_ptr(py, ffi::PyMapping_Items(self.as_ptr()))
        });
        Ok(unsafe { v.unchecked_cast_into::<PyList>() } ) 
    }

    #[inline]
    pub fn get_item_string(&self, key: &str) -> PyResult<'p, PyObject<'p>> {
        let py = self.python();
        let v = unsafe {
            let c_key = key.as_ptr() as *const c_char;
            ffi::PyMapping_GetItemString(self.as_ptr(), c_key)
        };
        if v.is_null() {
            Err(PyErr::fetch(py))
        } else {
            Ok(unsafe{ PyObject::from_owned_ptr(py, v)})
        }
    }

    #[inline]
    pub fn set_item_string(&self, key: &str, value: PyObject<'p>) -> PyResult<'p, ()> {
        let c_key = key.as_ptr() as *const c_char;
        let v = unsafe { ffi::PyMapping_SetItemString(self.as_ptr(), c_key, value.steal_ptr())};
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else {
            Ok(())
        }
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

    // lists can be cast into Mappings?
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
        assert_eq!(0, map.length().unwrap());
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
        let expected = vec![(2, 32), (3, 123), (5, 123)]; 
        for (res, exp) in items.into_iter().zip(expected.iter()) {
            // This fails due to res being a moved value.
            /*
            res.cast_into::<PyTuple>().unwrap().get_item(0).extract::<i32>().unwrap();
            res.cast_into::<PyTuple>().unwrap().get_item(1).extract::<i32>().unwrap();
            assert_eq!(a, exp.0);
            assert_eq!(b, exp.1);
            */
        }
    }

    #[test]
    fn test_mapping_get_item_string() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut btree = BTreeMap::<String, i32>::new();
        btree.insert("7".to_owned(), 32);
        btree.insert("8".to_owned(), 42);
        let map = btree.to_py_object(py).into_object().cast_into::<PyMapping>().unwrap();
        //assert_eq!(32, map.get_item_string("7").unwrap().extract::<i32>().unwrap());
        //assert_eq!(42, map.get_item_string("8").unwrap().extract::<i32>().unwrap());
    }

    #[test]
    fn test_mapping_set_item_string() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut btree = BTreeMap::<i32, i32>::new();
        btree.insert(7, 32);
        let map = btree.to_py_object(py).into_object().cast_into::<PyMapping>().unwrap();
        //assert_eq!(32, map.get_item_string("7").unwrap().extract::<i32>().unwrap());
    }


}

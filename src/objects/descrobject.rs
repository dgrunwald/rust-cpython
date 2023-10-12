use std::ptr;

use crate::conversion::ToPyObject;
use crate::err::{self, PyErr, PyResult};
use crate::ffi;
use crate::objects::{PyObject, PyTuple};
use crate::python::{Python, PythonObject, ToPythonPointer};

/// Represents a read-only Python dictionary
pub struct PyDictProxy(PyObject);

pyobject_newtype!(PyDictProxy, PyDictProxy_Check, PyDictProxy_Type);

impl PyDictProxy {
    #[inline]
    pub fn len(&self, _py: Python) -> usize {
        unsafe { ffi::PyObject_Size(self.0.as_ptr()) as usize }
    }

    pub fn get_item<K>(&self, py: Python, key: K) -> Option<PyObject>
    where
        K: ToPyObject,
    {
    	key.with_borrowed_ptr(py, |key| unsafe {
            PyObject::from_borrowed_ptr_opt(py, ffi::PyObject_GetItem(self.0.as_ptr(), key))
        })
    }

    pub fn contains<K>(&self, py: Python, key: K) -> PyResult<bool>
    where
        K: ToPyObject,
    {
        key.with_borrowed_ptr(py, |key| unsafe {
            match ffi::PyMapping_HasKey(self.0.as_ptr(), key) {
                1 => Ok(true),
                0 => Ok(false),
                _ => Err(PyErr::fetch(py)),
            }
        })
    }

    pub fn keys(&self, py: Python) -> PyObject {
        // Returns a PySequence object
        unsafe {
        	PyObject::from_borrowed_ptr(py, ffi::PyMapping_Keys(self.0.as_ptr()))
        }
    }

    pub fn values(&self, py: Python) -> PyObject {
        // Returns a PySequence object
        unsafe {
        	PyObject::from_borrowed_ptr(py, ffi::PyMapping_Values(self.0.as_ptr()))
        }
    }

    pub fn items(&self, py: Python) -> PyObject {
        // Returns a PySequence object
        unsafe {
        	PyObject::from_borrowed_ptr(py, ffi::PyMapping_Items(self.0.as_ptr()))
        }
    }
}



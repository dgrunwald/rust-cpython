use libc::{c_char, c_int, c_void};

use crate::methodobject::PyMethodDef;
use crate::object::*;
use crate::structmember::PyMemberDef;
use crate::pyport::Py_ssize_t;

pub type getter = unsafe extern "C" fn(slf: *mut PyObject, closure: *mut c_void) -> *mut PyObject;

pub type setter =
    unsafe extern "C" fn(slf: *mut PyObject, value: *mut PyObject, closure: *mut c_void) -> c_int;

#[repr(C)]
#[derive(Copy)]
pub struct PyGetSetDef {
    #[cfg(not(Py_3_7))]
    pub name: *mut c_char,
    #[cfg(Py_3_7)]
    pub name: *const c_char,
    pub get: Option<getter>,
    pub set: Option<setter>,
    #[cfg(not(Py_3_7))]
    pub doc: *mut c_char,
    #[cfg(Py_3_7)]
    pub doc: *const c_char,
    pub closure: *mut c_void,
}

impl Clone for PyGetSetDef {
    #[inline]
    fn clone(&self) -> PyGetSetDef {
        *self
    }
}


#[inline(always)]
pub unsafe fn PyDictProxy_Check(op: *mut PyObject) -> c_int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_DEFAULT | Py_TPFLAGS_HAVE_GC)
}

#[inline(always)]
pub unsafe fn PyDictProxy_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyDictProxy_Type) as c_int
}


#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyClassMethodDescr_Type: PyTypeObject;
    pub static mut PyGetSetDescr_Type: PyTypeObject;
    pub static mut PyMemberDescr_Type: PyTypeObject;
    pub static mut PyMethodDescr_Type: PyTypeObject;
    pub static mut PyWrapperDescr_Type: PyTypeObject;
    pub static mut PyDictProxy_Type: PyTypeObject;

    pub fn PyDescr_NewMethod(arg1: *mut PyTypeObject, arg2: *mut PyMethodDef) -> *mut PyObject;
    pub fn PyDescr_NewClassMethod(arg1: *mut PyTypeObject, arg2: *mut PyMethodDef)
        -> *mut PyObject;
    pub fn PyDescr_NewMember(arg1: *mut PyTypeObject, arg2: *mut PyMemberDef) -> *mut PyObject;
    pub fn PyDescr_NewGetSet(arg1: *mut PyTypeObject, arg2: *mut PyGetSetDef) -> *mut PyObject;
    pub fn PyDictProxy_New(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyWrapper_New(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;

    pub static mut PyProperty_Type: PyTypeObject;
}

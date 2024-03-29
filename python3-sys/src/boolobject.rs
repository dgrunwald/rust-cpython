use libc::{c_int, c_long};

use crate::longobject::PyLongObject;
use crate::object::*;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyBool_Type: PyTypeObject;
    static mut _Py_FalseStruct: PyLongObject;
    static mut _Py_TrueStruct: PyLongObject;
    pub fn PyBool_FromLong(arg1: c_long) -> *mut PyObject;
}

#[inline(always)]
pub unsafe fn PyBool_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyBool_Type) as c_int
}

#[inline(always)]
pub unsafe fn Py_False() -> *mut PyObject {
    &mut _Py_FalseStruct as *mut PyLongObject as *mut PyObject
}

#[inline(always)]
pub unsafe fn Py_True() -> *mut PyObject {
    &mut _Py_TrueStruct as *mut PyLongObject as *mut PyObject
}

#[inline(always)]
pub unsafe fn Py_IsFalse(obj: *mut PyObject) -> c_int {
    (obj == Py_False()) as c_int
}

#[inline(always)]
pub unsafe fn Py_IsTrue(obj: *mut PyObject) -> c_int {
    (obj == Py_True()) as c_int
}

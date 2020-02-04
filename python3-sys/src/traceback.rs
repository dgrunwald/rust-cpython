use libc::c_int;

use crate::object::*;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyTraceBack_Here(arg1: *mut crate::PyFrameObject) -> c_int;
    pub fn PyTraceBack_Print(arg1: *mut PyObject, arg2: *mut PyObject) -> c_int;

    pub static mut PyTraceBack_Type: PyTypeObject;
}

#[inline(always)]
pub unsafe fn PyTraceBack_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyTraceBack_Type) as c_int
}

use libc::c_int;

use crate::object::PyObject;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyEval_EvalCode(
        co: *mut PyObject,
        globals: *mut PyObject,
        locals: *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyEval_EvalCodeEx(
        co: *mut PyObject,
        globals: *mut PyObject,
        locals: *mut PyObject,
        args: *const *mut PyObject,
        argc: c_int,
        kwds: *const *mut PyObject,
        kwdc: c_int,
        defs: *const *mut PyObject,
        defc: c_int,
        kwdefs: *mut PyObject,
        closure: *mut PyObject,
    ) -> *mut PyObject;
}

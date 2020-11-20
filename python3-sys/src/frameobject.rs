use libc::{c_char, c_int};

use crate::code::{PyCodeObject, CO_MAXBLOCKS};
use crate::object::*;
use crate::pystate::PyThreadState;

#[cfg(not(Py_LIMITED_API))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyTryBlock {
    pub b_type: c_int,
    pub b_handler: c_int,
    pub b_level: c_int,
}

#[cfg(Py_LIMITED_API)]
pub enum PyFrameObject {}

#[cfg(not(Py_LIMITED_API))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyFrameObject {
    pub ob_base: PyVarObject,
    pub f_back: *mut PyFrameObject,       /* previous frame, or NULL */
    pub f_code: *mut PyCodeObject,        /* code segment */
    pub f_builtins: *mut PyObject,        /* builtin symbol table (PyDictObject) */
    pub f_globals: *mut PyObject,         /* global symbol table (PyDictObject) */
    pub f_locals: *mut PyObject,          /* local symbol table (any mapping) */
    pub f_valuestack: *mut *mut PyObject, /* points after the last local */
    /* Next free slot in f_valuestack.  Frame creation sets to f_valuestack.
    Frame evaluation usually NULLs it, but a frame that yields sets it
    to the current stack top. */
    pub f_stacktop: *mut *mut PyObject,
    pub f_trace: *mut PyObject, /* Trace function */

    #[cfg(not(Py_3_7))]
    pub f_exc_type: *mut PyObject,
    #[cfg(not(Py_3_7))]
    pub f_exc_value: *mut PyObject,
    #[cfg(not(Py_3_7))]
    pub f_exc_traceback: *mut PyObject,

    #[cfg(Py_3_7)]
    pub f_trace_lines: c_char,
    #[cfg(Py_3_7)]
    pub f_trace_opcodes: c_char,

    #[cfg(not(Py_3_4))]
    pub f_tstate: *mut PyThreadState,

    #[cfg(Py_3_4)]
    pub f_gen: *mut PyObject,

    pub f_lasti: c_int, /* Last instruction if called */
    /* Call PyFrame_GetLineNumber() instead of reading this field
     directly.  As of 2.3 f_lineno is only valid when tracing is
     active (i.e. when f_trace is set).  At other times we use
     PyCode_Addr2Line to calculate the line from the current
    bytecode index. */
    pub f_lineno: c_int, /* Current line number */
    pub f_iblock: c_int, /* index in f_blockstack */
    #[cfg(Py_3_4)]
    pub f_executing: c_char, /* whether the frame is still executing */
    pub f_blockstack: [PyTryBlock; CO_MAXBLOCKS], /* for try and loop blocks */
    pub f_localsplus: [*mut PyObject; 1], /* locals+stack, dynamically sized */
}

#[cfg(not(Py_LIMITED_API))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyFrame_Type: PyTypeObject;
}

#[cfg(not(Py_LIMITED_API))]
#[inline]
pub unsafe fn PyFrame_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyFrame_Type) as c_int
}

#[cfg(not(Py_LIMITED_API))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyFrame_New(
        tstate: *mut PyThreadState,
        code: *mut PyCodeObject,
        globals: *mut PyObject,
        locals: *mut PyObject,
    ) -> *mut PyFrameObject;

    pub fn PyFrame_BlockSetup(
        f: *mut PyFrameObject,
        _type: c_int,
        handler: c_int,
        level: c_int,
    ) -> ();
    pub fn PyFrame_BlockPop(f: *mut PyFrameObject) -> *mut PyTryBlock;

    pub fn PyFrame_LocalsToFast(f: *mut PyFrameObject, clear: c_int) -> ();
    #[cfg(Py_3_4)]
    pub fn PyFrame_FastToLocalsWithError(f: *mut PyFrameObject) -> c_int;
    pub fn PyFrame_FastToLocals(f: *mut PyFrameObject) -> ();

    #[cfg(not(Py_3_9))]
    pub fn PyFrame_ClearFreeList() -> c_int;
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyFrame_GetLineNumber(f: *mut PyFrameObject) -> c_int;

    #[cfg(Py_3_9)]
    pub fn PyFrame_GetCode(frame: *mut PyFrameObject) -> *mut PyCodeObject;
}

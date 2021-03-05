use libc::{c_char, c_int, c_void};

use crate::object::PyObject;
use crate::pystate::PyThreadState;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[deprecated(since = "0.5.2", note = "Deprecated since Python 3.9")]
    pub fn PyEval_CallObjectWithKeywords(
        callable: *mut PyObject,
        obj: *mut PyObject,
        kwargs: *mut PyObject,
    ) -> *mut PyObject;
}

#[inline]
#[deprecated(since = "0.5.2", note = "Deprecated since Python 3.9")]
pub unsafe fn PyEval_CallObject(callable: *mut PyObject, arg: *mut PyObject) -> *mut PyObject {
    #[allow(deprecated)]
    PyEval_CallObjectWithKeywords(callable, arg, core::ptr::null_mut())
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[deprecated(since = "0.5.2", note = "Deprecated since Python 3.9")]
    pub fn PyEval_CallFunction(
        callable: *mut PyObject,
        format: *const c_char,
        ...
    ) -> *mut PyObject;
    #[deprecated(since = "0.5.2", note = "Deprecated since Python 3.9")]
    pub fn PyEval_CallMethod(
        obj: *mut PyObject,
        name: *const c_char,
        format: *const c_char,
        ...
    ) -> *mut PyObject;
    pub fn PyEval_GetBuiltins() -> *mut PyObject;
    pub fn PyEval_GetGlobals() -> *mut PyObject;
    pub fn PyEval_GetLocals() -> *mut PyObject;
    pub fn PyEval_GetFrame() -> *mut crate::PyFrameObject;
    pub fn Py_AddPendingCall(
        func: Option<extern "C" fn(arg1: *mut c_void) -> c_int>,
        arg: *mut c_void,
    ) -> c_int;
    pub fn Py_MakePendingCalls() -> c_int;
    pub fn Py_SetRecursionLimit(arg1: c_int) -> ();
    pub fn Py_GetRecursionLimit() -> c_int;

    ignore! {
        fn _Py_CheckRecursiveCall(_where: *mut c_char) -> c_int;
        static mut _Py_CheckRecursionLimit: c_int;
    }

    #[cfg(Py_3_9)]
    pub fn Py_EnterRecursiveCall(_where: *const c_char) -> c_int;
    #[cfg(Py_3_9)]
    pub fn Py_LeaveRecursiveCall() -> c_void;
}

// TODO: Py_EnterRecursiveCall for Python <3.9

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyEval_GetFuncName(arg1: *mut PyObject) -> *const c_char;
    pub fn PyEval_GetFuncDesc(arg1: *mut PyObject) -> *const c_char;
    #[cfg(not(Py_3_7))]
    pub fn PyEval_GetCallStats(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyEval_EvalFrame(arg1: *mut crate::PyFrameObject) -> *mut PyObject;
    pub fn PyEval_EvalFrameEx(f: *mut crate::PyFrameObject, exc: c_int) -> *mut PyObject;
    pub fn PyEval_SaveThread() -> *mut PyThreadState;
    pub fn PyEval_RestoreThread(arg1: *mut PyThreadState) -> ();
}

#[cfg(any(Py_3_7, py_sys_config = "WITH_THREAD"))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyEval_ThreadsInitialized() -> c_int;
    pub fn PyEval_InitThreads() -> ();
    #[deprecated(
        since = "0.2.1",
        note = "Deprecated since Python 3.2: This function does not update the current thread state. Please use PyEval_RestoreThread() or PyEval_AcquireThread() instead."
    )]
    pub fn PyEval_AcquireLock() -> ();
    #[deprecated(
        since = "0.2.1",
        note = "Deprecated since Python 3.2: This function does not update the current thread state. Please use PyEval_RestoreThread() or PyEval_AcquireThread() instead."
    )]
    pub fn PyEval_ReleaseLock() -> ();
    pub fn PyEval_AcquireThread(tstate: *mut PyThreadState) -> ();
    pub fn PyEval_ReleaseThread(tstate: *mut PyThreadState) -> ();
    #[cfg(not(Py_3_8))]
    pub fn PyEval_ReInitThreads() -> ();
}

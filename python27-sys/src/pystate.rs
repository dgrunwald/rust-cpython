use libc::{c_int, c_long};
use object::{PyObject, PyTypeObject};

use code::{PyCodeObject, CO_MAXBLOCKS};
use pyport::Py_ssize_t;


#[allow(missing_copy_implementations)]
pub enum PyInterpreterState { }

#[repr(C)]
#[derive(Copy)]
pub struct PyFrameObject {
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_next: *mut PyObject,
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
    pub ob_size: Py_ssize_t,
    pub f_back: *mut PyFrameObject,	/* previous frame, or NULL */
    pub f_code: *mut PyCodeObject,	/* code segment */
    pub f_builtins: *mut PyObject,	/* builtin symbol table (PyDictObject) */
    pub f_globals: *mut PyObject,	/* global symbol table (PyDictObject) */
    pub f_locals: *mut PyObject,		/* local symbol table (any mapping) */
    pub f_valuestack: *mut PyObject,	/* points after the last local */
    /* Next free slot in f_valuestack.  Frame creation sets to f_valuestack.
       Frame evaluation usually NULLs it, but a frame that yields sets it
       to the current stack top. */
    pub f_stacktop: Option<PyObject>,
    pub f_trace: Option<Py_tracefunc>,		/* Trace function */

    /* If an exception is raised in this frame, the next three are used to
     * record the exception info (if any) originally in the thread state.  See
     * comments before set_exc_info() -- it's not obvious.
     * Invariant:  if _type is NULL, then so are _value and _traceback.
     * Desired invariant:  all three are NULL, or all three are non-NULL.  That
     * one isn't currently true, but "should be".
     */
    pub f_exc_type: *mut PyObject,
    pub f_exc_value: *mut PyObject,
    pub f_exc_traceback: *mut PyObject,

    pub f_tstate: *mut PyThreadState,

    pub f_lasti: c_int,		/* Last instruction if called */
    /* Call PyFrame_GetLineNumber() instead of reading this field
       directly.  As of 2.3 f_lineno is only valid when tracing is
       active (i.e. when f_trace is set).  At other times we use
       PyCode_Addr2Line to calculate the line from the current
       bytecode index. */
    pub f_lineno: c_int,		/* Current line number */
    pub f_iblock: c_int,		/* index in f_blockstack */
    pub f_blockstack: [PyObject; CO_MAXBLOCKS], /* for try and loop blocks */
    pub  f_localsplus: [PyObject; 1]	/* locals+stack, dynamically sized */
}

#[inline(always)]
pub unsafe fn PyFrameObject_Check(op : *mut PyObject) -> c_int {
    1
}

impl Clone for PyFrameObject {
    #[inline] fn clone(&self) -> PyFrameObject { *self }
}

pub type Py_tracefunc =
    unsafe extern "C" fn
                              (arg1: *mut PyObject, arg2: *mut PyFrameObject,
                               arg3: c_int, arg4: *mut PyObject)
                              -> c_int;

/* The following values are used for 'what' for tracefunc functions: */
pub const PyTrace_CALL : c_int = 0;
pub const PyTrace_EXCEPTION : c_int = 1;
pub const PyTrace_LINE : c_int = 2;
pub const PyTrace_RETURN : c_int = 3;
pub const PyTrace_C_CALL : c_int = 4;
pub const PyTrace_C_EXCEPTION : c_int = 5;
pub const PyTrace_C_RETURN : c_int = 6;

#[repr(C)]
#[derive(Copy)]
pub struct PyThreadState {
    pub next: *mut PyThreadState,
    pub interp: *mut PyInterpreterState,
    pub frame: *mut PyFrameObject,
    pub recursion_depth: c_int,
    pub tracing: c_int,
    pub use_tracing: c_int,
    pub c_profilefunc: Option<Py_tracefunc>,
    pub c_tracefunc: Option<Py_tracefunc>,
    pub c_profileobj: *mut PyObject,
    pub c_traceobj: *mut PyObject,
    pub curexc_type: *mut PyObject,
    pub curexc_value: *mut PyObject,
    pub curexc_traceback: *mut PyObject,
    pub exc_type: *mut PyObject,
    pub exc_value: *mut PyObject,
    pub exc_traceback: *mut PyObject,
    pub dict: *mut PyObject,
    pub tick_counter: c_int,
    pub gilstate_counter: c_int,
    pub async_exc: *mut PyObject,
    pub thread_id: c_long,
    pub trash_delete_nesting: c_int,
    pub trash_delete_later: *mut PyObject,
}

impl Clone for PyThreadState {
    #[inline] fn clone(&self) -> PyThreadState { *self }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PyGILState_STATE {
    PyGILState_LOCKED,
    PyGILState_UNLOCKED
}


extern "C" {
    static mut _PyThreadState_Current: *mut PyThreadState;
    //static mut _PyThreadState_GetFrame: PyThreadFrameGetter;

    pub fn PyInterpreterState_New() -> *mut PyInterpreterState;
    pub fn PyInterpreterState_Clear(arg1: *mut PyInterpreterState);
    pub fn PyInterpreterState_Delete(arg1: *mut PyInterpreterState);
    pub fn PyThreadState_New(arg1: *mut PyInterpreterState)
     -> *mut PyThreadState;
    pub fn _PyThreadState_Prealloc(arg1: *mut PyInterpreterState)
     -> *mut PyThreadState;
    pub fn _PyThreadState_Init(arg1: *mut PyThreadState);
    pub fn PyThreadState_Clear(arg1: *mut PyThreadState);
    pub fn PyThreadState_Delete(arg1: *mut PyThreadState);
    #[cfg(py_sys_config="WITH_THREAD")]
    pub fn PyThreadState_DeleteCurrent();
    pub fn PyThreadState_Get() -> *mut PyThreadState;
    pub fn PyThreadState_Swap(arg1: *mut PyThreadState) -> *mut PyThreadState;
    pub fn PyThreadState_GetDict() -> *mut PyObject;
    pub fn PyThreadState_SetAsyncExc(arg1: c_long,
                                     arg2: *mut PyObject) -> c_int;
    pub fn PyGILState_Ensure() -> PyGILState_STATE;
    pub fn PyGILState_Release(arg1: PyGILState_STATE);
    pub fn PyGILState_GetThisThreadState() -> *mut PyThreadState;
    fn _PyThread_CurrentFrames() -> *mut PyObject;
    pub fn PyInterpreterState_Head() -> *mut PyInterpreterState;
    pub fn PyInterpreterState_Next(arg1: *mut PyInterpreterState)
     -> *mut PyInterpreterState;
    pub fn PyInterpreterState_ThreadHead(arg1: *mut PyInterpreterState)
     -> *mut PyThreadState;
    pub fn PyThreadState_Next(arg1: *mut PyThreadState) -> *mut PyThreadState;

    pub fn PyFrame_GetLineNumber(f: *mut PyFrameObject) -> c_int;
}

#[cfg(py_sys_config="Py_DEBUG")]
#[inline(always)]
pub unsafe fn PyThreadState_GET() -> *mut PyThreadState {
    PyThreadState_Get()
}

#[cfg(not(py_sys_config="Py_DEBUG"))]
#[inline(always)]
pub unsafe fn PyThreadState_GET() -> *mut PyThreadState {
    _PyThreadState_Current
}

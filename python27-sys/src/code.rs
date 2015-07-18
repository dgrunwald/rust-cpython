use libc::{c_char, c_int, c_void};
use object::*;
use pyport::Py_ssize_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyCodeObject {
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_next: *mut PyObject,
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
    pub co_argcount: c_int,		/* #arguments, except *args */
    pub co_nlocals: c_int,		/* #local variables */
    pub co_stacksize: c_int,		/* #entries needed for evaluation stack */
    pub co_flags: c_int,		/* CO_..., see below */
    pub co_code: *mut PyObject,		/* instruction opcodes */
    pub co_consts: *mut PyObject,	/* list (constants used) */
    pub co_names: *mut PyObject,		/* list of strings (names used) */
    pub co_varnames: *mut PyObject,	/* tuple of strings (local variable names) */
    pub co_freevars: *mut PyObject,	/* tuple of strings (free variable names) */
    pub co_cellvars: *mut PyObject,      /* tuple of strings (cell variable names) */
    /* The rest doesn't count for hash/cmp */
    pub co_filename: *mut PyObject,	/* string (where it was loaded from) */
    pub co_name: *mut PyObject,		/* string (name, for reference) */
    pub co_firstlineno: c_int,		/* first source line number */
    pub co_lnotab: *mut PyObject,	/* string (encoding addr<->lineno mapping) See
				   Objects/lnotab_notes.txt for details. */
    pub co_zombieframe: *mut c_void,     /* for optimization only (see frameobject.c) */
    pub co_weakreflist: *mut PyObject   /* to support weakrefs to code objects */
}

/* Masks for co_flags */
pub const CO_OPTIMIZED : c_int = 0x0001;
pub const CO_NEWLOCALS : c_int = 0x0002;
pub const CO_VARARGS : c_int = 0x0004;
pub const CO_VARKEYWORDS : c_int = 0x0008;
pub const CO_NESTED : c_int = 0x0010;
pub const CO_GENERATOR : c_int = 0x0020;
/* The CO_NOFREE flag is set if there are no free or cell variables.
   This information is redundant, but it allows a single flag test
   to determine whether there is any extra work to be done when the
   call frame it setup.
*/
pub const CO_NOFREE : c_int = 0x0040;

pub const CO_FUTURE_DIVISION : c_int = 0x2000;
pub const CO_FUTURE_ABSOLUTE_IMPORT : c_int = 0x4000; /* do absolute imports by default */
pub const CO_FUTURE_WITH_STATEMENT : c_int = 0x8000;
pub const CO_FUTURE_PRINT_FUNCTION : c_int = 0x10000;
pub const CO_FUTURE_UNICODE_LITERALS : c_int = 0x20000;

pub const CO_MAXBLOCKS: usize = 20; /* Max static block nesting within a function */

extern "C" {
    pub static mut PyCode_Type: PyTypeObject;

    pub fn PyCode_New(arg1: c_int, arg2: c_int,
                      arg3: c_int, arg4: c_int,
                      arg5: *mut PyObject, arg6: *mut PyObject,
                      arg7: *mut PyObject, arg8: *mut PyObject,
                      arg9: *mut PyObject, arg10: *mut PyObject,
                      arg11: *mut PyObject, arg12: *mut PyObject,
                      arg13: c_int, arg14: *mut PyObject)
     -> *mut PyCodeObject;
    pub fn PyCode_NewEmpty(filename: *const c_char,
                           funcname: *const c_char,
                           firstlineno: c_int) -> *mut PyCodeObject;
    pub fn PyCode_Addr2Line(arg1: *mut PyCodeObject, arg2: c_int)
     -> c_int;
    //fn _PyCode_CheckLineNumber(co: *mut PyCodeObject,
    //                               lasti: c_int,
    //                               bounds: *mut PyAddrPair) -> c_int;
    pub fn PyCode_Optimize(code: *mut PyObject, consts: *mut PyObject,
                           names: *mut PyObject, lineno_obj: *mut PyObject)
     -> *mut PyObject;
}

#[inline(always)]
pub unsafe fn PyCode_Check(op : *mut PyObject) -> c_int {
    let u : *mut PyTypeObject = &mut PyCode_Type;
    (Py_TYPE(op) == u) as c_int
}

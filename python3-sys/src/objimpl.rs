use libc::{c_int, c_void, size_t};

use crate::object::*;
use crate::pyport::Py_ssize_t;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg(not(all(py_sys_config = "Py_DEBUG", not(Py_3_4))))]
    pub fn PyObject_Malloc(size: size_t) -> *mut c_void;
    #[cfg(Py_3_5)]
    pub fn PyObject_Calloc(nelem: size_t, elsize: size_t) -> *mut c_void;
    #[cfg(not(all(py_sys_config = "Py_DEBUG", not(Py_3_4))))]
    pub fn PyObject_Realloc(ptr: *mut c_void, new_size: size_t) -> *mut c_void;
    #[cfg(not(all(py_sys_config = "Py_DEBUG", not(Py_3_4))))]
    pub fn PyObject_Free(ptr: *mut c_void) -> ();

    #[cfg(all(py_sys_config = "Py_DEBUG", not(Py_3_4)))]
    pub fn _PyObject_DebugMalloc(arg1: size_t) -> *mut c_void;
    #[cfg(all(py_sys_config = "Py_DEBUG", not(Py_3_4)))]
    pub fn _PyObject_DebugRealloc(arg1: *mut c_void, arg2: size_t) -> *mut c_void;
    #[cfg(all(py_sys_config = "Py_DEBUG", not(Py_3_4)))]
    pub fn _PyObject_DebugFree(arg1: *mut c_void);

    #[cfg(all(not(Py_LIMITED_API), Py_3_4))]
    pub fn _Py_GetAllocatedBlocks() -> Py_ssize_t;
    pub fn PyObject_Init(arg1: *mut PyObject, arg2: *mut PyTypeObject) -> *mut PyObject;
    pub fn PyObject_InitVar(
        arg1: *mut PyVarObject,
        arg2: *mut PyTypeObject,
        arg3: Py_ssize_t,
    ) -> *mut PyVarObject;
    pub fn _PyObject_New(arg1: *mut PyTypeObject) -> *mut PyObject;
    pub fn _PyObject_NewVar(arg1: *mut PyTypeObject, arg2: Py_ssize_t) -> *mut PyVarObject;

    pub fn PyGC_Collect() -> Py_ssize_t;
}

#[cfg(all(py_sys_config = "Py_DEBUG", not(Py_3_4)))]
pub use self::_PyObject_DebugFree as PyObject_Free;
#[cfg(all(py_sys_config = "Py_DEBUG", not(Py_3_4)))]
pub use self::_PyObject_DebugMalloc as PyObject_Malloc;
#[cfg(all(py_sys_config = "Py_DEBUG", not(Py_3_4)))]
pub use self::_PyObject_DebugRealloc as PyObject_Realloc;

#[repr(C)]
#[derive(Copy)]
#[cfg(all(not(Py_LIMITED_API), Py_3_4))]
pub struct PyObjectArenaAllocator {
    pub ctx: *mut c_void,
    pub alloc: Option<extern "C" fn(ctx: *mut c_void, size: size_t) -> *mut c_void>,
    pub free: Option<extern "C" fn(ctx: *mut c_void, ptr: *mut c_void, size: size_t) -> ()>,
}
#[cfg(all(not(Py_LIMITED_API), Py_3_4))]
impl Clone for PyObjectArenaAllocator {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(not(Py_LIMITED_API), Py_3_4))]
impl Default for PyObjectArenaAllocator {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(not(Py_LIMITED_API), Py_3_4))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyObject_GetArenaAllocator(allocator: *mut PyObjectArenaAllocator) -> ();
    pub fn PyObject_SetArenaAllocator(allocator: *mut PyObjectArenaAllocator) -> ();
}

/// Test if a type has a GC head
#[inline(always)]
pub unsafe fn PyType_IS_GC(t: *mut PyTypeObject) -> c_int {
    PyType_HasFeature(t, Py_TPFLAGS_HAVE_GC)
}

/// Test if an object has a GC head
#[inline(always)]
#[cfg(all(not(Py_LIMITED_API), not(Py_3_9)))]
pub unsafe fn PyObject_IS_GC(o: *mut PyObject) -> c_int {
    (PyType_IS_GC(Py_TYPE(o)) != 0
        && match (*Py_TYPE(o)).tp_is_gc {
            Some(tp_is_gc) => tp_is_gc(o) != 0,
            None => true,
        }) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg(all(not(Py_LIMITED_API), Py_3_9))]
    pub fn PyObject_IS_GC(o: *mut PyObject) -> c_int;

    pub fn _PyObject_GC_Resize(arg1: *mut PyVarObject, arg2: Py_ssize_t) -> *mut PyVarObject;

    #[cfg(not(Py_LIMITED_API))]
    pub fn _PyObject_GC_Malloc(size: size_t) -> *mut PyObject;
    #[cfg(all(not(Py_LIMITED_API), Py_3_5))]
    pub fn _PyObject_GC_Calloc(size: size_t) -> *mut PyObject;
    pub fn _PyObject_GC_New(arg1: *mut PyTypeObject) -> *mut PyObject;
    pub fn _PyObject_GC_NewVar(arg1: *mut PyTypeObject, arg2: Py_ssize_t) -> *mut PyVarObject;
    pub fn PyObject_GC_Track(arg1: *mut c_void) -> ();
    pub fn PyObject_GC_UnTrack(arg1: *mut c_void) -> ();
    pub fn PyObject_GC_Del(arg1: *mut c_void) -> ();

    #[cfg(Py_3_9)]
    pub fn PyObject_GC_IsTracked(o: *mut PyObject) -> c_int;
    #[cfg(Py_3_9)]
    pub fn PyObject_GC_IsFinalized(o: *mut PyObject) -> c_int;
}

/// Test if a type supports weak references
#[inline(always)]
#[cfg(not(Py_LIMITED_API))]
pub unsafe fn PyType_SUPPORTS_WEAKREFS(t: *mut PyTypeObject) -> c_int {
    ((*t).tp_weaklistoffset > 0) as c_int
}

#[inline(always)]
#[cfg(all(not(Py_LIMITED_API), not(Py_3_9)))]
pub unsafe fn PyObject_GET_WEAKREFS_LISTPTR(o: *mut PyObject) -> *mut *mut PyObject {
    let weaklistoffset = (*Py_TYPE(o)).tp_weaklistoffset as isize;
    (o as *mut u8).offset(weaklistoffset) as *mut *mut PyObject
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg(all(not(Py_LIMITED_API), Py_3_9))]
    pub fn PyObject_GET_WEAKREFS_LISTPTR(o: *mut PyObject) -> *mut *mut PyObject;
}

use core::ptr;
use libc::{c_char, c_int, c_void};

use crate::object::*;
use crate::pyport::Py_ssize_t;

#[inline]
pub unsafe fn PyObject_DelAttrString(o: *mut PyObject, attr_name: *const c_char) -> c_int {
    PyObject_SetAttrString(o, attr_name, ptr::null_mut())
}

#[inline]
pub unsafe fn PyObject_DelAttr(o: *mut PyObject, attr_name: *mut PyObject) -> c_int {
    PyObject_SetAttr(o, attr_name, ptr::null_mut())
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg(Py_3_9)]
    pub fn PyObject_CallNoArgs(func: *mut PyObject) -> *mut PyObject;

    pub fn PyObject_Call(
        callable: *mut PyObject,
        args: *mut PyObject,
        kwargs: *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyObject_CallObject(callable: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_CallFunction(
        callable: *mut PyObject,
        format: *const c_char,
        ...
    ) -> *mut PyObject;
    pub fn PyObject_CallMethod(
        obj: *mut PyObject,
        name: *const c_char,
        format: *const c_char,
        ...
    ) -> *mut PyObject;

    pub fn PyObject_CallFunctionObjArgs(callable: *mut PyObject, ...) -> *mut PyObject;
    pub fn PyObject_CallMethodObjArgs(
        obj: *mut PyObject,
        name: *mut PyObject,
        ...
    ) -> *mut PyObject;
    pub fn PyObject_Type(o: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_Size(o: *mut PyObject) -> Py_ssize_t;
}

#[inline]
pub unsafe fn PyObject_Length(o: *mut PyObject) -> Py_ssize_t {
    PyObject_Size(o)
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg(all(not(Py_LIMITED_API), Py_3_4))]
    pub fn PyObject_LengthHint(o: *mut PyObject, arg1: Py_ssize_t) -> Py_ssize_t;

    pub fn PyObject_GetItem(o: *mut PyObject, key: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_SetItem(o: *mut PyObject, key: *mut PyObject, v: *mut PyObject) -> c_int;
    pub fn PyObject_DelItemString(o: *mut PyObject, key: *const c_char) -> c_int;
    pub fn PyObject_DelItem(o: *mut PyObject, key: *mut PyObject) -> c_int;
    #[deprecated(since = "0.2.1", note = "Old Buffer API")]
    pub fn PyObject_AsCharBuffer(
        obj: *mut PyObject,
        buffer: *mut *const c_char,
        buffer_len: *mut Py_ssize_t,
    ) -> c_int;
    #[deprecated(since = "0.2.1", note = "Old Buffer API")]
    pub fn PyObject_CheckReadBuffer(obj: *mut PyObject) -> c_int;
    #[deprecated(since = "0.2.1", note = "Old Buffer API")]
    pub fn PyObject_AsReadBuffer(
        obj: *mut PyObject,
        buffer: *mut *const c_void,
        buffer_len: *mut Py_ssize_t,
    ) -> c_int;
    #[deprecated(since = "0.2.1", note = "Old Buffer API")]
    pub fn PyObject_AsWriteBuffer(
        obj: *mut PyObject,
        buffer: *mut *mut c_void,
        buffer_len: *mut Py_ssize_t,
    ) -> c_int;
}

// This is the old version of PyObject_CheckBuffer that was implemented as a macro.
// The new version is in pybuffer.rs.
#[cfg(all(not(Py_LIMITED_API), not(Py_3_9)))]
#[inline]
pub unsafe fn PyObject_CheckBuffer(o: *mut PyObject) -> c_int {
    let tp_as_buffer = (*(*o).ob_type).tp_as_buffer;
    (!tp_as_buffer.is_null() && (*tp_as_buffer).bf_getbuffer.is_some()) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyObject_Format(obj: *mut PyObject, format_spec: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_GetIter(arg1: *mut PyObject) -> *mut PyObject;
    #[cfg(Py_3_10)]
    pub fn PyObject_GetAIter(arg1: *mut PyObject) -> *mut PyObject;

    // Note: prior to 3.8, PyIter_Check was a macro instead
    #[cfg(Py_3_8)]
    pub fn PyIter_Check(o: *mut PyObject) -> c_int;

    #[cfg(Py_3_10)]
    pub fn PyAIter_Check(o: *mut PyObject) -> c_int;

    pub fn PyIter_Next(arg1: *mut PyObject) -> *mut PyObject;
    #[cfg(Py_3_10)]
    pub fn PyIter_Send(iter: *mut PyObject, arg: *mut PyObject, result: *mut *mut PyObject) -> PySendResult;
}

#[cfg(not(Py_3_8))]
#[inline]
pub unsafe fn PyIter_Check(o: *mut PyObject) -> c_int {
    (match (*(*o).ob_type).tp_iternext {
        Some(tp_iternext) => {
            tp_iternext as *const c_void
                != crate::object::_PyObject_NextNotImplemented as *const c_void
        }
        None => false,
    }) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyNumber_Check(o: *mut PyObject) -> c_int;
    pub fn PyNumber_Add(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Subtract(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Multiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg(Py_3_5)]
    pub fn PyNumber_MatrixMultiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_FloorDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_TrueDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Remainder(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Divmod(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Power(o1: *mut PyObject, o2: *mut PyObject, o3: *mut PyObject)
        -> *mut PyObject;
    pub fn PyNumber_Negative(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Positive(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Absolute(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Invert(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Lshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Rshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_And(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Xor(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Or(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
}

// Note: Py 3.8 has PyIndex_Check as a function, prior to that it was only availabe as a macro
#[cfg(all(not(Py_LIMITED_API), not(Py_3_8)))]
#[inline]
pub unsafe fn PyIndex_Check(o: *mut PyObject) -> c_int {
    let tp_as_number = (*(*o).ob_type).tp_as_number;
    (!tp_as_number.is_null() && (*tp_as_number).nb_index.is_some()) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg(Py_3_8)]
    pub fn PyIndex_Check(o: *mut PyObject) -> c_int;

    pub fn PyNumber_Index(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_AsSsize_t(o: *mut PyObject, exc: *mut PyObject) -> Py_ssize_t;
    pub fn PyNumber_Long(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_Float(o: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceAdd(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceSubtract(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceMultiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    #[cfg(Py_3_5)]
    pub fn PyNumber_InPlaceMatrixMultiply(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceFloorDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceTrueDivide(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceRemainder(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlacePower(
        o1: *mut PyObject,
        o2: *mut PyObject,
        o3: *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyNumber_InPlaceLshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceRshift(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceAnd(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceXor(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_InPlaceOr(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PyNumber_ToBase(n: *mut PyObject, base: c_int) -> *mut PyObject;

    pub fn PySequence_Check(o: *mut PyObject) -> c_int;
    pub fn PySequence_Size(o: *mut PyObject) -> Py_ssize_t;
}

#[inline]
pub unsafe fn PySequence_Length(o: *mut PyObject) -> Py_ssize_t {
    PySequence_Size(o)
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PySequence_Concat(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PySequence_Repeat(o: *mut PyObject, count: Py_ssize_t) -> *mut PyObject;
    pub fn PySequence_GetItem(o: *mut PyObject, i: Py_ssize_t) -> *mut PyObject;
    pub fn PySequence_GetSlice(o: *mut PyObject, i1: Py_ssize_t, i2: Py_ssize_t) -> *mut PyObject;
    pub fn PySequence_SetItem(o: *mut PyObject, i: Py_ssize_t, v: *mut PyObject) -> c_int;
    pub fn PySequence_DelItem(o: *mut PyObject, i: Py_ssize_t) -> c_int;
    pub fn PySequence_SetSlice(
        o: *mut PyObject,
        i1: Py_ssize_t,
        i2: Py_ssize_t,
        v: *mut PyObject,
    ) -> c_int;
    pub fn PySequence_DelSlice(o: *mut PyObject, i1: Py_ssize_t, i2: Py_ssize_t) -> c_int;
    pub fn PySequence_Tuple(o: *mut PyObject) -> *mut PyObject;
    pub fn PySequence_List(o: *mut PyObject) -> *mut PyObject;
    pub fn PySequence_Fast(o: *mut PyObject, m: *const c_char) -> *mut PyObject;
    // TODO: PySequence_Fast macros
    pub fn PySequence_Count(o: *mut PyObject, value: *mut PyObject) -> Py_ssize_t;
    pub fn PySequence_Contains(seq: *mut PyObject, ob: *mut PyObject) -> c_int;
}

#[inline]
pub unsafe fn PySequence_In(o: *mut PyObject, value: *mut PyObject) -> c_int {
    PySequence_Contains(o, value)
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PySequence_Index(o: *mut PyObject, value: *mut PyObject) -> Py_ssize_t;
    pub fn PySequence_InPlaceConcat(o1: *mut PyObject, o2: *mut PyObject) -> *mut PyObject;
    pub fn PySequence_InPlaceRepeat(o: *mut PyObject, count: Py_ssize_t) -> *mut PyObject;
    pub fn PyMapping_Check(o: *mut PyObject) -> c_int;
    pub fn PyMapping_Size(o: *mut PyObject) -> Py_ssize_t;
}

#[inline]
pub unsafe fn PyMapping_Length(o: *mut PyObject) -> Py_ssize_t {
    PyMapping_Size(o)
}

#[inline]
pub unsafe fn PyMapping_DelItemString(o: *mut PyObject, key: *mut c_char) -> c_int {
    PyObject_DelItemString(o, key)
}

#[inline]
pub unsafe fn PyMapping_DelItem(o: *mut PyObject, key: *mut PyObject) -> c_int {
    PyObject_DelItem(o, key)
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyMapping_HasKeyString(o: *mut PyObject, key: *const c_char) -> c_int;
    pub fn PyMapping_HasKey(o: *mut PyObject, key: *mut PyObject) -> c_int;
    pub fn PyMapping_Keys(o: *mut PyObject) -> *mut PyObject;
    pub fn PyMapping_Values(o: *mut PyObject) -> *mut PyObject;
    pub fn PyMapping_Items(o: *mut PyObject) -> *mut PyObject;
    pub fn PyMapping_GetItemString(o: *mut PyObject, key: *const c_char) -> *mut PyObject;
    pub fn PyMapping_SetItemString(
        o: *mut PyObject,
        key: *const c_char,
        value: *mut PyObject,
    ) -> c_int;
    pub fn PyObject_IsInstance(object: *mut PyObject, typeorclass: *mut PyObject) -> c_int;
    pub fn PyObject_IsSubclass(object: *mut PyObject, typeorclass: *mut PyObject) -> c_int;
}

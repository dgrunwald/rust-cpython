// Copyright (c) 2016 Daniel Grunwald
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this
// software and associated documentation files (the "Software"), to deal in the Software
// without restriction, including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons
// to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
// PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
// FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use libc::{c_char, c_int};
use std::ffi::CString;
use std::{isize, mem, ptr};

use crate::conversion::ToPyObject;
use crate::err::{PyErr, PyResult};
use crate::exc;
use crate::ffi;
use crate::function::CallbackConverter;
use crate::objects::PyObject;
use crate::py_class::CompareOp;
use crate::python::{Python, PythonObject};
use crate::Py_hash_t;

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_type_object_static_init {
    ($class_name:ident,
     $gc:tt,
    /* slots: */ {
        /* type_slots */  [ $( $slot_name:ident : $slot_value:expr, )* ]
        $as_number:tt
        $as_sequence:tt
        $as_mapping:tt
        $setdelitem:tt
    }) => (
        $crate::_detail::ffi::PyTypeObject {
            $( $slot_name : $slot_value, )*
            tp_dealloc: Some($crate::py_class::slots::tp_dealloc_callback::<$class_name>),
            tp_flags: $crate::py_class_type_object_flags!($gc),
            tp_traverse: $crate::py_class_tp_traverse!($class_name, $gc),
            ..
            $crate::_detail::ffi::PyTypeObject_INIT
        }
    );
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_type_object_flags {
    (/* gc: */ {
        /* traverse_proc: */ None,
        /* traverse_data: */ [ /*name*/ ]
    }) => {
        $crate::py_class::slots::TPFLAGS_DEFAULT
    };
    (/* gc: */ {
        $traverse_proc: expr,
        $traverse_data: tt
    }) => {
        $crate::py_class::slots::TPFLAGS_DEFAULT | $crate::_detail::ffi::Py_TPFLAGS_HAVE_GC
    };
}

#[cfg(feature = "python27-sys")]
pub const TPFLAGS_DEFAULT: ::libc::c_long = ffi::Py_TPFLAGS_DEFAULT | ffi::Py_TPFLAGS_CHECKTYPES;

#[cfg(feature = "python3-sys")]
pub const TPFLAGS_DEFAULT: ::libc::c_ulong = ffi::Py_TPFLAGS_DEFAULT;

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_type_object_dynamic_init {
    // initialize those fields of PyTypeObject that we couldn't initialize statically
    ($class: ident, $py:ident, $type_object:ident, $module_name: ident,
        /* slots: */ {
            $type_slots:tt
            $as_number:tt
            $as_sequence:tt
            $as_mapping:tt
            $setdelitem:tt
        }
        $props:tt
    ) => {
        unsafe {
            $type_object.init_ob_type(&mut $crate::_detail::ffi::PyType_Type);
            $type_object.tp_name =
                $crate::py_class::slots::build_tp_name($module_name, stringify!($class));
            $type_object.tp_basicsize = <$class as $crate::py_class::BaseObject>::size()
                as $crate::_detail::ffi::Py_ssize_t;
        }
        // call slot macros outside of unsafe block
        *(unsafe { &mut $type_object.tp_as_sequence }) =
            $crate::py_class_as_sequence!($as_sequence);
        *(unsafe { &mut $type_object.tp_as_number }) = $crate::py_class_as_number!($as_number);
        $crate::py_class_as_mapping!($type_object, $as_mapping, $setdelitem);
        *(unsafe { &mut $type_object.tp_getset }) = $crate::py_class_tp_getset!($class, $props);
    };
}

pub fn build_tp_name(module_name: Option<&str>, type_name: &str) -> *mut c_char {
    let name = match module_name {
        Some(module_name) => CString::new(format!("{}.{}", module_name, type_name)),
        None => CString::new(type_name),
    };
    name.expect("Module name/type name must not contain NUL byte")
        .into_raw()
}

pub unsafe extern "C" fn tp_dealloc_callback<T>(obj: *mut ffi::PyObject)
where
    T: super::BaseObject,
{
    let guard = crate::function::AbortOnDrop("Cannot unwind out of tp_dealloc");
    let py = Python::assume_gil_acquired();
    let r = T::dealloc(py, obj);
    mem::forget(guard);
    r
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_wrap_newfunc {
    ($class:ident :: $f:ident [ $( { $pname:ident : $ptype:ty = $detail:tt } )* ]) => {{
        unsafe extern "C" fn wrap_newfunc(
            cls: *mut $crate::_detail::ffi::PyTypeObject,
            args: *mut $crate::_detail::ffi::PyObject,
            kwargs: *mut $crate::_detail::ffi::PyObject)
        -> *mut $crate::_detail::ffi::PyObject
        {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "()");
            $crate::_detail::handle_callback(
                LOCATION, $crate::_detail::PyObjectCallbackConverter,
                |py| {
                    $crate::py_argparse_raw!(py, Some(LOCATION), args, kwargs,
                        [ $( { $pname : $ptype = $detail } )* ]
                        {
                            let cls = $crate::PyType::from_type_ptr(py, cls);
                            let ret = $class::$f(&cls, py $(, $pname )* );
                            $crate::PyDrop::release_ref(cls, py);
                            ret
                        })
                })
        }
        Some(wrap_newfunc)
    }}
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_as_sequence {
    ([]) => (0 as *mut $crate::_detail::ffi::PySequenceMethods);
    ([$( $slot_name:ident : $slot_value:expr ,)+]) => {{
        static mut SEQUENCE_METHODS : $crate::_detail::ffi::PySequenceMethods
            = $crate::_detail::ffi::PySequenceMethods {
                $( $slot_name : $slot_value, )*
                ..
                $crate::_detail::ffi::PySequenceMethods_INIT
            };
        unsafe { &mut SEQUENCE_METHODS }
    }}
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_as_number {
    ([]) => (0 as *mut $crate::_detail::ffi::PyNumberMethods);
    ([$( $slot_name:ident : $slot_value:expr ,)+]) => {{
        static mut NUMBER_METHODS : $crate::_detail::ffi::PyNumberMethods
            = $crate::_detail::ffi::PyNumberMethods {
                $( $slot_name : $slot_value, )*
                ..
                $crate::_detail::ffi::PyNumberMethods_INIT
            };
        unsafe { &mut NUMBER_METHODS }
    }}
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_as_mapping {
    ( $type_object:ident, [], [
        sdi_setitem: {},
        sdi_delitem: {},
    ]) => {};
    ( $type_object:ident, [ $( $slot_name:ident : $slot_value:expr ,)+ ], [
        sdi_setitem: {},
        sdi_delitem: {},
    ]) => {
        static mut MAPPING_METHODS : $crate::_detail::ffi::PyMappingMethods
            = $crate::_detail::ffi::PyMappingMethods {
                $( $slot_name : $slot_value, )*
                ..
                $crate::_detail::ffi::PyMappingMethods_INIT
            };
        unsafe { $type_object.tp_as_mapping = &mut MAPPING_METHODS; }
    };
    ( $type_object:ident, [ $( $slot_name:ident : $slot_value:expr ,)* ], [
        sdi_setitem: $setitem:tt,
        sdi_delitem: $delitem:tt,
    ]) => {{
        unsafe extern "C" fn mp_ass_subscript(
            slf: *mut $crate::_detail::ffi::PyObject,
            key: *mut $crate::_detail::ffi::PyObject,
            val: *mut $crate::_detail::ffi::PyObject
        ) -> $crate::_detail::libc::c_int {
            if val.is_null() {
                $crate::py_class_mp_ass_subscript!($delitem, slf,
                    b"Subscript assignment not supported by %.200s\0",
                    key)
            } else {
                $crate::py_class_mp_ass_subscript!($setitem, slf,
                    b"Subscript deletion not supported by %.200s\0",
                    key, val)
            }
        }
        static mut MAPPING_METHODS : $crate::_detail::ffi::PyMappingMethods
            = $crate::_detail::ffi::PyMappingMethods {
                $( $slot_name : $slot_value, )*
                mp_ass_subscript: Some(mp_ass_subscript),
                ..
                $crate::_detail::ffi::PyMappingMethods_INIT
            };
        unsafe { $type_object.tp_as_mapping = &mut MAPPING_METHODS; }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_mp_ass_subscript {
    ({}, $slf:ident, $error:expr, $( $arg:expr ),+) => {
        $crate::py_class::slots::mp_ass_subscript_error($slf, $error)
    };
    ({$slot:expr}, $slf:ident, $error:expr, $( $arg:expr ),+) => {
        $slot.unwrap()($slf, $( $arg ),+)
    }
}

pub unsafe fn mp_ass_subscript_error(o: *mut ffi::PyObject, err: &[u8]) -> c_int {
    ffi::PyErr_Format(
        ffi::PyExc_NotImplementedError,
        err.as_ptr() as *const c_char,
        (*ffi::Py_TYPE(o)).tp_name,
    );
    -1
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_call_slot_impl_with_ref {
    (
        $py:ident,
        $slf:ident,
        $f:ident,
        $arg:ident: [ Option<&$arg_type:ty> ],
        $arg_normal:expr,
        $arg_if_none:expr,
        $arg_if_some:expr
        $(, $extra_arg:ident)*
    ) => {{
        if $arg.is_none($py) {
            Ok($slf.$f($py, $arg_if_none $(, $extra_arg)*))
        } else {
            <$arg_type as $crate::RefFromPyObject>::with_extracted(
                $py,
                &$arg,
                |$arg: &$arg_type| $slf.$f($py, $arg_if_some $(, $extra_arg)*)
            )
        }
    }};

    (
        $py:ident,
        $slf:ident,
        $f:ident,
        $arg:ident: [ &$arg_type:ty ],
        $arg_normal:expr,
        $arg_if_none:expr,
        $arg_if_some:expr
        $(, $extra_arg:ident)*
    ) => {{
        <$arg_type as $crate::RefFromPyObject>::with_extracted(
            $py,
            &$arg,
            |$arg: &$arg_type| $slf.$f($py, $arg_normal $(, $extra_arg)*)
        )
    }};

    (
        $py:ident,
        $slf:ident,
        $f:ident,
        $arg:ident: [ $arg_type:ty ],
        $arg_normal:expr,
        $arg_if_none:expr,
        $arg_if_some:expr
        $(, $extra_arg:ident)*
    ) => {{
        <$arg_type as $crate::FromPyObject>::extract($py, &$arg)
            .map(|$arg| $slf.$f($py, $arg_normal $(, $extra_arg)*))
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_unary_slot {
    ($class:ident :: $f:ident, $res_type:ty, $conv:expr) => {{
        unsafe extern "C" fn wrap_unary(slf: *mut $crate::_detail::ffi::PyObject) -> $res_type {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "()");
            $crate::_detail::handle_callback(LOCATION, $conv, |py| {
                let slf =
                    $crate::PyObject::from_borrowed_ptr(py, slf).unchecked_cast_into::<$class>();
                let ret = slf.$f(py);
                $crate::PyDrop::release_ref(slf, py);
                ret
            })
        }
        Some(wrap_unary)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_binary_slot {
    ($class:ident :: $f:ident, $arg_type:tt, $res_type:ty, $conv:expr) => {{
        unsafe extern "C" fn wrap_binary(
            slf: *mut $crate::_detail::ffi::PyObject,
            arg: *mut $crate::_detail::ffi::PyObject,
        ) -> $res_type {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "()");
            $crate::_detail::handle_callback(LOCATION, $conv, |py| {
                let slf =
                    $crate::PyObject::from_borrowed_ptr(py, slf).unchecked_cast_into::<$class>();
                let arg = $crate::PyObject::from_borrowed_ptr(py, arg);
                let ret = match $crate::py_class_call_slot_impl_with_ref!(
                    py,
                    slf,
                    $f,
                    arg: $arg_type,
                    arg,
                    None,
                    Some(arg)
                ) {
                    Ok(r) => r,
                    Err(e) => Err(e),
                };
                $crate::PyDrop::release_ref(arg, py);
                $crate::PyDrop::release_ref(slf, py);
                ret
            })
        }
        Some(wrap_binary)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_ternary_slot {
    ($class:ident :: $f:ident, $arg1_type:tt, $arg2_type:ty, $res_type:ty, $conv:expr) => {{
        unsafe extern "C" fn wrap_binary(
            slf: *mut $crate::_detail::ffi::PyObject,
            arg1: *mut $crate::_detail::ffi::PyObject,
            arg2: *mut $crate::_detail::ffi::PyObject,
        ) -> $res_type {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "()");
            $crate::_detail::handle_callback(LOCATION, $conv, |py| {
                let slf =
                    $crate::PyObject::from_borrowed_ptr(py, slf).unchecked_cast_into::<$class>();
                let arg1 = $crate::PyObject::from_borrowed_ptr(py, arg1);
                let arg2 = $crate::PyObject::from_borrowed_ptr(py, arg2);
                let ret = match <$arg2_type as $crate::FromPyObject>::extract(py, &arg2) {
                    Ok(arg2) => {
                        match $crate::py_class_call_slot_impl_with_ref!(
                            py,
                            slf,
                            $f,
                            arg1: $arg1_type,
                            arg1,
                            None,
                            Some(arg1),
                            arg2
                        ) {
                            Ok(r) => r,
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                };
                $crate::PyDrop::release_ref(arg1, py);
                $crate::PyDrop::release_ref(arg2, py);
                $crate::PyDrop::release_ref(slf, py);
                ret
            })
        }
        Some(wrap_binary)
    }};
}

pub fn extract_op(py: Python, op: c_int) -> PyResult<CompareOp> {
    match op {
        ffi::Py_LT => Ok(CompareOp::Lt),
        ffi::Py_LE => Ok(CompareOp::Le),
        ffi::Py_EQ => Ok(CompareOp::Eq),
        ffi::Py_NE => Ok(CompareOp::Ne),
        ffi::Py_GT => Ok(CompareOp::Gt),
        ffi::Py_GE => Ok(CompareOp::Ge),
        _ => Err(PyErr::new_lazy_init(
            py.get_type::<exc::ValueError>(),
            Some(
                "tp_richcompare called with invalid comparison operator"
                    .to_py_object(py)
                    .into_object(),
            ),
        )),
    }
}

// sq_richcompare is special-cased slot
#[macro_export]
#[doc(hidden)]
macro_rules! py_class_richcompare_slot {
    ($class:ident :: $f:ident, $arg_type:tt, $res_type:ty, $conv:expr) => {{
        unsafe extern "C" fn tp_richcompare(
            slf: *mut $crate::_detail::ffi::PyObject,
            arg: *mut $crate::_detail::ffi::PyObject,
            op: $crate::_detail::libc::c_int,
        ) -> $res_type {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "()");
            $crate::_detail::handle_callback(LOCATION, $conv, |py| {
                let slf =
                    $crate::PyObject::from_borrowed_ptr(py, slf).unchecked_cast_into::<$class>();
                let arg = $crate::PyObject::from_borrowed_ptr(py, arg);
                let ret = match $crate::py_class::slots::extract_op(py, op) {
                    Ok(op) => {
                        match $crate::py_class_call_slot_impl_with_ref!(
                            py,
                            slf,
                            $f,
                            arg: $arg_type,
                            arg,
                            None,
                            Some(arg),
                            op
                        ) {
                            Ok(r) => r.map(|r| r.into_py_object(py).into_object()),
                            Err(e) => Ok(py.NotImplemented()),
                        }
                    }
                    Err(_) => Ok(py.NotImplemented()),
                };
                $crate::PyDrop::release_ref(arg, py);
                $crate::PyDrop::release_ref(slf, py);
                ret
            })
        }
        Some(tp_richcompare)
    }};
}

// sq_contains is special-cased slot because it converts type errors to Ok(false)
#[macro_export]
#[doc(hidden)]
macro_rules! py_class_contains_slot {
    ($class:ident :: $f:ident, $arg_type:tt) => {{
        unsafe extern "C" fn sq_contains(
            slf: *mut $crate::_detail::ffi::PyObject,
            arg: *mut $crate::_detail::ffi::PyObject,
        ) -> $crate::_detail::libc::c_int {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "()");
            $crate::_detail::handle_callback(
                LOCATION,
                $crate::py_class::slots::BoolConverter,
                |py| {
                    let slf = $crate::PyObject::from_borrowed_ptr(py, slf)
                        .unchecked_cast_into::<$class>();
                    let arg = $crate::PyObject::from_borrowed_ptr(py, arg);
                    let ret = match $crate::py_class_call_slot_impl_with_ref!(
                        py,
                        slf,
                        $f,
                        arg: $arg_type,
                        arg,
                        None,
                        Some(arg)
                    ) {
                        Ok(r) => r,
                        Err(e) => $crate::py_class::slots::type_error_to_false(py, e),
                    };
                    $crate::PyDrop::release_ref(arg, py);
                    $crate::PyDrop::release_ref(slf, py);
                    ret
                },
            )
        }
        Some(sq_contains)
    }};
}

pub fn type_error_to_false(py: Python, e: PyErr) -> PyResult<bool> {
    if e.matches(py, py.get_type::<exc::TypeError>()) {
        Ok(false)
    } else {
        Err(e)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_numeric_slot {
    (binary $class:ident :: $f:ident) => {{
        unsafe extern "C" fn binary_numeric(
            lhs: *mut $crate::_detail::ffi::PyObject,
            rhs: *mut $crate::_detail::ffi::PyObject,
        ) -> *mut $crate::_detail::ffi::PyObject {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "()");
            $crate::_detail::handle_callback(
                LOCATION,
                $crate::_detail::PyObjectCallbackConverter,
                |py| {
                    let lhs = $crate::PyObject::from_borrowed_ptr(py, lhs);
                    let rhs = $crate::PyObject::from_borrowed_ptr(py, rhs);
                    let ret = $class::$f(py, &lhs, &rhs);
                    $crate::PyDrop::release_ref(rhs, py);
                    $crate::PyDrop::release_ref(lhs, py);
                    ret
                },
            )
        }
        Some(binary_numeric)
    }};
    (ternary $class:ident :: $f:ident) => {{
        unsafe extern "C" fn ternary_numeric(
            lhs: *mut $crate::_detail::ffi::PyObject,
            rhs: *mut $crate::_detail::ffi::PyObject,
            ex: *mut $crate::_detail::ffi::PyObject,
        ) -> *mut $crate::_detail::ffi::PyObject {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "()");
            $crate::_detail::handle_callback(
                LOCATION,
                $crate::_detail::PyObjectCallbackConverter,
                |py| {
                    let lhs = $crate::PyObject::from_borrowed_ptr(py, lhs);
                    let rhs = $crate::PyObject::from_borrowed_ptr(py, rhs);
                    let ex = $crate::PyObject::from_borrowed_ptr(py, ex);
                    let ret = $class::$f(py, &lhs, &rhs, &ex);
                    $crate::PyDrop::release_ref(ex, py);
                    $crate::PyDrop::release_ref(rhs, py);
                    $crate::PyDrop::release_ref(lhs, py);
                    ret
                },
            )
        }
        Some(ternary_numeric)
    }};
}

pub struct UnitCallbackConverter;

impl CallbackConverter<()> for UnitCallbackConverter {
    type R = c_int;

    #[inline]
    fn convert(_: (), _: Python) -> c_int {
        0
    }

    #[inline]
    fn error_value() -> c_int {
        -1
    }
}

pub struct LenResultConverter;

impl CallbackConverter<usize> for LenResultConverter {
    type R = isize;

    fn convert(val: usize, py: Python) -> isize {
        if val <= (isize::MAX as usize) {
            val as isize
        } else {
            PyErr::new_lazy_init(py.get_type::<exc::OverflowError>(), None).restore(py);
            -1
        }
    }

    #[inline]
    fn error_value() -> isize {
        -1
    }
}

pub struct IterNextResultConverter;

impl<T> CallbackConverter<Option<T>> for IterNextResultConverter
where
    T: ToPyObject,
{
    type R = *mut ffi::PyObject;

    fn convert(val: Option<T>, py: Python) -> *mut ffi::PyObject {
        match val {
            Some(val) => val.into_py_object(py).into_object().steal_ptr(),
            None => unsafe {
                ffi::PyErr_SetNone(ffi::PyExc_StopIteration);
                ptr::null_mut()
            },
        }
    }

    #[inline]
    fn error_value() -> *mut ffi::PyObject {
        ptr::null_mut()
    }
}

pub trait WrappingCastTo<T> {
    fn wrapping_cast(self) -> T;
}

macro_rules! wrapping_cast {
    ($from:ty, $to:ty) => {
        impl WrappingCastTo<$to> for $from {
            #[inline]
            fn wrapping_cast(self) -> $to {
                self as $to
            }
        }
    };
}
wrapping_cast!(u8, Py_hash_t);
wrapping_cast!(u16, Py_hash_t);
wrapping_cast!(u32, Py_hash_t);
wrapping_cast!(usize, Py_hash_t);
wrapping_cast!(u64, Py_hash_t);
wrapping_cast!(i8, Py_hash_t);
wrapping_cast!(i16, Py_hash_t);
wrapping_cast!(i32, Py_hash_t);
wrapping_cast!(isize, Py_hash_t);
wrapping_cast!(i64, Py_hash_t);

pub struct HashConverter;

impl<T> CallbackConverter<T> for HashConverter
where
    T: WrappingCastTo<Py_hash_t>,
{
    type R = Py_hash_t;

    #[inline]
    fn convert(val: T, _py: Python) -> Py_hash_t {
        let hash = val.wrapping_cast();
        if hash == -1 {
            -2
        } else {
            hash
        }
    }

    #[inline]
    fn error_value() -> Py_hash_t {
        -1
    }
}

pub struct BoolConverter;

impl CallbackConverter<bool> for BoolConverter {
    type R = c_int;

    #[inline]
    fn convert(val: bool, _py: Python) -> c_int {
        val as c_int
    }

    #[inline]
    fn error_value() -> c_int {
        -1
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_call_slot {
    ($class:ident :: $f:ident [ $( { $pname:ident : $ptype:ty = $detail:tt } )* ]) => {{
        unsafe extern "C" fn wrap_call(
            slf: *mut $crate::_detail::ffi::PyObject,
            args: *mut $crate::_detail::ffi::PyObject,
            kwargs: *mut $crate::_detail::ffi::PyObject)
        -> *mut $crate::_detail::ffi::PyObject
        {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "()");
            $crate::_detail::handle_callback(
                LOCATION, $crate::_detail::PyObjectCallbackConverter,
                |py| {
                    $crate::py_argparse_raw!(py, Some(LOCATION), args, kwargs,
                        [ $( { $pname : $ptype = $detail } )* ]
                        {
                            let slf = $crate::PyObject::from_borrowed_ptr(py, slf).unchecked_cast_into::<$class>();
                            let ret = slf.$f(py $(, $pname )* );
                            $crate::PyDrop::release_ref(slf, py);
                            ret
                        })
                })
        }
        Some(wrap_call)
    }}
}

/// Used as implementation in the `sq_item` slot to forward calls to the `mp_subscript` slot.
pub unsafe extern "C" fn sq_item(
    obj: *mut ffi::PyObject,
    index: ffi::Py_ssize_t,
) -> *mut ffi::PyObject {
    let arg = ffi::PyLong_FromSsize_t(index);
    if arg.is_null() {
        return arg;
    }
    let ret = ffi::PyObject_GetItem(obj, arg);
    ffi::Py_DECREF(arg);
    ret
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_prop_getter {
    ($class:ident :: $f:ident) => {{
        unsafe extern "C" fn wrap_getter(
            slf: *mut $crate::_detail::ffi::PyObject,
            _closure: *mut $crate::_detail::libc::c_void,
        ) -> *mut $crate::_detail::ffi::PyObject {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "{}");
            $crate::_detail::handle_callback(
                LOCATION,
                $crate::_detail::PyObjectCallbackConverter,
                |py| {
                    let slf = $crate::PyObject::from_borrowed_ptr(py, slf)
                        .unchecked_cast_into::<$class>();
                    let ret = slf.$f(py);
                    $crate::PyDrop::release_ref(slf, py);
                    ret
                },
            )
        }
        Some(wrap_getter)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_prop_setter {
    ($class:ident :: $f:ident, $value_type:tt) => {{
        unsafe extern "C" fn wrap_setter(
            slf: *mut $crate::_detail::ffi::PyObject,
            obj: *mut $crate::_detail::ffi::PyObject,
            _closure: *mut $crate::_detail::libc::c_void,
        ) -> $crate::_detail::libc::c_int {
            const LOCATION: &'static str = concat!(stringify!($class), ".", stringify!($f), "{}");
            $crate::_detail::handle_callback(
                LOCATION,
                $crate::py_class::slots::UnitCallbackConverter,
                |py| {
                    let slf = $crate::PyObject::from_borrowed_ptr(py, slf)
                        .unchecked_cast_into::<$class>();
                    let ret = if obj.is_null() {
                        slf.$f(py, None)
                    } else {
                        let obj = $crate::PyObject::from_borrowed_ptr(py, obj);
                        let ret = match $crate::py_class_call_slot_impl_with_ref!(
                            py,
                            slf,
                            $f,
                            obj: $value_type,
                            Some(obj),
                            Some(None),
                            Some(Some(obj))
                        ) {
                            Ok(r) => r,
                            Err(e) => Err(e),
                        };
                        $crate::PyDrop::release_ref(obj, py);
                        ret
                    };
                    $crate::PyDrop::release_ref(slf, py);
                    ret
                },
            )
        }
        Some(wrap_setter)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_tp_getset {
    ( $class:ident, { [] [] } ) => { 0 as *mut $crate::_detail::ffi::PyGetSetDef };
    (
        $class:ident,
        {
            [ $( { $doc:expr } $getter_name:ident: $prop_type:ty, )* ]
            [ $( $setter_name:ident: $value_type:tt => $setter_setter:ident, )* ]
        }
    ) => {{
        let mut index = 0usize;
        $( let $getter_name = index; index += 1; )*
        unsafe {
            static mut GETSET: &mut [$crate::_detail::ffi::PyGetSetDef] = &mut [
                $($crate::_detail::ffi::PyGetSetDef {
                    name: 0 as *mut _,
                    get: $crate::py_class_prop_getter!($class::$getter_name),
                    set: None,
                    doc: 0 as *mut _,
                    closure: 0 as *mut _,
                },)*
                $crate::_detail::ffi::PyGetSetDef {
                    name: 0 as *mut _,
                    get: None,
                    set: None,
                    doc: 0 as *mut _,
                    closure: 0 as *mut _,
                }
            ];
            $(
                GETSET[$getter_name].name = $crate::strip_raw!(
                    concat!(stringify!($getter_name), "\0")
                ).as_ptr() as *mut _;
                if !$doc.is_empty() {
                    GETSET[$getter_name].doc = concat!($doc, "\0").as_ptr() as *mut _;
                }
            )*
            $(
                GETSET[$setter_name].set = $crate::py_class_prop_setter!($class::$setter_setter, $value_type);
            )*
            GETSET.as_ptr() as *mut _
        }
    }};
}

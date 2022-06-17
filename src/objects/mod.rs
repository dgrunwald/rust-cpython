// Copyright (c) 2015 Daniel Grunwald
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

#![allow(clippy::transmute_ptr_to_ptr)]

pub use self::module::PyModule;
pub use self::object::PyObject;
pub use self::typeobject::PyType;

#[cfg(feature = "python3-sys")]
pub use self::string::PyString as PyUnicode;
#[cfg(feature = "python27-sys")]
pub use self::string::PyUnicode;
pub use self::string::{PyBytes, PyString, PyStringData};

pub use self::boolobject::PyBool;
pub use self::capsule::PyCapsule;
pub use self::dict::PyDict;
pub use self::iterator::PyIterator;
pub use self::list::PyList;
pub use self::none::PyNone;
#[cfg(feature = "python27-sys")]
pub use self::num::PyInt;
#[cfg(feature = "python3-sys")]
pub use self::num::PyLong as PyInt;
pub use self::num::{PyFloat, PyLong};
pub use self::sequence::PySequence;
pub use self::set::PySet;
pub use self::tuple::{NoArgs, PyTuple};

#[macro_export]
macro_rules! pyobject_newtype(
    ($name: ident) => (
        $crate::py_impl_to_py_object_for_python_object!($name);
        $crate::py_impl_from_py_object_for_python_object!($name);

        impl $crate::PythonObject for $name {
            #[inline]
            fn as_object(&self) -> &$crate::PyObject {
                &self.0
            }

            #[inline]
            fn into_object(self) -> $crate::PyObject {
                self.0
            }

            /// Unchecked downcast from PyObject to Self.
            /// Undefined behavior if the input object does not have the expected type.
            #[inline]
            unsafe fn unchecked_downcast_from(obj: $crate::PyObject) -> Self {
                $name(obj)
            }

            /// Unchecked downcast from PyObject to Self.
            /// Undefined behavior if the input object does not have the expected type.
            #[inline]
            unsafe fn unchecked_downcast_borrow_from<'a>(obj: &'a $crate::PyObject) -> &'a Self {
                std::mem::transmute(obj)
            }
        }
    );
    ($name: ident, $checkfunction: ident) => (
        pyobject_newtype!($name);

        impl crate::python::PythonObjectWithCheckedDowncast for $name {
            #[inline]
            fn downcast_from<'p>(py: crate::python::Python<'p>, obj: crate::objects::object::PyObject) -> $crate::_detail::Result<$name, crate::python::PythonObjectDowncastError<'p>> {
                unsafe {
                    if crate::ffi::$checkfunction(obj.as_ptr()) != 0 {
                        Ok($name(obj))
                    } else {
                        Err(crate::python::PythonObjectDowncastError::new(
                            py,
                            stringify!($name),
                            obj.get_type(py)
                        ))
                    }
                }
            }

            #[inline]
            fn downcast_borrow_from<'a, 'p>(py: crate::python::Python<'p>, obj: &'a crate::objects::object::PyObject) -> $crate::_detail::Result<&'a $name, crate::python::PythonObjectDowncastError<'p>> {
                unsafe {
                    if crate::ffi::$checkfunction(obj.as_ptr()) != 0 {
                        Ok(std::mem::transmute(obj))
                    } else {
                        Err(crate::python::PythonObjectDowncastError::new(
                            py,
                            stringify!($name),
                            obj.get_type(py)
                        ))
                    }
                }
            }
        }
    );
    ($name: ident, $checkfunction: ident, $typeobject: ident) => (
        pyobject_newtype!($name, $checkfunction);

        impl crate::python::PythonObjectWithTypeObject for $name {
            #[inline]
            fn type_object(py: crate::python::Python) -> crate::objects::typeobject::PyType {
                unsafe { crate::objects::typeobject::PyType::from_type_ptr(py, &mut crate::ffi::$typeobject) }
            }
        }
    );
);

macro_rules! extract(
    ($obj:ident to $t:ty; $(#[$meta:meta])* $py:ident => $body: block) => {
        impl <'s> crate::conversion::FromPyObject<'s>
            for $t
        {
            $(#[$meta])*
            fn extract($py: Python, $obj: &'s PyObject) -> PyResult<Self> {
                $body
            }
        }
    }
);

mod boolobject;
mod capsule;
mod dict;
pub mod exc;
mod iterator;
mod list;
mod module;
mod none;
mod num;
mod object;
mod sequence;
mod set;
mod string;
mod tuple;
mod typeobject;

#[cfg(feature = "python27-sys")]
pub mod oldstyle;

mod tests;

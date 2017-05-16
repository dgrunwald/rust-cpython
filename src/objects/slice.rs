use ffi;
use python::{Python, PythonObject};
use err::{self, PyResult, PyErr};
use super::{exc, PyObject};
use conversion::{FromPyObject, ToPyObject};
use std::ops;

/// Represents a Python slice object.
pub struct PySlice(PyObject);

pyobject_newtype!(PySlice, PySlice_Check, PySlice_Type);

impl PySlice {
    /// Construct a new PySlice with given start, stop, and step.
    #[inline]
    pub fn new<T, U, V>(py: Python, start: T, stop: U, step: V) -> Self
        where T: ToPyObject,
              U: ToPyObject,
              V: ToPyObject
    {
        start.with_borrowed_ptr(py, |start|
            stop.with_borrowed_ptr(py, |stop|
                step.with_borrowed_ptr(py, |step| unsafe {
                    err::cast_from_owned_ptr_or_panic(
                        py, ffi::PySlice_New(start, stop, step))
                })
            )
        )
    }

    /// Accessor method for the start of the PySlice.
    #[inline]
    pub fn start(&self) -> &PyObject {
        unsafe {
            let ptr = self.0.as_ptr() as *mut ffi::PySliceObject;
            PyObject::borrow_from_ptr(&(*ptr).start)
        }
    }

    /// Accessor method for the stop of the PySlice.
    #[inline]
    pub fn stop(&self) -> &PyObject {
        unsafe {
            let ptr = self.0.as_ptr() as *mut ffi::PySliceObject;
            PyObject::borrow_from_ptr(&(*ptr).stop)
        }
    }

    /// Accessor method for the step of the PySlice.
    #[inline]
    pub fn step(&self) -> &PyObject {
        unsafe {
            let ptr = self.0.as_ptr() as *mut ffi::PySliceObject;
            PyObject::borrow_from_ptr(&(*ptr).step)
        }
    }
}

/// Converts a rust Range to a Python slice.
impl<T> ToPyObject for ops::Range<T> where T: ToPyObject {
    type ObjectType = PySlice;

    #[inline]
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        Self::ObjectType::new(py, &self.start, &self.end, py.None())
    }
}

/// Converts a rust RangeFrom to a Python slice.
impl<T> ToPyObject for ops::RangeFrom<T> where T: ToPyObject {
    type ObjectType = PySlice;

    #[inline]
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        Self::ObjectType::new(py, &self.start, py.None(), py.None())
    }
}

/// Converts a rust RangeFull to a Python slice.
impl ToPyObject for ops::RangeFull {
    type ObjectType = PySlice;

    #[inline]
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        Self::ObjectType::new(py, py.None(), py.None(), py.None())
    }
}

/// Converts a rust RangeTo to a Python slice.
impl<T> ToPyObject for ops::RangeTo<T> where T: ToPyObject {
    type ObjectType = PySlice;

    #[inline]
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        Self::ObjectType::new(py, py.None(), &self.end, py.None())
    }
}

macro_rules! range_from_slice_body {
    ($Range:ident; $($none_part:ident),*; $($rust_part:ident $py_part:ident),*) => {
        #[inline]
        fn extract(py: Python, obj: &'a PyObject) -> PyResult<Self> {
            let obj = try!(obj.cast_as::<PySlice>(py));
            $(
                if obj.$none_part().as_ptr() != unsafe { ffi::Py_None() } {
                    const MSG: &'static str =
                        concat!("cannot cast slice with ", stringify!($none_part),
                            " as ::std::ops::", stringify!($Range));
                    return Err(PyErr::new_lazy_init(
                        py.get_type::<exc::ValueError>(),
                        Some(MSG.to_py_object(py).into_object())
                    ));
                }
            )*

            Ok(ops::$Range {
                $(
                    $rust_part: try!(T::extract(py, obj.$py_part())),
                )*
            })
        }
    }
}
macro_rules! range_from_slice {
    ($Range:ident<T>; $($none_part:ident),*; $($rust_part:ident $py_part:ident),*) => {
        impl<'a, T> FromPyObject<'a> for ops::$Range<T> where T: for<'b> FromPyObject<'b> {
            range_from_slice_body!($Range; $($none_part),*; $($rust_part $py_part),*);
        }
    };
    ($Range:ident; $($none_part:ident),*; ) => {
        impl<'a> FromPyObject<'a> for ops::$Range {
            range_from_slice_body!($Range; $($none_part),*; );
        }
    };
}

/// Converts a Python slice to rust Range types.
range_from_slice!(Range<T>; step; start start, end stop);
range_from_slice!(RangeFrom<T>; stop, step; start start);
range_from_slice!(RangeFull; start, stop, step; );
range_from_slice!(RangeTo<T>; start, step; end stop);

#[cfg(test)]
mod test {
    use python::{Python, PythonObject};
    use conversion::ToPyObject;
    use super::PySlice;
    use std::ops;

    #[test]
    fn test_range_slice_conversion() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        assert_eq!(2..9, (2..9).to_py_object(py).as_object().extract(py).unwrap());
        assert_eq!(2.., (2..).to_py_object(py).as_object().extract(py).unwrap());
        assert_eq!(.., (..).to_py_object(py).as_object().extract(py).unwrap());
        assert_eq!(..9, (..9).to_py_object(py).as_object().extract(py).unwrap());

        let x: ops::Range<String> = ("a".."z").to_py_object(py).as_object().extract(py).unwrap();
        assert_eq!(x, "a".to_owned().."z".to_owned());
    }
}

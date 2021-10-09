use std::cmp::Ordering;
use std::fmt;

use crate::conversion::ToPyObject;
use crate::err::{self, PyErr, PyResult};
use crate::ffi;
use crate::objects::{PyObject, PyInt, PyLong, PyFloat};
use crate::python::{Python, PythonObject, ToPythonPointer};


use super::ObjectProtocol;

/// Operations on numeric objects
pub trait NumberProtocol: ObjectProtocol {
    /// Perform addition (self + other)
    ///
    /// Invokes the `__add__` magic-method
    #[inline]
    fn add(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Add(self.as_ptr(), other))
        })
    }
    /// Perform subtraction (self - other)
    ///
    /// Invokes the `__sub__` magic-method
    #[inline]
    fn subtract(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Subtract(self.as_ptr(), other))
        })
    }
    /// Perform multiplication (self * other)
    ///
    /// Invokes the `__mul__` magic-method
    #[inline]
    fn multiply(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Multiply(self.as_ptr(), other))
        })
    }
    /// Perform matrix multiplication, equivalent to the Python expression `self @ other`
    ///
    /// Invokes the `__matmul__` magic-method
    ///
    /// This method is only available with Python 3.
    ///
    /// See [PEP 0456](https://www.python.org/dev/peps/pep-0465/) for details.
    #[inline]
    #[cfg(feature = "python3-sys")]
    fn matrix_multiply(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_MatrixMultiply(self.as_ptr(), other))
        })
    }
    /// Perform exponentiation, equivalent to the Python expression `self ** other`,
    /// or the two-argument form of the builtin method pow: `pow(self, other)`
    ///
    /// Invokes the `__pow__` magic-method
    ///
    /// See also [NumberProtocol::power_modulo].
    #[inline]
    fn power(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        self.power_modulo(py, other, py.None())
    }   
    /// Perform exponentiation modulo an integer,
    /// mathematically equivalent to `self ** other % mod`
    /// but computed much more efficiently.
    /// 
    /// Equivalent to invoking the three-argument form
    /// of the builtin `pow` method: `pow(self, other, z)`
    ///
    /// Invoking with a `None` for modulo is equivalent to
    /// the regular power operation.
    ///
    /// Invokes the `__pow__` magic-method
    #[inline]
    fn power_modulo(&self, py: Python, exp: impl ToPyObject, z: impl ToPyObject) -> PyResult<PyObject> {
        exp.with_borrowed_ptr(py, |exp| {
            z.with_borrowed_ptr(py, |z| unsafe {
                err::result_from_owned_ptr(py, ffi::PyNumber_Power(self.as_ptr(), exp, z))
            })
        })
    }
    /// Perform the "true division" operation,
    /// equivalent to the Python expression `self / other`,
    /// 
    /// Invokes the `__truediv__` magic-method.
    #[inline]   
    fn true_divide(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_TrueDivide(self.as_ptr(), other))
        })
    }
    /// Perform the "floor division" operation,
    /// equivalent to the Python expression `self // other`,
    /// 
    /// This method was added in Python 3.
    /// If compiling against Python 2, it unconditional throws an error.
    ///
    /// Invokes the `__floordiv__` magic-method.
    #[inline]
    fn floor_divide(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_FloorDivide(self.as_ptr(), other))
        })
    }
    /// Return the remainder of dividing `self` by `other`,
    /// equivalent to the Python expression `self % other`
    ///
    /// Invokes the `__mod__` magic-method.
    #[inline]
    fn modulo(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> { 
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Remainder(self.as_ptr(), other))
        })
    }
    /// Perform combined division and modulo,
    /// equivalent to the builtin method `divmod(self, other)`
    ///
    /// Invokes the `__divmod__` magic-method.
    #[inline]
    fn div_mod(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Divmod(self.as_ptr(), other))
        })
    }
    /// Perform the negation of self (-self)
    ///
    /// Invokes the `__neg__` magic-method.
    #[inline]
    fn negative(&self, py: Python) -> PyResult<PyObject> {
        unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Negative(self.as_ptr()))
        }
    }
    /// Invoke the 'positive' operation, equivalent to the
    /// Python expression `+self`
    ///
    /// Invokes the `__pos__` magic-method
    #[inline]
    fn positive(&self, py: Python) -> PyResult<PyObject> {       
        unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Positive(self.as_ptr()))
        }
    }
    /// Return the absolute value of self,
    /// equivalent to calling the builtin function `abs`
    ///
    /// Invokes the `__abs__` magic-method.
    #[inline]
    fn absolute(&self, py: Python) -> PyResult<PyObject> {
        unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Absolute(self.as_ptr()))
        }
    }
    /// Perform the bitwise negation of self,
    /// equivalent to the Python expression `~self`
    ///
    /// Invokes the `__invert__` magic-method
    #[inline]
    fn bitwise_invert(&self, py: Python) -> PyResult<PyObject> {
        unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Invert(self.as_ptr()))
        }
    }
    /// Shift this value to the left by the specified number of bits,
    /// equivalent to the Python expression `self << bits`
    ///
    /// Invokes the `__lshift__` magic-method
    #[inline]
    fn left_shift(&self, py: Python, bits: impl ToPyObject) -> PyResult<PyObject> {
        bits.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Lshift(self.as_ptr(), other))
        })
    }
    /// Shift this value to the right by the specified number of bits,
    /// equivalent to the Python expression `self >> bits`
    ///
    /// Invokes the `__rshift__` magic-method
    #[inline]
    fn right_shift(&self, py: Python, bits: impl ToPyObject) -> PyResult<PyObject> {
        bits.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Rshift(self.as_ptr(), other))
        })
    }
    /// Perform the "bitwise and" of `self & other`
    ///
    /// Invokes the `__and__` magic-method.
    #[inline]
    fn bitwise_and(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_And(self.as_ptr(), other))
        })
    }
    /// Perform the "bitwise exclusive or",
    /// equivalent to Python expression `self ^ other`
    ///
    /// Invokes the `__xor__` magic-method.
    #[inline]
    fn bitwise_xor(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Xor(self.as_ptr(), other))
        })
    }
    /// Perform the "bitwise or" of `self | other`
    ///
    /// Invokes the `__or__` magic-method.
    #[inline]
    fn bitwise_or(&self, py: Python, other: impl ToPyObject) -> PyResult<PyObject> {
        other.with_borrowed_ptr(py, |other| unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Or(self.as_ptr(), other))
        })
    }
    /// Convert this object to an integer,
    /// equivalent to the builtin function `int(self)`
    ///
    /// Invokes the `__int__` magic-method.
    ///
    /// Throws an exception if unable to perform
    /// the conversion.
    #[inline]
    fn to_int(&self, py: Python) -> PyResult<PyLong> {
        let obj = unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Long(self.as_ptr()))?
        };
        Ok(obj.cast_into::<PyLong>(py)?)
    }
    /// Convert this object to a float,
    /// equivalent to the builtin function `float(self)`
    ///
    /// Invokes the `__float__` magic-method.
    ///
    /// Throws an exception if unable to perform
    /// the conversion.
    #[inline]
    fn to_float(&self, py: Python) -> PyResult<PyFloat> {
        let obj = unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Float(self.as_ptr()))?
        };
        Ok(obj.cast_into::<PyFloat>(py)?)
    }
    /// Losslessly convert this object to an integer index,
    /// as if calling `operator.index()`
    ///
    /// The presence of this method indicates 
    /// this object is an integer type.
    ///
    /// Calls the `__index__` magic-method.
    ///
    /// See also: [Documentation on the corresponding magic-method](https://docs.python.org/3/reference/datamodel.html?highlight=__index__#object.__index__)
    #[inline]
    fn to_index(&self, py: Python) -> PyResult<PyLong> {
        let obj = unsafe {
            err::result_from_owned_ptr(py, ffi::PyNumber_Index(self.as_ptr()))?
        };
        Ok(obj.cast_into::<PyLong>(py)?)
    }
}

impl NumberProtocol for PyObject {}


#[cfg(test)]
mod test {
    use crate::*;
    use super::*;

    #[test]
    fn addition() {
        let guard = Python::acquire_gil();
        let py = guard.python();
        let i1 = (5i32).to_py_object(py).into_object();
        let i2 = (12i32).to_py_object(py).into_object();
        let actual_res = i1.add(py, i2).unwrap();
        let expected_res = (17i32).to_py_object(py).into_object();
        assert_eq!(
            actual_res.compare(py, expected_res).unwrap(),
            Ordering::Equal
        );
    }
}

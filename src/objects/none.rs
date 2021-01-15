// Copyright (c) 2021 Mark Juggurnauth-Thomas
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

use super::exc;
use super::object::PyObject;
use crate::conversion::{FromPyObject, ToPyObject};
use crate::err::{PyErr, PyResult};
use crate::python::{Python, PythonObject};

/// An empty struct that represents `None` in Python.
///
/// This can be used as a function return type for functions that should return
/// `None` in Python.
///
/// # Example
/// ```
/// use cpython::{Python, PyResult, PyNone};
///
/// fn example(py: Python) -> PyResult<PyNone> {
///    Ok(PyNone)
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Default, Hash, Ord)]
pub struct PyNone;

impl ToPyObject for PyNone {
    type ObjectType = PyObject;

    #[inline]
    fn to_py_object(&self, py: Python) -> PyObject {
        py.None()
    }
}

impl FromPyObject<'_> for PyNone {
    fn extract(py: Python, obj: &PyObject) -> PyResult<Self> {
        if *obj == py.None() {
            Ok(PyNone)
        } else {
            let msg = format!("Expected None but received {}", obj.get_type(py).name(py));
            Err(PyErr::new_lazy_init(
                py.get_type::<exc::TypeError>(),
                Some(msg.to_py_object(py).into_object()),
            ))
        }
    }
}

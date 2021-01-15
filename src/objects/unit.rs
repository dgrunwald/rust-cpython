use super::PyObject;
use crate::conversion::ToPyObject;
use crate::err::PyResult;
use crate::ffi;
use crate::python::Python;

impl ToPyObject for () {
    type ObjectType = PyObject;

    #[inline]
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        py.None()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Python, ToPyObject};

    #[test]
    fn test_unit_to_python() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let py_none = ().to_py_object(py);

        assert!(py_none == py.None());
    }
}

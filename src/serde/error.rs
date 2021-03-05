use crate::exc;
use crate::ObjectProtocol;
use crate::PyErr;
use crate::Python;
use serde::{de, ser};
use std::fmt;

/// Error type used by serialization.
pub struct Error(PyErr);

impl Error {
    /// Construct a `ValueError` from a message.
    pub(crate) fn value_error(py: Python, msg: impl ToString) -> Self {
        PyErr::new::<exc::ValueError, _>(py, msg.to_string()).into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let repr = self
            .0
            .pvalue
            .as_ref()
            .unwrap_or(&self.0.ptype)
            .repr(py)
            .map(|s| s.to_string_lossy(py).to_string())
            .unwrap_or_else(|_| "<error in repr>".into());
        write!(f, "{}", repr)?;
        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<PyErr> for Error {
    fn from(err: PyErr) -> Self {
        Self(err)
    }
}

impl From<Error> for PyErr {
    fn from(error: Error) -> Self {
        error.0
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Self::value_error(py, msg)
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Self::value_error(py, msg)
    }
}

//! `serde` integration.
//! - `to_py_object` converts a Rust value to a Python object.
//! - `from_py_object` converts a Python object back to a Rust value.
//!
//! Requires the `serde-convert` feature.

mod de;
mod error;
mod ser;

#[cfg(test)]
mod tests;

pub use de::from_py_object;
pub use error::Error;
pub use ser::to_py_object;

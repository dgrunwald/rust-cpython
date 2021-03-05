#![no_std]
#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    unused_parens,
    clippy::missing_safety_doc,
    clippy::transmute_ptr_to_ptr,
    clippy::unused_unit,
    clippy::identity_op
)]

// Macro for marking parts of the Python headers as ignored.
macro_rules! ignore {
    ( $( $_tokens:tt )* ) => {};
}

pub use crate::boolobject::*;
pub use crate::bufferobject::*;
pub use crate::bytearrayobject::*;
pub use crate::bytesobject::*;
pub use crate::cellobject::*;
pub use crate::ceval::*;
pub use crate::classobject::*;
pub use crate::cobject::*;
pub use crate::code::*;
pub use crate::compile::*;
pub use crate::complexobject::*;
pub use crate::descrobject::*;
pub use crate::dictobject::*;
pub use crate::enumobject::*;
pub use crate::eval::*;
pub use crate::fileobject::*;
pub use crate::floatobject::*;
pub use crate::frameobject::PyFrameObject;
pub use crate::funcobject::*;
pub use crate::genobject::*;
pub use crate::import::*;
pub use crate::intobject::*;
pub use crate::iterobject::*;
pub use crate::listobject::*;
pub use crate::longobject::*;
pub use crate::marshal::*;
pub use crate::memoryobject::*;
pub use crate::methodobject::*;
pub use crate::modsupport::*;
pub use crate::moduleobject::*;
pub use crate::object::*;
pub use crate::objectabstract::*;
pub use crate::objimpl::*;
pub use crate::pyarena::*;
pub use crate::pycapsule::*;
pub use crate::pydebug::*;
pub use crate::pyerrors::*;
pub use crate::pymem::*;
pub use crate::pyport::*;
pub use crate::pystate::PyGILState_STATE::*;
pub use crate::pystate::*;
pub use crate::pythonrun::*;
pub use crate::rangeobject::*;
pub use crate::setobject::*;
pub use crate::sliceobject::*;
pub use crate::stringobject::*;
pub use crate::structmember::PyMemberDef;
pub use crate::sysmodule::*;
pub use crate::traceback::*;
pub use crate::tupleobject::*;
#[cfg(py_sys_config = "Py_USING_UNICODE")]
pub use crate::unicodeobject::*;
pub use crate::warnings::*;
pub use crate::weakrefobject::*;

mod pyport;

mod pymem;

mod object;

mod objimpl;

mod pydebug;

// TODO: incomplete
#[cfg(py_sys_config = "Py_USING_UNICODE")]
mod unicodeobject;

mod intobject;

mod boolobject;

mod longobject;

mod floatobject;

mod complexobject;

mod rangeobject;

mod stringobject;

mod memoryobject;

mod bufferobject;

mod bytesobject;

mod bytearrayobject;

mod tupleobject;

mod listobject;

mod dictobject;

mod enumobject;

mod setobject;

mod methodobject;

mod moduleobject;

mod funcobject;

mod classobject;

mod fileobject;

mod cobject;

mod pycapsule;

mod traceback;

mod sliceobject;

mod cellobject;

mod iterobject;

mod genobject;

mod descrobject;

mod warnings;

mod weakrefobject;

// TODO: incomplete
// mod codecs;

mod pyerrors;

mod pystate;

mod pyarena;

mod modsupport;

mod pythonrun;

mod ceval;

mod sysmodule;

// TODO: incomplete
// mod intrcheck;

mod import;

mod objectabstract;

mod code;

mod compile;

mod eval;

mod marshal;

// TODO: incomplete
// mod pyctype;

// TODO: incomplete
// mod pystrtod;

// TODO: incomplete
// mod pystrcmp;

// TODO: incomplete
// mod dtoa;

// TODO: incomplete
// mod pyfpe;

// Additional headers that are not exported by Python.h

pub mod structmember;

pub mod frameobject;

pub const Py_single_input: libc::c_int = 256;

pub const Py_file_input: libc::c_int = 257;

pub const Py_eval_input: libc::c_int = 258;

#[cfg(not(py_sys_config = "Py_USING_UNICODE"))]
#[inline(always)]
pub fn PyUnicode_Check(op: *mut PyObject) -> libc::c_int {
    0
}

#[cfg(not(py_sys_config = "Py_USING_UNICODE"))]
#[inline(always)]
pub fn PyUnicode_CheckExact(op: *mut PyObject) -> libc::c_int {
    0
}

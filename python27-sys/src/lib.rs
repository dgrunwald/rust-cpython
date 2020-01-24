#![no_std]
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate libc;

pub use boolobject::*;
pub use bufferobject::*;
pub use bytearrayobject::*;
pub use bytesobject::*;
pub use cellobject::*;
pub use ceval::*;
pub use classobject::*;
pub use cobject::*;
pub use code::*;
pub use compile::*;
pub use complexobject::*;
pub use descrobject::*;
pub use dictobject::*;
pub use enumobject::*;
pub use eval::*;
pub use fileobject::*;
pub use floatobject::*;
pub use frameobject::PyFrameObject;
pub use funcobject::*;
pub use genobject::*;
pub use import::*;
pub use intobject::*;
pub use iterobject::*;
pub use listobject::*;
pub use longobject::*;
pub use marshal::*;
pub use memoryobject::*;
pub use methodobject::*;
pub use modsupport::*;
pub use moduleobject::*;
pub use object::*;
pub use objectabstract::*;
pub use objimpl::*;
pub use pyarena::*;
pub use pycapsule::*;
pub use pydebug::*;
pub use pyerrors::*;
pub use pymem::*;
pub use pyport::*;
pub use pystate::PyGILState_STATE::*;
pub use pystate::*;
pub use pythonrun::*;
pub use rangeobject::*;
pub use setobject::*;
pub use sliceobject::*;
pub use stringobject::*;
pub use structmember::PyMemberDef;
pub use sysmodule::*;
pub use traceback::*;
pub use tupleobject::*;
#[cfg(py_sys_config = "Py_USING_UNICODE")]
pub use unicodeobject::*;
pub use warnings::*;
pub use weakrefobject::*;

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

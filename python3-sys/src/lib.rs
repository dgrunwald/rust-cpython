#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_parens)]
#![cfg_attr(Py_LIMITED_API, allow(unused_imports))]

// old: marked with TODO
// Based on the headers of Python 3.4.3
// Supports the stable ABI (PEP 384) only.

// new:
// Based on the headers of Python 3.3.0, 3.4.0 and 3.5.0.

pub use crate::bltinmodule::*;
pub use crate::boolobject::*;
pub use crate::bytearrayobject::*;
pub use crate::bytesobject::*;
pub use crate::ceval::*;
pub use crate::code::*;
pub use crate::codecs::*;
pub use crate::compile::*;
pub use crate::complexobject::*;
pub use crate::descrobject::*;
pub use crate::dictobject::*;
pub use crate::enumobject::*;
pub use crate::eval::*;
pub use crate::fileobject::*;
#[cfg(Py_3_5)]
pub use crate::fileutils::*;
pub use crate::floatobject::*;
pub use crate::frameobject::PyFrameObject;
pub use crate::import::*;
#[cfg(all(Py_3_8, not(Py_LIMITED_API)))]
pub use crate::initconfig::*;
pub use crate::intrcheck::*;
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
#[cfg(Py_3_6)]
pub use crate::osmodule::*;
pub use crate::pyarena::*;
pub use crate::pycapsule::*;
pub use crate::pydebug::*;
pub use crate::pyerrors::*;
#[cfg(Py_3_4)]
pub use crate::pyhash::*;
pub use crate::pylifecycle::*;
pub use crate::pymem::*;
pub use crate::pyport::*;
pub use crate::pystate::*;
pub use crate::pystrtod::*;
pub use crate::pythonrun::*;
pub use crate::rangeobject::*;
pub use crate::setobject::*;
pub use crate::sliceobject::*;
pub use crate::structseq::*;
pub use crate::sysmodule::*;
pub use crate::traceback::*;
pub use crate::tupleobject::*;
pub use crate::typeslots::*;
pub use crate::unicodeobject::*;
pub use crate::warnings::*;
pub use crate::weakrefobject::*;

mod pyport;

// contains nothing of interest for Rust
// mod pymacro;

// contains nothing of interest for Rust; moved to internal/pycore_atomic.h in 3.8
// mod pyatomic;

// contains nothing of interest for Rust
// mod pymath;

// contains nothing of interest
// [cfg(not(Py_LIMITED_API))]
// mod pytime;

mod pymem;

mod object;

mod objimpl;

mod typeslots;

#[cfg(Py_3_4)]
mod pyhash;

mod pydebug;

mod bytearrayobject;

mod bytesobject;

mod unicodeobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod longobject;

// TODO excluded by PEP-384
// mod longintrepr;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod boolobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod floatobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod complexobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod rangeobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod memoryobject;

mod tupleobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod listobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod dictobject;

// TODO new in 3.5
// mod odictobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod enumobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod setobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod methodobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod moduleobject;

// TODO excluded by PEP-384
// mod funcobject;

// TODO excluded by PEP-384
// mod classobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod fileobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod pycapsule;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod traceback;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod sliceobject;

// TODO excluded by PEP-384
// mod cellobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod iterobject;

// TODO excluded by PEP-384
// mod genobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod descrobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod warnings;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod weakrefobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod structseq;

// TODO
// mod namespaceobject;

// TODO
// mod picklebufobject;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod codecs;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod pyerrors;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod pystate;

// TODO
// #[cfg(Py_3_7)]
// mod context;

#[cfg(Py_LIMITED_API)]
mod pyarena {}

// TODO: incomplete
#[cfg(not(Py_LIMITED_API))]
mod pyarena;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod modsupport;

// TODO some functions need to be moved to pylifecycle
mod pythonrun;

mod pylifecycle;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod ceval;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod sysmodule;

#[cfg(Py_3_6)]
mod osmodule;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod intrcheck;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod import;

mod objectabstract;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod bltinmodule;

#[cfg(Py_LIMITED_API)]
mod code {}

#[cfg(not(Py_LIMITED_API))]
mod code;

// TODO: incomplete
mod compile;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod eval;

// TODO excluded by PEP-384
// mod pyctype;

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
mod pystrtod;

// TODO nothing interesting for Rust?
// mod pystrcmp;

// TODO excluded by PEP-384
// mod dtoa;

#[cfg(Py_3_5)]
mod fileutils;

// TODO probably not interesting for rust
// mod pyfpe;

// TODO probably not interesting for rust
// mod tracemalloc;

// Additional headers that are not exported by Python.h

// TODO supports PEP-384 only; needs adjustment for Python 3.3 and 3.5
pub mod structmember;

pub mod frameobject;

mod marshal;

#[cfg(all(Py_3_8, not(Py_LIMITED_API)))]
mod initconfig;

#![allow(unused_imports)]  // imports only used in some configurations

use libc::{c_char, c_int};

use crate::code::*;
use crate::object::PyObject;
use crate::pyarena::*;
use crate::pythonrun::*;

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(not(Py_LIMITED_API))]
pub struct PyFutureFeatures {
    pub ff_features: c_int,
    pub ff_lineno: c_int,
}

// TODO: PyCF_MASK etc. constants

// Note: struct PyCompilerFlags was moved from pythonrun.h to compile.h in Python 3.7;
// We still have our version in pythonrun.rs

#[cfg(not(Py_LIMITED_API))]
pub const FUTURE_NESTED_SCOPES: &str = "nested_scopes";
#[cfg(not(Py_LIMITED_API))]
pub const FUTURE_GENERATORS: &str = "generators";
#[cfg(not(Py_LIMITED_API))]
pub const FUTURE_DIVISION: &str = "division";
#[cfg(not(Py_LIMITED_API))]
pub const FUTURE_ABSOLUTE_IMPORT: &str = "absolute_import";
#[cfg(not(Py_LIMITED_API))]
pub const FUTURE_WITH_STATEMENT: &str = "with_statement";
#[cfg(not(Py_LIMITED_API))]
pub const FUTURE_PRINT_FUNCTION: &str = "print_function";
#[cfg(not(Py_LIMITED_API))]
pub const FUTURE_UNICODE_LITERALS: &str = "unicode_literals";
#[cfg(not(Py_LIMITED_API))]
pub const FUTURE_BARRY_AS_BDFL: &str = "barry_as_FLUFL";
#[cfg(all(not(Py_LIMITED_API), Py_3_5))]
pub const FUTURE_GENERATOR_STOP: &str = "generator_stop";
#[cfg(all(not(Py_LIMITED_API), Py_3_7))]
pub const FUTURE_ANNOTATIONS: &str = "annotations";

#[cfg(not(Py_LIMITED_API))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg(not(Py_3_10))]
    pub fn PyNode_Compile(arg1: *mut _node, arg2: *const c_char) -> *mut PyCodeObject;
    #[cfg(not(Py_3_10))]
    pub fn PyAST_CompileEx(
        _mod: *mut _mod,
        filename: *const c_char,
        flags: *mut PyCompilerFlags,
        optimize: c_int,
        arena: *mut PyArena,
    ) -> *mut PyCodeObject;
    #[cfg(all(Py_3_4, not(Py_3_10)))]
    pub fn PyAST_CompileObject(
        _mod: *mut _mod,
        filename: *mut PyObject,
        flags: *mut PyCompilerFlags,
        optimize: c_int,
        arena: *mut PyArena,
    ) -> *mut PyCodeObject;
    #[cfg(not(Py_3_10))]
    pub fn PyFuture_FromAST(_mod: *mut _mod, filename: *const c_char) -> *mut PyFutureFeatures;
    #[cfg(all(Py_3_4, not(Py_3_10)))]
    pub fn PyFuture_FromASTObject(
        _mod: *mut _mod,
        filename: *mut PyObject,
    ) -> *mut PyFutureFeatures;
    #[cfg(Py_3_4)]
    pub fn PyCompile_OpcodeStackEffect(opcode: c_int, oparg: c_int) -> c_int;
    #[cfg(Py_3_8)]
    pub fn PyCompile_OpcodeStackEffectWithJump(opcode: c_int, oparg: c_int, jump: c_int) -> c_int;
}

pub const Py_single_input: c_int = 256;
pub const Py_file_input: c_int = 257;
pub const Py_eval_input: c_int = 258;
#[cfg(Py_3_8)]
pub const Py_func_type_input: c_int = 345;
#[cfg(Py_3_9)]
pub const Py_fstring_input: c_int = 800;

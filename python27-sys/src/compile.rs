use libc::{c_char, c_int};

use crate::code::*;
use crate::pyarena::PyArena;
use crate::pythonrun::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyFutureFeatures {
    pub ff_features: c_int,
    pub ff_lineno: c_int,
}

pub const FUTURE_NESTED_SCOPES: &str = "nested_scopes";
pub const FUTURE_GENERATORS: &str = "generators";
pub const FUTURE_DIVISION: &str = "division";
pub const FUTURE_ABSOLUTE_IMPORT: &str = "absolute_import";
pub const FUTURE_WITH_STATEMENT: &str = "with_statement";
pub const FUTURE_PRINT_FUNCTION: &str = "print_function";
pub const FUTURE_UNICODE_LITERALS: &str = "unicode_literals";

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyNode_Compile(arg1: *mut Struct__node, arg2: *const c_char) -> *mut PyCodeObject;
    pub fn PyAST_Compile(
        arg1: *mut Struct__mod,
        arg2: *const c_char,
        arg3: *mut PyCompilerFlags,
        arg4: *mut PyArena,
    ) -> *mut PyCodeObject;
    pub fn PyFuture_FromAST(arg1: *mut Struct__mod, arg2: *const c_char) -> *mut PyFutureFeatures;
}

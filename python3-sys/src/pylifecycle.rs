// There are 2 pylifecycle.h in CPython. We currently only define the Py_LIMITED_API
// symbols because other symbols exist in their legacy locations in other modules.

#[cfg(Py_3_8)]
#[cfg(not(Py_LIMITED_API))]
use crate::initconfig::{PyConfig, PyPreConfig, PyStatus};
#[cfg(Py_3_8)]
#[cfg(not(Py_LIMITED_API))]
use crate::pyport::Py_ssize_t;
#[cfg(Py_3_8)]
#[cfg(not(Py_LIMITED_API))]
use libc::{c_char, c_int, c_void, wchar_t, FILE};

// Symbols from Include/pylifecycle.h

// TODO move these symbols from their legacy locations into this module.

// Symbols from Include/cpython/pylifecycle.h

#[cfg(Py_3_8)]
#[cfg(not(Py_LIMITED_API))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg(Py_3_10)]
    pub fn Py_FrozenMain(argc: c_int, argv: *mut *mut c_char) -> c_int;

    // Py_SetStandardStreamEncoding: still in pythonrun.rs

    pub fn Py_PreInitialize(src_config: *const PyPreConfig) -> PyStatus;
    pub fn Py_PreInitializeFromBytesArgs(
        src_config: *const PyPreConfig,
        argc: Py_ssize_t,
        argv: *mut *mut c_char,
    ) -> PyStatus;
    pub fn Py_PreInitializeFromArgs(
        src_config: *const PyPreConfig,
        argc: Py_ssize_t,
        argv: *mut *mut wchar_t,
    ) -> PyStatus;

    pub fn _Py_IsCoreInitialized() -> c_int;

    pub fn Py_InitializeFromConfig(config: *const PyConfig) -> PyStatus;
    pub fn _Py_InitializeMain() -> PyStatus;
    pub fn Py_RunMain() -> c_int;

    pub fn Py_ExitStatusException(err: PyStatus) -> ();
    #[cfg(not(Py_3_10))]
    pub fn _Py_PyAtExit(
        func: Option<extern "C" fn(obj: *mut crate::object::PyObject) -> ()>,
        module: *mut crate::object::PyObject,
    ) -> ();
    pub fn _Py_RestoreSignals() -> ();
    pub fn Py_FdIsInteractive(file: *mut FILE, filename: *const c_char) -> c_int;
    pub fn _Py_SetProgramFullPath(path: *const wchar_t) -> ();
    pub fn _Py_gitidentifier() -> *const c_char;
    pub fn _Py_gitversion() -> *const c_char;
    pub fn _Py_IsFinalizing() -> c_int;
    pub fn _PyOS_URandom(buffer: *mut c_void, size: Py_ssize_t) -> c_int;
    pub fn _PyOS_URandomNonblock(buffer: *mut c_void, size: Py_ssize_t) -> c_int;
    pub fn _Py_CoerceLegacyLocale(warn: c_int) -> c_int;
    pub fn _Py_LegacyLocaleDetected(warn: c_int) -> c_int;
    pub fn _Py_SetLocaleFromEnv(category: c_int) -> *mut c_char;
}

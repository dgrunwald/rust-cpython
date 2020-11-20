// This entire module is Python 3.8+ and !Py_LIMITED_API only.

use crate::pyport::Py_ssize_t;
use libc::{c_char, c_int, c_ulong, wchar_t};
#[cfg(Py_3_9)]
use libc::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PyStatusType {
    _PyStatus_TYPE_OK = 0,
    _PyStatus_TYPE_ERROR = 1,
    _PyStatus_TYPE_EXIT = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyStatus {
    pub _type: PyStatusType,
    pub func: *const c_char,
    pub err_msg: *const c_char,
    pub exitcode: c_int,
}

impl Default for PyStatus {
    fn default() -> Self {
        // zeroed() is UB for enums. So we are explicit about
        // what value it is set to. This probably isn't necessary
        // as we are dealing with a C-compat struct and 0 is a valid
        // enum value. But explicit is better than UB.
        let mut status: Self = unsafe { core::mem::zeroed() };
        status._type = PyStatusType::_PyStatus_TYPE_OK;

        status
    }
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyStatus_Ok() -> PyStatus;
    pub fn PyStatus_Error(err_msg: *const c_char) -> PyStatus;
    pub fn PyStatus_NoMemory() -> PyStatus;
    pub fn PyStatus_Exit(exitcode: c_int) -> PyStatus;

    pub fn PyStatus_IsError(err: PyStatus) -> c_int;
    pub fn PyStatus_IsExit(err: PyStatus) -> c_int;
    pub fn PyStatus_Exception(err: PyStatus) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyWideStringList {
    pub length: Py_ssize_t,
    pub items: *mut *mut wchar_t,
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyWideStringList_Append(list: *mut PyWideStringList, item: *const wchar_t) -> PyStatus;
    pub fn PyWideStringList_Insert(
        list: *mut PyWideStringList,
        index: Py_ssize_t,
        item: *const wchar_t,
    ) -> PyStatus;
}

#[repr(C)]
#[derive(Clone)]
pub struct PyPreConfig {
    pub _config_init: c_int,
    pub parse_argv: c_int,
    pub isolated: c_int,
    pub use_environment: c_int,
    pub configure_locale: c_int,
    pub coerce_c_locale: c_int,
    pub coerce_c_locale_warn: c_int,
    #[cfg(windows)]
    pub legacy_windows_fs_encoding: c_int,
    pub utf8_mode: c_int,
    pub dev_mode: c_int,
    pub allocator: c_int,
}

impl Default for PyPreConfig {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyPreConfig_InitPythonConfig(config: *mut PyPreConfig) -> ();
    pub fn PyPreConfig_InitIsolatedConfig(config: *mut PyPreConfig) -> ();
}

#[repr(C)]
#[derive(Clone)]
pub struct PyConfig {
    pub _config_init: c_int,
    pub isolated: c_int,
    pub use_environment: c_int,
    pub dev_mode: c_int,
    pub install_signal_handlers: c_int,
    pub use_hash_seed: c_int,
    pub hash_seed: c_ulong,
    pub faulthandler: c_int,
    #[cfg(Py_3_9)]
    pub _use_peg_parser: c_int,
    pub tracemalloc: c_int,
    pub import_time: c_int,
    pub show_ref_count: c_int,
    #[cfg(not(Py_3_9))]
    pub show_alloc_count: c_int,
    pub dump_refs: c_int,
    pub malloc_stats: c_int,
    pub filesystem_encoding: *mut wchar_t,
    pub filesystem_errors: *mut wchar_t,
    pub pycache_prefix: *mut wchar_t,
    pub parse_argv: c_int,
    pub argv: PyWideStringList,
    pub program_name: *mut wchar_t,
    pub xoptions: PyWideStringList,
    pub warnoptions: PyWideStringList,
    pub site_import: c_int,
    pub bytes_warning: c_int,
    pub inspect: c_int,
    pub interactive: c_int,
    pub optimization_level: c_int,
    pub parser_debug: c_int,
    pub write_bytecode: c_int,
    pub verbose: c_int,
    pub quiet: c_int,
    pub user_site_directory: c_int,
    pub configure_c_stdio: c_int,
    pub buffered_stdio: c_int,
    pub stdio_encoding: *mut wchar_t,
    pub stdio_errors: *mut wchar_t,
    #[cfg(windows)]
    pub legacy_windows_stdio: c_int,
    pub check_hash_pycs_mode: *mut wchar_t,
    pub pathconfig_warnings: c_int,
    pub pythonpath_env: *mut wchar_t,
    pub home: *mut wchar_t,
    pub module_search_paths_set: c_int,
    pub module_search_paths: PyWideStringList,
    pub executable: *mut wchar_t,
    pub base_executable: *mut wchar_t,
    pub prefix: *mut wchar_t,
    pub base_prefix: *mut wchar_t,
    pub exec_prefix: *mut wchar_t,
    pub base_exec_prefix: *mut wchar_t,
    #[cfg(Py_3_9)]
    pub platlibdir: *mut wchar_t,
    pub skip_source_first_line: c_int,
    pub run_command: *mut wchar_t,
    pub run_module: *mut wchar_t,
    pub run_filename: *mut wchar_t,
    pub _install_importlib: c_int,
    pub _init_main: c_int,
    #[cfg(Py_3_9)]
    pub _isolated_interpreter: c_int,
    #[cfg(Py_3_9)]
    pub _orig_argv: PyWideStringList,
}

impl Default for PyConfig {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyConfig_InitPythonConfig(config: *mut PyConfig) -> ();
    pub fn PyConfig_InitIsolatedConfig(config: *mut PyConfig) -> ();
    pub fn PyConfig_Clear(config: *mut PyConfig) -> ();
    pub fn PyConfig_SetString(
        config: *mut PyConfig,
        config_str: *mut *mut wchar_t,
        value: *const wchar_t,
    ) -> PyStatus;
    pub fn PyConfig_SetBytesString(
        config: *mut PyConfig,
        config_str: *mut *mut wchar_t,
        value: *const c_char,
    ) -> PyStatus;
    pub fn PyConfig_Read(config: *mut PyConfig) -> PyStatus;
    pub fn PyConfig_SetBytesArgv(
        config: *mut PyConfig,
        argc: Py_ssize_t,
        argv: *const *mut c_char,
    ) -> PyStatus;
    pub fn PyConfig_SetArgv(
        config: *mut PyConfig,
        argc: Py_ssize_t,
        argv: *const *mut wchar_t,
    ) -> PyStatus;
    pub fn PyConfig_SetWideStringList(
        config: *mut PyConfig,
        list: *mut PyWideStringList,
        length: Py_ssize_t,
        items: *mut *mut wchar_t,
    ) -> PyStatus;

    #[cfg(Py_3_9)]
    pub fn Py_GetArgcArgv(argc: *mut c_int, argv: *mut *mut *mut wchar_t) -> c_void;
}

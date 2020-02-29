// This entire module is Python 3.8+ and !Py_LIMITED_API only.

use crate::pyport::Py_ssize_t;
use libc::{c_char, c_int, c_ulong, wchar_t};

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
    _type: PyStatusType,
    func: *const c_char,
    err_msg: *const c_char,
    exitcode: c_int,
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
    length: Py_ssize_t,
    items: *mut *mut wchar_t,
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
    _config_init: c_int,
    parse_argv: c_int,
    isolated: c_int,
    use_environment: c_int,
    configure_locale: c_int,
    coerce_c_locale: c_int,
    coerce_c_locale_warn: c_int,
    #[cfg(windows)]
    legacy_windows_fs_encoding: c_int,
    utf8_mode: c_int,
    dev_mode: c_int,
    allocator: c_int,
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyPreConfig_InitPythonConfig(config: *mut PyPreConfig) -> ();
    pub fn PyPreConfig_InitIsolatedConfig(config: *mut PyPreConfig) -> ();
}

#[repr(C)]
#[derive(Clone)]
pub struct PyConfig {
    _config_init: c_int,
    isolated: c_int,
    use_environment: c_int,
    dev_mode: c_int,
    install_signal_handlers: c_int,
    use_hash_seed: c_int,
    hash_seed: c_ulong,
    faulthandler: c_int,
    tracemalloc: c_int,
    import_time: c_int,
    show_ref_count: c_int,
    show_alloc_count: c_int,
    dump_refs: c_int,
    malloc_stats: c_int,
    filesystem_encoding: *mut wchar_t,
    filesystem_errors: *mut wchar_t,
    pycache_prefix: *mut wchar_t,
    parse_argv: c_int,
    argv: PyWideStringList,
    program_name: *mut wchar_t,
    xoptions: PyWideStringList,
    warnoptions: PyWideStringList,
    site_import: c_int,
    bytes_warning: c_int,
    inspect: c_int,
    interactive: c_int,
    optimization_level: c_int,
    parser_debug: c_int,
    write_bytecode: c_int,
    verbose: c_int,
    quiet: c_int,
    user_site_directory: c_int,
    configure_c_stdio: c_int,
    buffered_stdio: c_int,
    stdio_encoding: *mut wchar_t,
    stdio_errors: *mut wchar_t,
    #[cfg(windows)]
    legacy_windows_stdio: c_int,
    check_hash_pycs_mode: *mut wchar_t,
    pathconfig_warnings: c_int,
    pythonpath_env: *mut wchar_t,
    home: *mut wchar_t,
    module_search_paths_set: c_int,
    module_search_paths: PyWideStringList,
    executable: *mut wchar_t,
    base_executable: *mut wchar_t,
    prefix: *mut wchar_t,
    base_prefix: *mut wchar_t,
    exec_prefix: *mut wchar_t,
    base_exec_prefix: *mut wchar_t,
    skip_source_first_line: c_int,
    run_command: *mut wchar_t,
    run_module: *mut wchar_t,
    run_filename: *mut wchar_t,
    _install_importlib: c_int,
    _init_main: c_int,
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
}

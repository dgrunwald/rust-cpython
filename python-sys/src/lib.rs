use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/python_path_funcs.rs"));

pub fn python_path() -> Option<PathBuf> {
    match python_build_prefix() {
        Some(pth) => Some(Path::new(pth).join(Path::new("bin/python"))),
        None => None
    }
}

pub fn pip_path() -> Option<PathBuf> {
    match python_build_prefix() {
        Some(pth) => Some(Path::new(pth).join(Path::new("bin/pip"))),
        None => None
    }
}

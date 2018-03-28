extern crate python_sys;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ExitStatus};
use std::io::{Error, ErrorKind};

pub fn install_python_package(code_path: PathBuf, requirements_path: Option<PathBuf>) -> Result<ExitStatus, Error> {
    let pip_path = python_sys::pip_path().unwrap();
    let python_path = python_sys::python_path().unwrap();

    match requirements_path {
        Some(req_path) => {
            let _ = Command::new(&python_path)
                .args(&[pip_path.to_str().unwrap(), "install", "-r", req_path.to_str().unwrap()])
                .status();
            ()

        },
        None => ()
    };
    match python_sys::python_build_prefix() {
        Some(_prefix) => {
            Command::new(&python_path)
                .args(&[pip_path.to_str().unwrap(), "install", "-e", code_path.to_str().unwrap(), "--ignore-installed"])
                .stdout(Stdio::inherit())
                .status()
        },
        None => Err(Error::new(ErrorKind::Other, "There was a problem running python"))
    }
}

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ExitStatus};
use std::io::{Error, ErrorKind};

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

pub fn install_python_package(code_path: PathBuf, requirements_path: Option<PathBuf>) -> Result<ExitStatus, std::io::Error> {
    let pip_path = pip_path().unwrap();
    let python_path = python_path().unwrap();

    match requirements_path {
        Some(req_path) => {
            let _ = Command::new(&python_path)
                .args(&[pip_path.to_str().unwrap(), "install", "-r", req_path.to_str().unwrap()])
                .status();
            ()

        },
        None => ()
    };
    match python_build_prefix() {
        Some(_prefix) => {
            Command::new(&python_path)
                .args(&[code_path.join("setup.py").to_str().unwrap(), "install"])
                .stdout(Stdio::inherit())
                .status()
        },
        None => Err(Error::new(ErrorKind::Other, "There was a problem running python"))
    }
}

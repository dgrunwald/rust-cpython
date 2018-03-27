extern crate regex;

use std::env;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug)]
struct PythonVersion {
    major: u8,
    minor: u8,
    patch: Option<String>,
}

fn main() {
    if !Path::new("pyenv/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    let version = version_from_cargo_feature();

    let python_install_path: Option<PathBuf>;
    if !version.is_err() {
        println!("building python: {:?}", version);
        let pth = setup_python();
        println!("cargo:rustc-env=COMPILED_PYTHON_INTERPRETER_PATH=\"{}\"", pth.to_str().unwrap());

        python_install_path = Some(pth)
    }
    else{
        python_install_path = None;
    }
    generate_python_sys_helpers(&python_install_path)
}

/// Generates code that can run pip etc. from the installed Python
///
/// Creates a new module with a methods returning python_path
fn generate_python_sys_helpers(python_path: &Option<PathBuf>) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("python_path_funcs.rs");
    let mut f = File::create(&dest_path).unwrap();

    let python_path_func = match python_path {
        &Some(ref pth) => String::from(format!("Some(\"{}\")", pth.to_str().unwrap())),
        &None => String::from("None")
    };

    f.write_all(b"pub fn python_build_prefix() -> Option<&'static str> {")
        .unwrap();
    f.write_all(python_path_func.as_bytes())
        .unwrap();
    f.write_all(b"}").unwrap();

    let python_version = version_from_cargo_feature().unwrap();

    let python_version_suffix = match python_path {
        &Some(ref _pth) => String::from(format!("Some(\"{}.{}\")", python_version.major, python_version.minor)),
        &None => String::from("None")
    };

    f.write_all(b"pub fn python_version_suffix() -> Option<&'static str> {")
        .unwrap();
    f.write_all(python_version_suffix.as_bytes())
        .unwrap();
    f.write_all(b"}").unwrap();
}

/// Leverages the `python-build` scripts in pyenv to build a particular version of python.
/// Build artifacts are deposited in the `OUT_DIR` and the python path is returned
fn setup_python() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();

    let build_tool_install_dir = Path::new(&out_dir).join("python_build");
    let _ = fs::create_dir(&build_tool_install_dir);

    let _ = Command::new("pyenv/plugins/python-build/install.sh")
        .env("PREFIX", &build_tool_install_dir)
        .output()
        .expect("Could not install the python-build tool.");

    let python_version = format_python_version(version_from_cargo_feature().unwrap());

    // create the python directory
    let python_install_dir = Path::new(&out_dir).join(format!("python{}-build", python_version));
    let _ = fs::create_dir(&python_install_dir);

    if !python_install_dir.join("bin/python").exists() {
        let python_build_command = Path::new(&build_tool_install_dir).join("bin/python-build");

        let python_configure_opts;
        if cfg!(target_os = "macos") {
            python_configure_opts = "--enable-framework";
        }
        else{
            python_configure_opts = "";
        }
        let _status = Command::new(python_build_command)
            .args(&[
                  "-v".to_owned(),
                  format!("{}", python_version),
                  String::from(python_install_dir.to_str().unwrap())
            ])
            .env("PYTHON_CONFIGURE_OPTS", python_configure_opts)
            .stdout(Stdio::inherit())
            .status()
            .expect("build failed for python");
    }

    println!(
        r"cargo:rustc-link-search=native={}",
        Path::new(&python_install_dir).join("lib").to_str().unwrap()
    );

    python_install_dir
}

///
/// Format the python version as a triple e.g. 3.6.dev
///
fn format_python_version(version: PythonVersion) -> std::string::String {
    let version_string = format!("{}.{}", version.major, version.minor);
    if version.patch.is_some() {
        let patch = version.patch.unwrap();
        // test if we are building a `-dev` python
        if patch.parse::<u32>().is_err() {
            format!("{}-{}", version_string, patch.to_lowercase())
        } else {
            format!("{}.{}", version_string, patch)
        }
    } else {
        version_string
    }
}

/// Function copied from rust-python:
/// https://github.com/dgrunwald/rust-cpython/blob/master/python3-sys/build.rs
///
/// Determine the python version we're supposed to be building
/// from the features passed via the environment.
///
fn version_from_cargo_feature() -> Result<PythonVersion, String> {
    let re = Regex::new(r"CARGO_FEATURE_BUILD_PYTHON_(\d+)_(\d+)(_(.+))?").unwrap();
    let mut vars = env::vars().collect::<Vec<_>>();
    vars.sort_by(|a, b| b.cmp(a));
    for (key, _) in vars {
        match re.captures(&key) {
            Some(cap) => {
                return Ok(PythonVersion {
                    major: cap.get(1).unwrap().as_str().parse().unwrap(),
                    minor: cap.get(2).unwrap().as_str().parse().unwrap(),
                    patch: match cap.get(4) {
                        Some(s) => Some(String::from(s.as_str())),
                        None => None,
                    },
                })
            }
            None => (),
        }
    }
    Err(
        "Python version feature was not found. At least one python version \
         feature must be enabled."
            .to_owned(),
    )
}

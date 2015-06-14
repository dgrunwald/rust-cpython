extern crate pkg_config;
extern crate regex;

use std::process::Command;
use std::collections::HashMap;
use std::env;
use regex::Regex;
use std::fs;

struct PythonVersion {
    major: u8,
    // minor == None means any minor version will do
    minor: Option<u8>
}

const CFG_KEY: &'static str = "py_sys_config";

// windows' python writes out lines with the windows crlf sequence;
// posix platforms and mac os should write out lines with just lf.
#[cfg(target_os="windows")]
static NEWLINE_SEQUENCE: &'static str = "\r\n";

#[cfg(not(target_os="windows"))]
static NEWLINE_SEQUENCE: &'static str = "\n";

// A list of python interpreter compile-time preprocessor defines that 
// we will pick up and pass to rustc via --cfg=py_sys_config={varname};
// this allows using them conditional cfg attributes in the .rs files, so
//
// #[cfg(py_sys_config="{varname}"]
//
// is the equivalent of #ifdef {varname} name in C.
//
// see Misc/SpecialBuilds.txt in the python source for what these mean.
//
// (hrm, this is sort of re-implementing what distutils does, except 
// by passing command line args instead of referring to a python.h)
#[cfg(not(target_os="windows"))]
static SYSCONFIG_FLAGS: [&'static str; 7] = [
    "Py_USING_UNICODE",
    "Py_UNICODE_WIDE",
    "WITH_THREAD",
    "Py_DEBUG",
    "Py_REF_DEBUG",
    "Py_TRACE_REFS",
    "COUNT_ALLOCS",
];

static SYSCONFIG_VALUES: [&'static str; 1] = [
    // cfg doesn't support flags with values, just bools - so flags 
    // below are translated into bools as {varname}_{val} 
    //
    // for example, Py_UNICODE_SIZE_2 or Py_UNICODE_SIZE_4
    "Py_UNICODE_SIZE" // note - not present on python 3.3+, which is always wide
];

/// Examine python's compile flags to pass to cfg by launching
/// the interpreter and printing variables of interest from 
/// sysconfig.get_config_vars.
#[cfg(not(target_os="windows"))]
fn get_config_vars(python_path: &String) -> Result<HashMap<String, String>, String>  {
    let mut script = "import sysconfig; \
config = sysconfig.get_config_vars();".to_owned();

    for k in SYSCONFIG_FLAGS.iter().chain(SYSCONFIG_VALUES.iter()) {
        script.push_str(&format!("print(config.get('{}', {}))", k, 
            if is_value(k) { "None" } else { "0" } ));
        script.push_str(";");
    }

    let mut cmd = Command::new(python_path);
    cmd.arg("-c").arg(script);

    let out = try!(cmd.output().map_err(|e| {
        format!("failed to run python interpreter `{:?}`: {}", cmd, e)
    }));

    if !out.status.success() {
        let stderr = String::from_utf8(out.stderr).unwrap();
        let mut msg = format!("python script failed with stderr:\n\n");
        msg.push_str(&stderr);
        return Err(msg);
    }

    let stdout = String::from_utf8(out.stdout).unwrap();
    let split_stdout: Vec<&str> = stdout.trim_right().split(NEWLINE_SEQUENCE).collect();
    if split_stdout.len() != SYSCONFIG_VALUES.len() + SYSCONFIG_FLAGS.len() {
        return Err(
            format!("python stdout len didn't return expected number of lines:
{}", split_stdout.len()).to_string());
    }
    let all_vars = SYSCONFIG_FLAGS.iter().chain(SYSCONFIG_VALUES.iter());
    // let var_map: HashMap<String, String> = HashMap::new();
    Ok(all_vars.zip(split_stdout.iter())
        .fold(HashMap::new(), |mut memo: HashMap<String, String>, (&k, &v)| {
            if !(v.to_owned() == "None" && is_value(k)) {
                memo.insert(k.to_owned(), v.to_owned());
            }
            memo
        }))
}

#[cfg(target_os="windows")]
fn get_config_vars(_: &String) -> Result<HashMap<String, String>, String> {
    // sysconfig is missing all the flags on windows, so we can't actually
    // query the interpreter directly for its build flags. 
    //
    // For the time being, this is the flags as defined in the python source's
    // PC\pyconfig.h. This won't work correctly if someone has built their
    // python with a modified pyconfig.h - sorry if that is you, you will have
    // to comment/uncomment the lines below.
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("Py_USING_UNICODE".to_owned(), "1".to_owned());
    map.insert("Py_UNICODE_WIDE".to_owned(), "0".to_owned());
    map.insert("WITH_THREAD".to_owned(), "1".to_owned());
    map.insert("Py_UNICODE_SIZE".to_owned(), "2".to_owned());

    // This is defined #ifdef _DEBUG. The visual studio build seems to produce
    // a specially named pythonXX_d.exe and pythonXX_d.dll when you build the
    // Debug configuration, which this script doesn't currently support anyway.
    // map.insert("Py_DEBUG", "1");
    
    // Uncomment these manually if your python was built with these and you want
    // the cfg flags to be set in rust.
    //
    // map.insert("Py_REF_DEBUG", "1");
    // map.insert("Py_TRACE_REFS", "1");
    // map.insert("COUNT_ALLOCS", 1");
    Ok(map)
}

fn is_value(key: &str) -> bool {
    SYSCONFIG_VALUES.iter().find(|x| **x == key).is_some()
}

fn cfg_line_for_var(key: &str, val: &str) -> Option<String> {
    if is_value(key) {
        // is a value; suffix the key name with the value
        Some(format!("cargo:rustc-cfg={}=\"{}_{}\"\n", CFG_KEY, key, val))
    } else if val != "0" {
        // is a flag that isn't zero
        Some(format!("cargo:rustc-cfg={}=\"{}\"", CFG_KEY, key))
    } else {
        // is a flag that is zero
        None
    }
}

/// Run a python script using the 'python' located by PATH.
fn run_python_script(script: &str) -> Result<String, String> {
    let mut cmd = Command::new("python");
    cmd.arg("-c").arg(script);

    let out = try!(cmd.output().map_err(|e| {
        format!("failed to run python interpreter `{:?}`: {}", cmd, e)
    }));

    if !out.status.success() {
        let stderr = String::from_utf8(out.stderr).unwrap();
        let mut msg = format!("python script failed with stderr:\n\n");
        msg.push_str(&stderr);
        return Err(msg);
    }

    let out = String::from_utf8(out.stdout).unwrap();
    return Ok(out);
}

#[cfg(not(target_os="macos"))]
#[cfg(not(target_os="windows"))]
fn get_rustc_link_lib(version: &PythonVersion, enable_shared: bool) -> Result<String, String> {
    let dotted_version = format!("{}.{}", version.major, version.minor.unwrap());
    if enable_shared {
        Ok(format!("cargo:rustc-link-lib=python{}", dotted_version))
    } else {
        Ok(format!("cargo:rustc-link-lib=static=python{}", dotted_version))
    }
}

#[cfg(target_os="macos")]
fn get_macos_linkmodel() -> Result<String, String> {
    let script = "import MacOS; print MacOS.linkmodel;";
    let out = run_python_script(script).unwrap();
    Ok(out.trim_right().to_owned())
}

#[cfg(target_os="macos")]
fn get_rustc_link_lib(version: &PythonVersion, _: bool) -> Result<String, String> {
    // os x can be linked to a framework or static or dynamic, and 
    // Py_ENABLE_SHARED is wrong; framework means shared library
    let dotted_version = format!("{}.{}", version.major, version.minor.unwrap());
    match get_macos_linkmodel().unwrap().as_ref() {
        "static" => Ok(format!("cargo:rustc-link-lib=static=python{}",
            dotted_version)),
        "dynamic" => Ok(format!("cargo:rustc-link-lib=python{}",
            dotted_version)),
        "framework" => Ok(format!("cargo:rustc-link-lib=python{}", 
            dotted_version)),
        other => Err(format!("unknown linkmodel {}", other))
    }
}

/// Check that the interpreter's reported version matches expected_version;
/// if it does, return the specific version the interpreter reports.
fn get_interpreter_version(line: &str, expected_version: &PythonVersion)
        -> Result<PythonVersion, String> {
    let version_re = Regex::new(r"\((\d+), (\d+)\)").unwrap();
    let interpreter_version = match version_re.captures(&line) {
        Some(cap) => PythonVersion {
            major: cap.at(1).unwrap().parse().unwrap(),
            minor: Some(cap.at(2).unwrap().parse().unwrap())
        },
        None => return Err(
            format!("Unexpected response to version query {}", line))
    };

    if interpreter_version.major != expected_version.major ||
        (expected_version.minor.is_some() &&
            interpreter_version.minor != expected_version.minor)
    {
        Err(format!("'python' is not version {}.{} (is {})",
                    expected_version.major,
                    match expected_version.minor {
                        Some(v) => v.to_string(),
                        None => "*".to_owned()
                    },
                    line))
    } else {
        Ok(interpreter_version)
    }
}

#[cfg(target_os="windows")]
fn get_rustc_link_lib(version: &PythonVersion, _: bool) -> Result<String, String> {
    // Py_ENABLE_SHARED doesn't seem to be present on windows.
    Ok(format!("cargo:rustc-link-lib=python{}{}", version.major, 
        match version.minor {
            Some(minor) => minor.to_string(),
            None => "".to_owned()
        }))
}

/// Deduce configuration from the 'python' in the current PATH and print
/// cargo vars to stdout.
///
/// Note that if the python doesn't satisfy expected_version, this will error.
fn configure_from_path(expected_version: &PythonVersion) -> Result<String, String> {
    let script = "import sys; import sysconfig; print(sys.version_info[0:2]); \
print(sysconfig.get_config_var('LIBDIR')); \
print(sysconfig.get_config_var('Py_ENABLE_SHARED')); \
print(sys.exec_prefix);";
    let out = run_python_script(script).unwrap();
    let lines: Vec<&str> = out.split(NEWLINE_SEQUENCE).collect();
    let version: &str = lines[0];
    let libpath: &str = lines[1];
    let enable_shared: &str = lines[2];
    let exec_prefix: &str = lines[3];

    let interpreter_version = try!(get_interpreter_version(version,
        expected_version));

    println!("{}", get_rustc_link_lib(&interpreter_version,
        enable_shared == "1").unwrap());

    if libpath != "None" {
        println!("cargo:rustc-link-search=native={}", libpath);
    }

    let rel_interpreter_path = if cfg!(target_os="windows") {
        "/python"
    } else {
        "/bin/python"
    };

    return Ok(format!("{}{}", exec_prefix, rel_interpreter_path));
}

/// Deduce configuration from the python-X.X in pkg-config and print
/// cargo vars to stdout.
fn configure_from_pkgconfig(version: &PythonVersion, pkg_name: &str) 
        -> Result<String, String> {
    // this emits relevant build info to stdout, which is picked up by the
    // build chain (funny name for something with side-effects!)
    try!(pkg_config::find_library(pkg_name));

    // This seems to be a convention - unfortunately pkg-config doesn't
    // tell you the executable name, but I've noticed at least on 
    // OS X homebrew the python bin dir for 3.4 doesn't actually contain
    // a 'python'.
    let exec_prefix = pkg_config::Config::get_variable(pkg_name, 
        "exec_prefix").unwrap();

    // try to find the python interpreter in the exec_prefix somewhere.
    // the .pc doesn't tell us :(

    let mut attempts = vec![
        format!("/bin/python{}", version.major),
        "/bin/python".to_owned()
    ];

    // Try to seek python(major).(minor) if the user specified a minor.
    //
    // Ideally, we'd still do this even if they didn't based off the
    // specific version of the package located by pkg_config above,
    // but it's not obvious how to reliably extract that out of the .pc.
    if version.minor.is_some() {
        attempts.insert(0, format!("/bin/python{}_{}", version.major,
            version.minor.unwrap()));
    }

    for attempt in attempts.iter() {
        let possible_exec_name = format!("{}{}", exec_prefix,
            attempt);
        match fs::metadata(&possible_exec_name) {
            Ok(_) => return Ok(possible_exec_name),
            Err(_) => ()
        };
    }
    return Err("Unable to locate python interpreter".to_owned());
}

/// Determine the python version we're supposed to be building
/// from the features passed via the environment.
///
/// The environment variable can choose to omit a minor
/// version if the user doesn't care.
fn version_from_env() -> Result<PythonVersion, String> {
    let re = Regex::new(r"CARGO_FEATURE_PYTHON_(\d+)(_(\d+))?").unwrap();
    // sort env::vars so we get more explicit version specifiers first
    // so if the user passes e.g. the python-3 feature and the python-3-5 
    // feature, python-3-5 takes priority.
    let mut vars = env::vars().collect::<Vec<_>>();
    vars.sort_by(|a, b| b.cmp(a));
    for (key, _) in vars {
        match re.captures(&key) {
            Some(cap) => return Ok(PythonVersion { 
                major: cap.at(1).unwrap().parse().unwrap(), 
                minor: match cap.at(3) {
                    Some(s) => Some(s.parse().unwrap()),
                    None => None
                }
            }),
            None => ()
        }
    }
    Err("Python version feature was not found. At least one python version \
         feature must be enabled.".to_owned())
}

fn main() {
    // 1. Setup cfg variables so we can do conditional compilation in this 
    // library based on the python interpeter's compilation flags. This is 
    // necessary for e.g. matching the right unicode and threading interfaces.
    //
    // By default, try to use pkgconfig - this seems to be a rust norm.
    //
    // If you want to use a different python, setting the appropriate
    // PYTHON_X.X_NO_PKG_CONFIG environment variable will cause the script 
    // to pick up the python in your PATH; e.g. for python27 X.X is 2.7.
    // 
    // This will work smoothly with an activated virtualenv.
    // 
    // If you have troubles with your shell accepting '.' in a var name, 
    // try using 'env' (sorry but this isn't our fault - it just has to 
    // match the pkg-config package name, which is going to have a . in it).
    let version = version_from_env().unwrap();
    let pkg_name = match version.minor {
        Some(minor) => format!("python-{}.{}", version.major, minor),
        None => format!("python{}", version.major)
    };

    let python_interpreter_path = match configure_from_pkgconfig(&version, &pkg_name) {
        Ok(p) => p,
        // no pkgconfig - either it failed or user set the environment 
        // variable "PYTHON_2.7_NO_PKG_CONFIG".
        Err(_) => configure_from_path(&version).unwrap()
    };

    let config_map = get_config_vars(&python_interpreter_path).unwrap();

    for (key, val) in &config_map {
        match cfg_line_for_var(key, val) {
            Some(line) => println!("{}", line),
            None => ()
        }
    }

    // 2. Export python interpreter compilation flags as cargo variables that 
    // will be visible to dependents. All flags will be available to dependent
    // build scripts in the environment variable DEP_PYTHON27_PYTHON_FLAGS as 
    // comma separated list; each item in the list looks like
    //
    // {VAL,FLAG}_{flag_name}=val;
    //
    // FLAG indicates the variable is always 0 or 1
    // VAL indicates it can take on any value
    //
    // rust-cypthon/build.rs contains an example of how to unpack this data
    // into cfg flags that replicate the ones present in this library, so 
    // you can use the same cfg syntax.
    let flags: String = config_map.iter().fold("".to_owned(), |memo, (key, val)| {
        if is_value(key) {
            memo + format!("VAL_{}={},", key, val).as_ref()
        } else if val != "0" {
            memo + format!("FLAG_{}={},", key, val).as_ref()
        } else {
            memo
        }
    });
    println!("cargo:python_flags={}", 
        if flags.len() > 0 { &flags[..flags.len()-1] } else { "" });
}
#![crate_type = "dylib"]

use cpython::{py_fn, py_module_initializer, PyDict, PyNone, PyResult, PyTuple, Python};

py_module_initializer!(hello, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add(py, "run", py_fn!(py, run(*args, **kwargs)))?;
    m.add(py, "val", py_fn!(py, val()))?;
    Ok(())
});

fn run(py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyNone> {
    println!("Rust says: Hello Python!");
    for arg in args.iter(py) {
        println!("Rust got {}", arg);
    }
    if let Some(kwargs) = kwargs {
        for (key, val) in kwargs.items(py) {
            println!("{} = {}", key, val);
        }
    }
    Ok(PyNone)
}

fn val(_: Python) -> PyResult<i32> {
    Ok(42)
}

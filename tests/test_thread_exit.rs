use cpython::*;

fn test_thread_exit_py(py: Python) -> PyResult<()> {
    let m = py.import("sys")?;
    m.add(py, "exit_thread", py_fn!(py, exit_thread()))?;
    py.run(
        "
import sys, threading
# should not abort
threading.Thread(target=sys.exit_thread).start()
",
        None,
        None,
    )?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok(())
}

// This test requires Rust 1.40, namely
// https://github.com/rust-lang/rust/pull/65646.
//
// `pthread_exit` on Linux raises a C++ `__foced_unwind` exception to unwind
// the stack. That exception does not inherit from C++ `std::exception` and
// must be rethrown if caught, otherwise pthread will abort with
// `FATAL: exception not rethrown`. On the Rust land, `catch_unwind` before
// PR65646 will incorrectly silent that C++ exception without rethrowing,
// breaking this test.
//
// Also skip nightly compilers, which might include some unstable changes
// affecting the test.
#[rustversion::all(since(1.40), stable)]
#[test]
fn test_thread_exit() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    test_thread_exit_py(py).unwrap();
}

fn exit_thread(py: Python) -> PyResult<String> {
    #[cfg(unix)]
    {
        py.allow_threads(|| {
            // Emulates PyThread_exit_thread. It can happen during Py_Finalize.
            unsafe { libc::pthread_exit(std::ptr::null_mut()) };
        })
    }
    #[cfg(not(unix))]
    {
        let _ = py;
        // pthread might not exist on this platform.
        Ok(String::new())
    }
}

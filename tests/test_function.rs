use cpython::{py_fn, NoArgs, ObjectProtocol, PyDict, PyResult, Python};
use std::sync::atomic;
use std::sync::atomic::Ordering::Relaxed;

#[test]
fn no_args() {
    static CALL_COUNT: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

    fn f(_py: Python) -> PyResult<usize> {
        Ok(CALL_COUNT.fetch_add(1, Relaxed))
    }

    let gil = Python::acquire_gil();
    let py = gil.python();
    let obj = py_fn!(py, f());

    assert_eq!(CALL_COUNT.load(Relaxed), 0);
    assert_eq!(
        obj.call(py, NoArgs, None)
            .unwrap()
            .extract::<i32>(py)
            .unwrap(),
        0
    );
    assert_eq!(
        obj.call(py, NoArgs, None)
            .unwrap()
            .extract::<i32>(py)
            .unwrap(),
        1
    );
    assert_eq!(CALL_COUNT.load(Relaxed), 2);
    assert!(obj.call(py, (1,), None).is_err());
    assert_eq!(CALL_COUNT.load(Relaxed), 2);
    assert_eq!(
        obj.call(py, NoArgs, Some(&PyDict::new(py)))
            .unwrap()
            .extract::<i32>(py)
            .unwrap(),
        2
    );
    assert_eq!(CALL_COUNT.load(Relaxed), 3);
    let dict = PyDict::new(py);
    dict.set_item(py, "param", 42).unwrap();
    assert!(obj.call(py, NoArgs, Some(&dict)).is_err());
    assert_eq!(CALL_COUNT.load(Relaxed), 3);
}

#[test]
fn one_arg() {
    fn f(_py: Python, i: usize) -> PyResult<usize> {
        Ok(i * 2)
    }

    let gil = Python::acquire_gil();
    let py = gil.python();
    let obj = py_fn!(py, f(i: usize));

    assert!(obj.call(py, NoArgs, None).is_err());
    assert_eq!(
        obj.call(py, (1,), None)
            .unwrap()
            .extract::<i32>(py)
            .unwrap(),
        2
    );
    assert!(obj.call(py, (1, 2), None).is_err());

    let dict = PyDict::new(py);
    dict.set_item(py, "i", 42).unwrap();
    assert_eq!(
        obj.call(py, NoArgs, Some(&dict))
            .unwrap()
            .extract::<i32>(py)
            .unwrap(),
        84
    );
    assert!(obj.call(py, (1,), Some(&dict)).is_err());
    dict.set_item(py, "j", 10).unwrap();
    assert!(obj.call(py, NoArgs, Some(&dict)).is_err());
}

#[test]
fn inline_two_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let obj = py_fn!(py, f(a: i32, b: i32) -> PyResult<i32> {
        drop(py); // avoid unused variable warning
        Ok(a * b)
    });

    assert!(obj.call(py, NoArgs, None).is_err());
    assert_eq!(
        obj.call(py, (6, 7), None)
            .unwrap()
            .extract::<i32>(py)
            .unwrap(),
        42
    );
}

#[test]
fn opt_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let obj = py_fn!(py, f(a: Option<&str>, b: &str, c: Option<&str> = None) -> PyResult<String> {
        drop(py);
        Ok(format!("a: {:?}  b: {:?}  c: {:?}", a, b, c))
    });

    assert_eq!(
        obj.call(py, (py.None(), "string"), None)
            .unwrap()
            .extract::<String>(py)
            .unwrap(),
        r#"a: None  b: "string"  c: None"#,
    );
    assert_eq!(
        obj.call(py, ("double", "string", py.None()), None)
            .unwrap()
            .extract::<String>(py)
            .unwrap(),
        r#"a: Some("double")  b: "string"  c: None"#,
    );
    assert_eq!(
        obj.call(py, ("triple", "string", "args"), None)
            .unwrap()
            .extract::<String>(py)
            .unwrap(),
        r#"a: Some("triple")  b: "string"  c: Some("args")"#,
    );
}

/* TODO: reimplement flexible sig support
#[test]
fn flexible_sig() {
    fn f(py: Python, args: &PyTuple, kwargs: &PyDict) -> PyResult<usize> {
        Ok(args.len(py) + 100 * kwargs.map_or(0, |kwargs| kwargs.len(py)))
    }

    let gil = Python::acquire_gil();
    let py = gil.python();
    let obj = py_fn!(f(*args, **kwargs)).to_py_object(py);

    assert_eq!(obj.call(py, NoArgs, None).unwrap().extract::<i32>(py).unwrap(), 0);
    assert_eq!(obj.call(py, (1,), None).unwrap().extract::<i32>(py).unwrap(), 1);
    assert_eq!(obj.call(py, (1,2), None).unwrap().extract::<i32>(py).unwrap(), 2);

    let dict = PyDict::new(py);
    dict.set_item(py, "i", 42).unwrap();
    assert_eq!(obj.call(py, NoArgs, Some(&dict)).unwrap().extract::<i32>(py).unwrap(), 100);
    assert_eq!(obj.call(py, (1,2), Some(&dict)).unwrap().extract::<i32>(py).unwrap(), 102);
    dict.set_item(py, "j", 10).unwrap();
    assert_eq!(obj.call(py, (1,2,3), Some(&dict)).unwrap().extract::<i32>(py).unwrap(), 203);
}
*/

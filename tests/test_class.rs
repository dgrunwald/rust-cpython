#![allow(dead_code, unused_variables)]

use cpython::_detail::ffi;
use cpython::*;
use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{isize, iter, mem};

macro_rules! py_run {
    ($py:expr, $val:ident, $code:expr) => {{
        let d = PyDict::new($py);
        d.set_item($py, stringify!($val), &$val).unwrap();
        $py.run($code, None, Some(&d)).expect($code);
    }};
}

macro_rules! py_assert {
    ($py:expr, $val:ident, $assertion:expr) => {
        py_run!($py, $val, concat!("assert ", $assertion))
    };
}

macro_rules! py_expect_exception {
    ($py:expr, $val:ident, $code:expr, $err:ident) => {{
        let d = PyDict::new($py);
        d.set_item($py, stringify!($val), &$val).unwrap();
        let res = $py.run($code, None, Some(&d));
        let err = res.unwrap_err();
        if !err.matches($py, $py.get_type::<exc::$err>()) {
            panic!(format!("Expected {} but got {:?}", stringify!($err), err))
        }
    }};
}

py_class!(class EmptyClass |py| { });

#[test]
fn empty_class() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let typeobj = py.get_type::<EmptyClass>();
    // By default, don't allow creating instances from python.
    assert!(typeobj.call(py, NoArgs, None).is_err());

    py_assert!(py, typeobj, "typeobj.__name__ == 'EmptyClass'");
}

py_class!(class EmptyClassInModule |py| { });

#[test]
fn empty_class_in_module() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let module = PyModule::new(py, "test_module.nested").unwrap();
    module.add_class::<EmptyClassInModule>(py).unwrap();

    let ty = module.get(py, "EmptyClassInModule").unwrap();
    assert_eq!(
        ty.getattr(py, "__name__")
            .unwrap()
            .extract::<String>(py)
            .unwrap(),
        "EmptyClassInModule"
    );
    assert_eq!(
        ty.getattr(py, "__module__")
            .unwrap()
            .extract::<String>(py)
            .unwrap(),
        "test_module.nested"
    );
}

py_class!(class EmptyClassWithNew |py| {
    def __new__(_cls) -> PyResult<EmptyClassWithNew> {
        EmptyClassWithNew::create_instance(py)
    }
});

#[test]
fn empty_class_with_new() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let typeobj = py.get_type::<EmptyClassWithNew>();
    assert!(typeobj
        .call(py, NoArgs, None)
        .unwrap()
        .cast_into::<EmptyClassWithNew>(py)
        .is_ok());
}

py_class!(class NewWithOneArg |py| {
    data _data: i32;
    def __new__(_cls, arg: i32) -> PyResult<NewWithOneArg> {
        NewWithOneArg::create_instance(py, arg)
    }
});

#[test]
fn new_with_one_arg() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let typeobj = py.get_type::<NewWithOneArg>();
    let obj = typeobj
        .call(py, (42,), None)
        .unwrap()
        .cast_into::<NewWithOneArg>(py)
        .unwrap();
    assert_eq!(*obj._data(py), 42);
}

py_class!(class NewWithTwoArgs |py| {
    data _data1: i32;
    data _data2: i32;
    def __new__(_cls, arg1: i32, arg2: i32) -> PyResult<NewWithTwoArgs> {
        NewWithTwoArgs::create_instance(py, arg1, arg2)
    }
});

#[test]
fn new_with_two_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let typeobj = py.get_type::<NewWithTwoArgs>();
    let obj = typeobj
        .call(py, (10, 20), None)
        .unwrap()
        .cast_into::<NewWithTwoArgs>(py)
        .unwrap();
    assert_eq!(*obj._data1(py), 10);
    assert_eq!(*obj._data2(py), 20);
}

struct TestDropCall {
    drop_called: Arc<AtomicBool>,
}
impl Drop for TestDropCall {
    fn drop(&mut self) {
        self.drop_called.store(true, Ordering::Relaxed);
    }
}

py_class!(class DataIsDropped |py| {
    data member1: TestDropCall;
    data member2: TestDropCall;
});

#[test]
fn data_is_dropped() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let drop_called1 = Arc::new(AtomicBool::new(false));
    let drop_called2 = Arc::new(AtomicBool::new(false));
    let inst = DataIsDropped::create_instance(
        py,
        TestDropCall {
            drop_called: drop_called1.clone(),
        },
        TestDropCall {
            drop_called: drop_called2.clone(),
        },
    );
    assert!(drop_called1.load(Ordering::Relaxed) == false);
    assert!(drop_called2.load(Ordering::Relaxed) == false);
    drop(inst);
    assert!(drop_called1.load(Ordering::Relaxed) == true);
    assert!(drop_called2.load(Ordering::Relaxed) == true);
}

py_class!(class InstanceMethod |py| {
    data member: i32;

    def method(&self) -> PyResult<i32> {
        Ok(*self.member(py))
    }
});

#[test]
fn instance_method() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let obj = InstanceMethod::create_instance(py, 42).unwrap();
    assert!(obj.method(py).unwrap() == 42);
    let d = PyDict::new(py);
    d.set_item(py, "obj", obj).unwrap();
    py.run("assert obj.method() == 42", None, Some(&d)).unwrap();
}

py_class!(class InstanceMethodWithArgs |py| {
    data member: i32;

    def method(&self, multiplier: i32) -> PyResult<i32> {
        Ok(*self.member(py) * multiplier)
    }

    def r#match(&self, r#match: i32) -> PyResult<i32> {
        Ok(r#match)
    }
});

#[test]
fn instance_method_with_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let obj = InstanceMethodWithArgs::create_instance(py, 7).unwrap();
    assert!(obj.method(py, 6).unwrap() == 42);
    let d = PyDict::new(py);
    d.set_item(py, "obj", obj).unwrap();
    py.run("assert obj.method(3) == 21", None, Some(&d))
        .unwrap();
    py.run("assert obj.method(multiplier=6) == 42", None, Some(&d))
        .unwrap();
    py.run("assert obj.match(match=3) == 3", None, Some(&d))
        .unwrap();
}

py_class!(class ClassMethod |py| {
    def __new__(cls) -> PyResult<ClassMethod> {
        ClassMethod::create_instance(py)
    }

    @classmethod
    def method(cls) -> PyResult<String> {
        Ok(format!("{}.method()!", cls.name(py)))
    }
});

#[test]
fn class_method() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let d = PyDict::new(py);
    d.set_item(py, "C", py.get_type::<ClassMethod>()).unwrap();
    py.run(
        "assert C.method() == 'ClassMethod.method()!'",
        None,
        Some(&d),
    )
    .unwrap();
    py.run(
        "assert C().method() == 'ClassMethod.method()!'",
        None,
        Some(&d),
    )
    .unwrap();
}

py_class!(class ClassMethodWithArgs |py| {
    @classmethod
    def method(cls, input: &str) -> PyResult<String> {
        Ok(format!("{}.method({})", cls.name(py), input))
    }
});

#[test]
fn class_method_with_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let d = PyDict::new(py);
    d.set_item(py, "C", py.get_type::<ClassMethodWithArgs>())
        .unwrap();
    py.run(
        "assert C.method('abc') == 'ClassMethodWithArgs.method(abc)'",
        None,
        Some(&d),
    )
    .unwrap();
}

py_class!(class StaticMethod |py| {
    def __new__(cls) -> PyResult<StaticMethod> {
        StaticMethod::create_instance(py)
    }

    @staticmethod
    def method() -> PyResult<&'static str> {
        Ok("StaticMethod.method()!")
    }
});

#[test]
fn static_method() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    assert_eq!(StaticMethod::method(py).unwrap(), "StaticMethod.method()!");
    let d = PyDict::new(py);
    d.set_item(py, "C", py.get_type::<StaticMethod>()).unwrap();
    py.run(
        "assert C.method() == 'StaticMethod.method()!'",
        None,
        Some(&d),
    )
    .unwrap();
    py.run(
        "assert C().method() == 'StaticMethod.method()!'",
        None,
        Some(&d),
    )
    .unwrap();
}

py_class!(class StaticMethodWithArgs |py| {
    @staticmethod
    def method(input: i32) -> PyResult<String> {
        Ok(format!("0x{:x}", input))
    }
});

#[test]
fn static_method_with_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    assert_eq!(StaticMethodWithArgs::method(py, 1234).unwrap(), "0x4d2");
    let d = PyDict::new(py);
    d.set_item(py, "C", py.get_type::<StaticMethodWithArgs>())
        .unwrap();
    py.run("assert C.method(1337) == '0x539'", None, Some(&d))
        .unwrap();
}

py_class!(class StaticData |py| {
    static VAL1 = 123;
    static VAL2 = py.None();
});

#[test]
fn static_data() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let d = PyDict::new(py);
    d.set_item(py, "C", py.get_type::<StaticData>()).unwrap();
    py.run("assert C.VAL1 == 123", None, Some(&d)).unwrap();
    py.run("assert C.VAL2 is None", None, Some(&d)).unwrap();
    assert!(py.run("C.VAL1 = 124", None, Some(&d)).is_err());
}

py_class!(class GCIntegration |py| {
    data self_ref: RefCell<PyObject>;
    data dropped: TestDropCall;

    def __traverse__(&self, visit) {
        visit.call(&*self.self_ref(py).borrow())
    }

    def __clear__(&self) {
        let old_ref = mem::replace(&mut *self.self_ref(py).borrow_mut(), py.None());
        // Release reference only after the mutable borrow has expired.
        old_ref.release_ref(py);
    }
});

#[test]
fn gc_integration() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let drop_called = Arc::new(AtomicBool::new(false));
    let inst = GCIntegration::create_instance(
        py,
        RefCell::new(py.None()),
        TestDropCall {
            drop_called: drop_called.clone(),
        },
    )
    .unwrap();
    *inst.self_ref(py).borrow_mut() = inst.as_object().clone_ref(py);
    inst.release_ref(py);

    py.run("import gc; gc.collect()", None, None).unwrap();
    assert!(drop_called.load(Ordering::Relaxed));
}

py_class!(class Len |py| {
    data l: usize;

    def __len__(&self) -> PyResult<usize> {
        Ok(*self.l(py))
    }
});

#[test]
fn len() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let inst = Len::create_instance(py, 10).unwrap();
    py_assert!(py, inst, "len(inst) == 10");
    unsafe {
        assert_eq!(ffi::PyObject_Size(inst.as_object().as_ptr()), 10);
        assert_eq!(ffi::PyMapping_Size(inst.as_object().as_ptr()), 10);
    }

    let inst = Len::create_instance(py, (isize::MAX as usize) + 1).unwrap();
    py_expect_exception!(py, inst, "len(inst)", OverflowError);
}

py_class!(class Iterator |py| {
    data iter: RefCell<Box<dyn iter::Iterator<Item=i32> + Send>>;

    def __iter__(&self) -> PyResult<Iterator> {
        Ok(self.clone_ref(py))
    }

    def __next__(&self) -> PyResult<Option<i32>> {
        Ok(self.iter(py).borrow_mut().next())
    }
});

#[test]
fn iterator() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let inst = Iterator::create_instance(py, RefCell::new(Box::new(5..8))).unwrap();
    py_assert!(py, inst, "iter(inst) is inst");
    py_assert!(py, inst, "list(inst) == [5, 6, 7]");
}

py_class!(class StringMethods |py| {
    def __str__(&self) -> PyResult<&'static str> {
        Ok("str")
    }

    def __repr__(&self) -> PyResult<&'static str> {
        Ok("repr")
    }

    def __format__(&self, format_spec: &str) -> PyResult<String> {
        Ok(format!("format({})", format_spec))
    }

    def __unicode__(&self) -> PyResult<PyUnicode> {
        Ok(PyUnicode::new(py, "unicode"))
    }

    def __bytes__(&self) -> PyResult<PyBytes> {
        Ok(PyBytes::new(py, b"bytes"))
    }
});

#[test]
fn string_methods() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let obj = StringMethods::create_instance(py).unwrap();
    py_assert!(py, obj, "str(obj) == 'str'");
    py_assert!(py, obj, "repr(obj) == 'repr'");
    py_assert!(py, obj, "'{0:x}'.format(obj) == 'format(x)'");
}

#[test]
#[cfg(feature = "python27-sys")]
fn python2_string_methods() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let obj = StringMethods::create_instance(py).unwrap();
    py_assert!(py, obj, "unicode(obj) == u'unicode'");
}

#[test]
#[cfg(feature = "python3-sys")]
fn python3_string_methods() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let obj = StringMethods::create_instance(py).unwrap();
    py_assert!(py, obj, "bytes(obj) == b'bytes'");
}

py_class!(class Comparisons |py| {
    data val: i32;

    def __hash__(&self) -> PyResult<i32> {
        Ok(*self.val(py))
    }

    def __bool__(&self) -> PyResult<bool> {
        Ok(*self.val(py) != 0)
    }
});

#[test]
fn comparisons() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let zero = Comparisons::create_instance(py, 0).unwrap();
    let one = Comparisons::create_instance(py, 1).unwrap();
    let ten = Comparisons::create_instance(py, 10).unwrap();
    let minus_one = Comparisons::create_instance(py, -1).unwrap();
    py_assert!(py, one, "hash(one) == 1");
    py_assert!(py, ten, "hash(ten) == 10");
    py_assert!(py, minus_one, "hash(minus_one) == -2");

    py_assert!(py, one, "bool(one) is True");
    py_assert!(py, zero, "not zero");
}

py_class!(class Sequence |py| {
    def __len__(&self) -> PyResult<usize> {
        Ok(5)
    }

    def __getitem__(&self, key: PyObject) -> PyResult<PyObject> {
        if let Ok(index) = key.extract::<i32>(py) {
            if index == 5 {
                return Err(PyErr::new::<exc::IndexError, NoArgs>(py, NoArgs));
            }
        }
        Ok(key)
    }
});

#[test]
fn sequence() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = Sequence::create_instance(py).unwrap();
    py_assert!(py, c, "list(c) == [0, 1, 2, 3, 4]");
    py_assert!(py, c, "c['abc'] == 'abc'");
}

py_class!(class SequenceRef |py| {
    def __getitem__(&self, key: &str) -> PyResult<String> {
        if key.is_empty() {
            return Err(PyErr::new::<exc::IndexError, NoArgs>(py, NoArgs));
        }
        Ok(format!("Item for {}", key))
    }

    def __delitem__(&self, key: &str) -> PyResult<()> {
        if key.is_empty() {
            return Err(PyErr::new::<exc::IndexError, NoArgs>(py, NoArgs));
        }
        Ok(())
    }
});

#[test]
fn sequence_ref() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = SequenceRef::create_instance(py).unwrap();
    py_assert!(py, c, "c['abc'] == 'Item for abc'");
    py_run!(py, c, "del c['abc']");
    py_expect_exception!(py, c, "c['']", IndexError);
    py_expect_exception!(py, c, "c[42]", TypeError);
    py_expect_exception!(py, c, "del c['']", IndexError);
}

py_class!(class Callable |py| {
    def __call__(&self, arg: i32) -> PyResult<i32> {
        Ok(arg * 6)
    }
});

#[test]
fn callable() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = Callable::create_instance(py).unwrap();
    py_assert!(py, c, "callable(c)");
    py_assert!(py, c, "c(7) == 42");

    let nc = Comparisons::create_instance(py, 0).unwrap();
    py_assert!(py, nc, "not callable(nc)");
}

py_class!(class SetItem |py| {
    data key: Cell<i32>;
    data val: Cell<i32>;

    def __setitem__(&self, key: i32, val: i32) -> PyResult<()> {
        self.key(py).set(key);
        self.val(py).set(val);
        Ok(())
    }
});

#[test]
fn setitem() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = SetItem::create_instance(py, Cell::new(0), Cell::new(0)).unwrap();
    py_run!(py, c, "c[1] = 2");
    assert_eq!(c.key(py).get(), 1);
    assert_eq!(c.val(py).get(), 2);
    py_expect_exception!(py, c, "del c[1]", NotImplementedError);
}

py_class!(class SetItemRef |py| {
    data key: RefCell<String>;
    data val: Cell<i32>;

    def __setitem__(&self, key: &str, val: i32) -> PyResult<()> {
        *self.key(py).borrow_mut() = key.to_string();
        self.val(py).set(val);
        Ok(())
    }
});

#[test]
fn setitem_ref() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = SetItemRef::create_instance(py, RefCell::new(String::new()), Cell::new(0)).unwrap();
    py_run!(py, c, "c['foo'] = 100");
    assert_eq!(*c.key(py).borrow(), "foo");
    assert_eq!(c.val(py).get(), 100);
    py_expect_exception!(py, c, "c[None] = 1", TypeError);
}

py_class!(class SetItemOptRef |py| {
    data key: RefCell<String>;
    data val: Cell<i32>;

    def __setitem__(&self, key: Option<&str>, val: i32) -> PyResult<()> {
        if let Some(key) = key {
            *self.key(py).borrow_mut() = key.to_string();
            self.val(py).set(val);
            Ok(())
        } else {
            Err(PyErr::new::<exc::IndexError, NoArgs>(py, NoArgs))
        }
    }
});

#[test]
fn setitem_opt_ref() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = SetItemOptRef::create_instance(py, RefCell::new(String::new()), Cell::new(0)).unwrap();
    py_run!(py, c, "c['foo'] = 100");
    assert_eq!(*c.key(py).borrow(), "foo");
    assert_eq!(c.val(py).get(), 100);
    py_expect_exception!(py, c, "c[None] = 1", IndexError);
}

py_class!(class DelItem |py| {
    data key: Cell<i32>;

    def __delitem__(&self, key: i32) -> PyResult<()> {
        self.key(py).set(key);
        Ok(())
    }
});

#[test]
fn delitem() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = DelItem::create_instance(py, Cell::new(0)).unwrap();
    py_run!(py, c, "del c[1]");
    assert_eq!(c.key(py).get(), 1);
    py_expect_exception!(py, c, "c[1] = 2", NotImplementedError);
}

py_class!(class SetDelItem |py| {
    data val: Cell<Option<i32>>;

    def __setitem__(&self, key: i32, val: i32) -> PyResult<()> {
        self.val(py).set(Some(val));
        Ok(())
    }

    def __delitem__(&self, key: i32) -> PyResult<()> {
        self.val(py).set(None);
        Ok(())
    }
});

#[test]
fn setdelitem() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = SetDelItem::create_instance(py, Cell::new(None)).unwrap();
    py_run!(py, c, "c[1] = 2");
    assert_eq!(c.val(py).get(), Some(2));
    py_run!(py, c, "del c[1]");
    assert_eq!(c.val(py).get(), None);
}

py_class!(class Reversed |py| {
    def __reversed__(&self) -> PyResult<&'static str> {
        Ok("I am reversed")
    }
});

#[test]
fn reversed() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = Reversed::create_instance(py).unwrap();
    py_run!(py, c, "assert reversed(c) == 'I am reversed'");
}

py_class!(class Contains |py| {
    def __contains__(&self, item: i32) -> PyResult<bool> {
        Ok(item >= 0)
    }
});

#[test]
fn contains() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = Contains::create_instance(py).unwrap();
    py_run!(py, c, "assert 1 in c");
    py_run!(py, c, "assert -1 not in c");
    py_run!(py, c, "assert 'wrong type' not in c");
}

py_class!(class ContainsRef |py| {
    def __contains__(&self, item: &str) -> PyResult<bool> {
        Ok(item.is_empty())
    }
});

#[test]
fn contains_ref() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = ContainsRef::create_instance(py).unwrap();
    py_run!(py, c, "assert '' in c");
    py_run!(py, c, "assert 'hello' not in c");
    py_run!(py, c, "assert 42 not in c");
}

py_class!(class ContainsOptRef |py| {
    def __contains__(&self, item: Option<&str>) -> PyResult<bool> {
        Ok(item.map_or(false, str::is_empty))
    }
});

#[test]
fn contains_opt_ref() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = ContainsOptRef::create_instance(py).unwrap();
    py_run!(py, c, "assert '' in c");
    py_run!(py, c, "assert 'hello' not in c");
    py_run!(py, c, "assert 42 not in c");
    py_run!(py, c, "assert None not in c");
}

py_class!(class UnaryArithmetic |py| {
    def __neg__(&self) -> PyResult<&'static str> {
        Ok("neg")
    }

    def __pos__(&self) -> PyResult<&'static str> {
        Ok("pos")
    }

    def __abs__(&self) -> PyResult<&'static str> {
        Ok("abs")
    }

    def __invert__(&self) -> PyResult<&'static str> {
        Ok("invert")
    }
});

#[test]
fn unary_arithmetic() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = UnaryArithmetic::create_instance(py).unwrap();
    py_run!(py, c, "assert -c == 'neg'");
    py_run!(py, c, "assert +c == 'pos'");
    py_run!(py, c, "assert abs(c) == 'abs'");
    py_run!(py, c, "assert ~c == 'invert'");
}

py_class!(class BinaryArithmetic |py| {
    def __repr__(&self) -> PyResult<&'static str> {
        Ok("BA")
    }

    def __add__(lhs, rhs) -> PyResult<String> {
        Ok(format!("{:?} + {:?}", lhs, rhs))
    }

    def __sub__(lhs, rhs) -> PyResult<String> {
        Ok(format!("{:?} - {:?}", lhs, rhs))
    }

    def __mul__(lhs, rhs) -> PyResult<String> {
        Ok(format!("{:?} * {:?}", lhs, rhs))
    }

    def __lshift__(lhs, rhs) -> PyResult<String> {
        Ok(format!("{:?} << {:?}", lhs, rhs))
    }

    def __rshift__(lhs, rhs) -> PyResult<String> {
        Ok(format!("{:?} >> {:?}", lhs, rhs))
    }

    def __and__(lhs, rhs) -> PyResult<String> {
        Ok(format!("{:?} & {:?}", lhs, rhs))
    }

    def __xor__(lhs, rhs) -> PyResult<String> {
        Ok(format!("{:?} ^ {:?}", lhs, rhs))
    }

    def __or__(lhs, rhs) -> PyResult<String> {
        Ok(format!("{:?} | {:?}", lhs, rhs))
    }
});

#[test]
fn binary_arithmetic() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = BinaryArithmetic::create_instance(py).unwrap();
    py_run!(py, c, "assert c + c == 'BA + BA'");
    py_run!(py, c, "assert c + 1 == 'BA + 1'");
    py_run!(py, c, "assert 1 + c == '1 + BA'");
    py_run!(py, c, "assert c - 1 == 'BA - 1'");
    py_run!(py, c, "assert 1 - c == '1 - BA'");
    py_run!(py, c, "assert c * 1 == 'BA * 1'");
    py_run!(py, c, "assert 1 * c == '1 * BA'");

    py_run!(py, c, "assert c << 1 == 'BA << 1'");
    py_run!(py, c, "assert 1 << c == '1 << BA'");
    py_run!(py, c, "assert c >> 1 == 'BA >> 1'");
    py_run!(py, c, "assert 1 >> c == '1 >> BA'");
    py_run!(py, c, "assert c & 1 == 'BA & 1'");
    py_run!(py, c, "assert 1 & c == '1 & BA'");
    py_run!(py, c, "assert c ^ 1 == 'BA ^ 1'");
    py_run!(py, c, "assert 1 ^ c == '1 ^ BA'");
    py_run!(py, c, "assert c | 1 == 'BA | 1'");
    py_run!(py, c, "assert 1 | c == '1 | BA'");
}

py_class!(class RichComparisons |py| {
    def __repr__(&self) -> PyResult<&'static str> {
        Ok("RC")
    }

    def __richcmp__(&self, other: &PyObject, op: CompareOp) -> PyResult<String> {
        match op {
            CompareOp::Lt => Ok(format!("{:?} < {:?}", self.as_object(), other)),
            CompareOp::Le => Ok(format!("{:?} <= {:?}", self.as_object(), other)),
            CompareOp::Eq => Ok(format!("{:?} == {:?}", self.as_object(), other)),
            CompareOp::Ne => Ok(format!("{:?} != {:?}", self.as_object(), other)),
            CompareOp::Gt => Ok(format!("{:?} > {:?}", self.as_object(), other)),
            CompareOp::Ge => Ok(format!("{:?} >= {:?}", self.as_object(), other))
        }
    }
});

py_class!(class RichComparisons2 |py| {
    def __repr__(&self) -> PyResult<&'static str> {
        Ok("RC2")
    }

    def __richcmp__(&self, other: &PyObject, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => Ok(true.to_py_object(py).into_object()),
            CompareOp::Ne => Ok(false.to_py_object(py).into_object()),
            _ => Ok(py.NotImplemented())
        }
    }
});

#[test]
fn rich_comparisons() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = RichComparisons::create_instance(py).unwrap();
    py_run!(py, c, "assert (c < c) == 'RC < RC'");
    py_run!(py, c, "assert (c < 1) == 'RC < 1'");
    py_run!(py, c, "assert (1 < c) == 'RC > 1'");
    py_run!(py, c, "assert (c <= c) == 'RC <= RC'");
    py_run!(py, c, "assert (c <= 1) == 'RC <= 1'");
    py_run!(py, c, "assert (1 <= c) == 'RC >= 1'");
    py_run!(py, c, "assert (c == c) == 'RC == RC'");
    py_run!(py, c, "assert (c == 1) == 'RC == 1'");
    py_run!(py, c, "assert (1 == c) == 'RC == 1'");
    py_run!(py, c, "assert (c != c) == 'RC != RC'");
    py_run!(py, c, "assert (c != 1) == 'RC != 1'");
    py_run!(py, c, "assert (1 != c) == 'RC != 1'");
    py_run!(py, c, "assert (c > c) == 'RC > RC'");
    py_run!(py, c, "assert (c > 1) == 'RC > 1'");
    py_run!(py, c, "assert (1 > c) == 'RC < 1'");
    py_run!(py, c, "assert (c >= c) == 'RC >= RC'");
    py_run!(py, c, "assert (c >= 1) == 'RC >= 1'");
    py_run!(py, c, "assert (1 >= c) == 'RC <= 1'");
}

#[test]
#[cfg(feature = "python3-sys")]
fn rich_comparisons_python_3_type_error() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c2 = RichComparisons2::create_instance(py).unwrap();
    py_expect_exception!(py, c2, "c2 < c2", TypeError);
    py_expect_exception!(py, c2, "c2 < 1", TypeError);
    py_expect_exception!(py, c2, "1 < c2", TypeError);
    py_expect_exception!(py, c2, "c2 <= c2", TypeError);
    py_expect_exception!(py, c2, "c2 <= 1", TypeError);
    py_expect_exception!(py, c2, "1 <= c2", TypeError);
    py_run!(py, c2, "assert (c2 == c2) == True");
    py_run!(py, c2, "assert (c2 == 1) == True");
    py_run!(py, c2, "assert (1 == c2) == True");
    py_run!(py, c2, "assert (c2 != c2) == False");
    py_run!(py, c2, "assert (c2 != 1) == False");
    py_run!(py, c2, "assert (1 != c2) == False");
    py_expect_exception!(py, c2, "c2 > c2", TypeError);
    py_expect_exception!(py, c2, "c2 > 1", TypeError);
    py_expect_exception!(py, c2, "1 > c2", TypeError);
    py_expect_exception!(py, c2, "c2 >= c2", TypeError);
    py_expect_exception!(py, c2, "c2 >= 1", TypeError);
    py_expect_exception!(py, c2, "1 >= c2", TypeError);
}

py_class!(class RichComparisonsRef |py| {
    def __repr__(&self) -> PyResult<&'static str> {
        Ok("RCR")
    }

    def __richcmp__(&self, other: &str, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok("RCR" < other),
            CompareOp::Le => Ok("RCR" <= other),
            CompareOp::Eq => Ok("RCR" == other),
            CompareOp::Ne => Ok("RCR" != other),
            CompareOp::Gt => Ok("RCR" > other),
            CompareOp::Ge => Ok("RCR" >= other),
        }
    }
});

#[test]
fn rich_comparisons_ref() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = RichComparisonsRef::create_instance(py).unwrap();
    py_assert!(py, c, "c < 'ZZZ'");
    py_assert!(py, c, "c <= 'ZZZ'");
    py_assert!(py, c, "c <= 'RCR'");
    py_assert!(py, c, "c == 'RCR'");
    py_assert!(py, c, "c != 'RRR'");
    py_assert!(py, c, "c > 'AAA'");
    py_assert!(py, c, "c >= 'AAA'");
    py_assert!(py, c, "c >= 'RCR'");
}

py_class!(class RichComparisonsOptRef |py| {
    def __repr__(&self) -> PyResult<&'static str> {
        Ok("RCR")
    }

    def __richcmp__(&self, other: Option<&str>, op: CompareOp) -> PyResult<bool> {
        match other {
            Some(other) => match op {
                CompareOp::Lt => Ok("RCR" < other),
                CompareOp::Le => Ok("RCR" <= other),
                CompareOp::Eq => Ok("RCR" == other),
                CompareOp::Ne => Ok("RCR" != other),
                CompareOp::Gt => Ok("RCR" > other),
                CompareOp::Ge => Ok("RCR" >= other),
            },
            None => match op {
                CompareOp::Ne | CompareOp::Gt | CompareOp::Ge => Ok(true),
                CompareOp::Eq | CompareOp::Lt | CompareOp::Le => Ok(false),
            },
        }
    }
});

#[test]
fn rich_comparisons_opt_ref() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = RichComparisonsOptRef::create_instance(py).unwrap();
    py_assert!(py, c, "c != None");
    py_assert!(py, c, "c == 'RCR'");
    py_assert!(py, c, "c > None");
    py_assert!(py, c, "None <= c");
}

py_class!(class InPlaceOperations |py| {
    data value: Cell<u32>;

    def __repr__(&self) -> PyResult<String> {
        Ok(format!("IPO({:?})", self.value(py).get()))
    }

    def __iadd__(&self, other: u32) -> PyResult<Self> {
        self.value(py).set(self.value(py).get() + other);
        Ok(self.clone_ref(py))
    }

    def __isub__(&self, other: u32) -> PyResult<Self> {
        self.value(py).set(self.value(py).get() - other);
        Ok(self.clone_ref(py))
    }

    def __imul__(&self, other: u32) -> PyResult<Self> {
        self.value(py).set(self.value(py).get() * other);
        Ok(self.clone_ref(py))
    }

    def __ilshift__(&self, other: u32) -> PyResult<Self> {
        self.value(py).set(self.value(py).get() << other);
        Ok(self.clone_ref(py))
    }

    def __irshift__(&self, other: u32) -> PyResult<Self> {
        self.value(py).set(self.value(py).get() >> other);
        Ok(self.clone_ref(py))
    }

    def __iand__(&self, other: u32) -> PyResult<Self> {
        self.value(py).set(self.value(py).get() & other);
        Ok(self.clone_ref(py))
    }

    def __ixor__(&self, other: u32) -> PyResult<Self> {
        self.value(py).set(self.value(py).get() ^ other);
        Ok(self.clone_ref(py))
    }

    def __ior__(&self, other: u32) -> PyResult<Self> {
        self.value(py).set(self.value(py).get() | other);
        Ok(self.clone_ref(py))
    }
});

#[test]
fn inplace_operations() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = InPlaceOperations::create_instance(py, Cell::new(0)).unwrap();
    py_run!(
        py,
        c,
        "d = c; c += 1; assert repr(c) == repr(d) == 'IPO(1)'"
    );

    let c = InPlaceOperations::create_instance(py, Cell::new(10)).unwrap();
    py_run!(
        py,
        c,
        "d = c; c -= 1; assert repr(c) == repr(d) == 'IPO(9)'"
    );

    let c = InPlaceOperations::create_instance(py, Cell::new(3)).unwrap();
    py_run!(
        py,
        c,
        "d = c; c *= 3; assert repr(c) == repr(d) == 'IPO(9)'"
    );

    let c = InPlaceOperations::create_instance(py, Cell::new(3)).unwrap();
    py_run!(
        py,
        c,
        "d = c; c <<= 2; assert repr(c) == repr(d) == 'IPO(12)'"
    );

    let c = InPlaceOperations::create_instance(py, Cell::new(12)).unwrap();
    py_run!(
        py,
        c,
        "d = c; c >>= 2; assert repr(c) == repr(d) == 'IPO(3)'"
    );

    let c = InPlaceOperations::create_instance(py, Cell::new(12)).unwrap();
    py_run!(
        py,
        c,
        "d = c; c &= 10; assert repr(c) == repr(d) == 'IPO(8)'"
    );

    let c = InPlaceOperations::create_instance(py, Cell::new(12)).unwrap();
    py_run!(
        py,
        c,
        "d = c; c |= 3; assert repr(c) == repr(d) == 'IPO(15)'"
    );

    let c = InPlaceOperations::create_instance(py, Cell::new(12)).unwrap();
    py_run!(
        py,
        c,
        "d = c; c ^= 5; assert repr(c) == repr(d) == 'IPO(9)'"
    );
}

py_class!(class InPlaceOperationsRef |py| {
    data value: RefCell<String>;

    def __repr__(&self) -> PyResult<String> {
        Ok(format!("IPOR({:?})", self.value(py).borrow()))
    }

    def __iadd__(&self, other: &str) -> PyResult<Self> {
        let mut value = self.value(py).borrow_mut();
        value.push_str(other);
        Ok(self.clone_ref(py))
    }
});

#[test]
fn inplace_operations_ref() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = InPlaceOperationsRef::create_instance(py, RefCell::new(String::new())).unwrap();
    py_run!(
        py,
        c,
        r#"d = c; c += "hello"; c += ", world"; assert repr(c) == repr(d) == 'IPOR("hello, world")'"#
    );
}

py_class!(class ContextManager |py| {
    data exit_called : Cell<bool>;

    def __enter__(&self) -> PyResult<i32> {
        Ok(42)
    }

    def __exit__(&self, ty: Option<PyType>, value: PyObject, traceback: PyObject) -> PyResult<bool> {
        self.exit_called(py).set(true);
        if ty == Some(py.get_type::<exc::ValueError>()) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
});

#[test]
fn context_manager() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = ContextManager::create_instance(py, Cell::new(false)).unwrap();
    py_run!(py, c, "with c as x:\n  assert x == 42");
    assert!(c.exit_called(py).get());

    c.exit_called(py).set(false);
    py_run!(py, c, "with c as x:\n  raise ValueError");
    assert!(c.exit_called(py).get());

    c.exit_called(py).set(false);
    py_expect_exception!(
        py,
        c,
        "with c as x:\n  raise NotImplementedError",
        NotImplementedError
    );
    assert!(c.exit_called(py).get());
}

py_class!(class Properties |py| {
    data value: Cell<i32>;
    data value_by_ref: RefCell<String>;
    data value_by_opt_ref: RefCell<String>;

    def __repr__(&self) -> PyResult<String> {
        Ok(format!("P({:?} {:?} {:?})",
            self.value(py).get(),
            self.value_by_ref(py).borrow(),
            self.value_by_opt_ref(py).borrow()))
    }

    @property def prop(&self) -> PyResult<i32> {
        Ok(self.value(py).get())
    }

    @prop.setter def set_prop(&self, value: Option<i32>) -> PyResult<()> {
        self.value(py).set(value.unwrap_or(0));
        Ok(())
    }

    @property def prop_by_ref(&self) -> PyResult<String> {
        Ok(self.value_by_ref(py).borrow().to_string())
    }

    @prop_by_ref.setter def set_prop_by_ref(&self, value: Option<&str>) -> PyResult<()> {
        *self.value_by_ref(py).borrow_mut() = value.unwrap_or("DELETED").to_string();
        Ok(())
    }

    @property def prop_by_opt_ref(&self) -> PyResult<String> {
        Ok(self.value_by_opt_ref(py).borrow().to_string())
    }

    @prop_by_opt_ref.setter def set_prop_by_opt_ref(&self, value: Option<Option<&str>>) -> PyResult<()> {
        let value = value.unwrap_or(Some("DELETED")).unwrap_or("NO VALUE");
        *self.value_by_opt_ref(py).borrow_mut() = value.to_string();
        Ok(())
    }

    /// docs for match
    @property def r#match(&self) -> PyResult<bool> {
        Ok(self.value(py).get() != 0)
    }
});

#[test]
fn properties() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let c = Properties::create_instance(
        py,
        Cell::new(0),
        RefCell::new(String::new()),
        RefCell::new(String::new()),
    )
    .unwrap();

    py_run!(
        py,
        c,
        "assert 'docs for match' in c.__class__.match.__doc__"
    );

    py_run!(py, c, "assert c.prop == 0");
    py_run!(py, c, "assert not c.match");
    py_run!(py, c, "c.prop = 42");
    assert_eq!(c.value(py).get(), 42);
    py_run!(py, c, "assert c.match");
    assert!(c.r#match(py).unwrap());

    // Instead of really deleting, our setter sets back to 0
    py_run!(py, c, "delattr(c, 'prop')");
    py_run!(py, c, "assert c.prop == 0");

    py_run!(py, c, "c.prop_by_ref = 'testing'");
    py_run!(py, c, "assert c.prop_by_ref == 'testing'");

    py_run!(py, c, "c.prop_by_opt_ref = 'something'");
    assert_eq!(*c.value_by_opt_ref(py).borrow(), "something");
    py_run!(py, c, "c.prop_by_opt_ref = None");
    py_run!(py, c, "repr(c) == 'P(42, \"testing\" \"NO VALUE\")'");

    py_run!(py, c, "del c.prop_by_opt_ref");
    py_run!(py, c, "repr(c) == 'P(42, \"testing\" \"DELETED\")'");
}

#[cfg(feature = "python3-sys")]
mod py3_only {
    use self::RefCountLog::*;
    use super::*;
    use std::sync::Mutex;

    #[derive(PartialEq, Debug)]
    enum RefCountLog {
        Plus,
        Minus,
    }

    struct SharedBuffer {
        buf: Vec<u8>,
        log: Mutex<Vec<RefCountLog>>,
    }

    impl SharedBuffer {
        fn new(buf: Vec<u8>) -> Arc<Self> {
            Arc::new(SharedBuffer {
                buf,
                log: Mutex::new(vec![]),
            })
        }
    }

    struct TestBuffer {
        ptr: Arc<SharedBuffer>,
        count_drop: bool,
    }

    impl TestBuffer {
        fn new(ptr: &Arc<SharedBuffer>) -> Self {
            ptr.log.lock().unwrap().push(RefCountLog::Plus);
            Self::from_arc(ptr.clone())
        }
        fn into_arc(mut self) -> Arc<SharedBuffer> {
            self.count_drop = false;
            self.ptr.clone()
        }
        fn from_arc(ptr: Arc<SharedBuffer>) -> Self {
            Self {
                ptr: ptr.clone(),
                count_drop: true,
            }
        }
    }
    impl Drop for TestBuffer {
        fn drop(&mut self) {
            if self.count_drop {
                self.ptr.log.lock().unwrap().push(RefCountLog::Minus);
            }
        }
    }

    unsafe impl buffer::BufferHandle for TestBuffer {
        fn as_bytes(&self) -> &[u8] {
            &self.ptr.buf
        }
        fn into_owned_void_pointer(self) -> *mut libc::c_void {
            let raw = Arc::into_raw(self.into_arc());
            assert_eq!(
                std::mem::size_of::<*mut libc::c_void>(),
                std::mem::size_of_val(&raw)
            );
            raw as *const libc::c_void as *mut libc::c_void
        }
        unsafe fn from_owned_void_pointer(ptr: *mut libc::c_void) -> Self {
            TestBuffer::from_arc(Arc::from_raw(ptr as *const libc::c_void as *const _))
        }
    }

    py_class!(class BufferProtocol |py| {
        data ptr: Arc<SharedBuffer>;

        def __buffer__(&self) -> PyResult<TestBuffer> {
            Ok(TestBuffer::new(self.ptr(py)))
        }
    });

    #[test]
    fn test_buffer() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let buf = SharedBuffer::new(vec![1, 2, 42]);
        let log_access = buf.clone();

        let c = BufferProtocol::create_instance(py, buf).unwrap();

        macro_rules! check_log {
            ($($e:expr),+; $($r:expr),+) => {
                py_run!(py, c, &[$($e),+].join("\n")[..]);
                {
                    let should_log = &[$($r),+];
                    let mut log = log_access.log.lock().unwrap();
                    assert_eq!(&**log, should_log);
                    log.clear();
                }
            }
        }

        check_log!("memoryview(c)"; Plus, Minus);
        check_log!("assert memoryview(c).readonly"; Plus, Minus);
        check_log!("assert len(memoryview(c)) == 3"; Plus, Minus);
        check_log!("assert memoryview(c)[0] == 1"; Plus, Minus);
        check_log!("assert memoryview(c)[2] == 42"; Plus, Minus);
        check_log!("assert list(memoryview(c)) == [1, 2, 42]"; Plus, Minus);
        check_log!(
            "a = memoryview(c)",
            "b = memoryview(c)",
            "assert a == b";
            Plus, Plus, Minus, Minus);
        check_log!(
            "a = memoryview(c)",
            "b = memoryview(a)",
            "assert a == b";
            Plus, Minus);
        check_log!(
            "def foo(x):",
            "  a = memoryview(x)",
            "  assert a[1] == 2",
            "foo(c)",
            "foo(c)";
            Plus, Minus, Plus, Minus);
        check_log!(
            "def foo(x):",
            "  a = memoryview(x)",
            "  assert a[1] == 2",
            "foo(c)",
            "e = memoryview(c)",
            "f = memoryview(c)",
            "assert e[2] == f[2] == 42";
            Plus, Minus, Plus, Plus, Minus, Minus);

        check_log!(
            "with memoryview(c):",
            "  pass",
            "with memoryview(c):",
            "  pass";
            Plus, Minus, Plus, Minus);

        py_expect_exception!(py, c, "memoryview(c)[3]", IndexError);
        py_expect_exception!(py, c, "memoryview(c)[2] = 413", TypeError);
    }

    py_class!(class DirectBufferProtocol |py| {
        data ptr: Arc<Vec<u8>>;

        def __direct_buffer__<'a>(&'a self) -> PyResult<&'a [u8]> {
            Ok(self.ptr(py))
        }
    });

    #[test]
    fn test_direct_buffer() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let buf = Arc::new(vec![1, 2, 42]);

        let c = DirectBufferProtocol::create_instance(py, buf).unwrap();

        macro_rules! check_log {
            ($($e:expr),+) => {
                py_run!(py, c, &[$($e),+].join("\n")[..]);
            }
        }

        check_log!("memoryview(c)");
        check_log!("assert memoryview(c).readonly");
        check_log!("assert len(memoryview(c)) == 3");
        check_log!("assert memoryview(c)[0] == 1");
        check_log!("assert memoryview(c)[2] == 42");
        check_log!("assert list(memoryview(c)) == [1, 2, 42]");
        check_log!("a = memoryview(c)", "b = memoryview(c)", "assert a == b");
        check_log!("a = memoryview(c)", "b = memoryview(a)", "assert a == b");
        check_log!(
            "def foo(x):",
            "  a = memoryview(x)",
            "  assert a[1] == 2",
            "foo(c)",
            "foo(c)"
        );
        check_log!(
            "def foo(x):",
            "  a = memoryview(x)",
            "  assert a[1] == 2",
            "foo(c)",
            "e = memoryview(c)",
            "f = memoryview(c)",
            "assert e[2] == f[2] == 42"
        );

        check_log!(
            "with memoryview(c):",
            "  pass",
            "with memoryview(c):",
            "  pass"
        );

        py_expect_exception!(py, c, "memoryview(c)[3]", IndexError);
        py_expect_exception!(py, c, "memoryview(c)[2] = 413", TypeError);
    }
}

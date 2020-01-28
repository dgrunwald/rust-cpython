use cpython::{py_class, GILGuard, Python};

py_class!(class Owner |py| {
    @shared data string: String;
});

fn prepare_env() -> (GILGuard, Owner) {
    let gil = Python::acquire_gil();
    let owner = {
        let py = gil.python();
        Owner::create_instance(py, "new".to_owned()).unwrap()
    };
    (gil, owner)
}

#[test]
fn test_leaked_borrow() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let leaked = owner.string(py).leak_immutable();
    let leaked_ref = unsafe { leaked.try_borrow(py) }.unwrap();
    assert_eq!(*leaked_ref, "new");
}

#[test]
fn test_leaked_borrow_mut() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let leaked = owner.string(py).leak_immutable();
    let mut leaked_iter = unsafe { leaked.map(py, |s| s.chars()) };
    let mut leaked_ref = unsafe { leaked_iter.try_borrow_mut(py) }.unwrap();
    assert_eq!(leaked_ref.next(), Some('n'));
    assert_eq!(leaked_ref.next(), Some('e'));
    assert_eq!(leaked_ref.next(), Some('w'));
    assert_eq!(leaked_ref.next(), None);
}

#[test]
fn test_leaked_borrow_after_mut() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let leaked = owner.string(py).leak_immutable();
    owner.string(py).borrow_mut().clear();
    assert!(unsafe { leaked.try_borrow(py) }.is_err());
}

#[test]
fn test_leaked_borrow_mut_after_mut() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let leaked = owner.string(py).leak_immutable();
    let mut leaked_iter = unsafe { leaked.map(py, |s| s.chars()) };
    owner.string(py).borrow_mut().clear();
    assert!(unsafe { leaked_iter.try_borrow_mut(py) }.is_err());
}

#[test]
#[should_panic(expected = "map() over invalidated leaked reference")]
fn test_leaked_map_after_mut() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let leaked = owner.string(py).leak_immutable();
    owner.string(py).borrow_mut().clear();
    let _leaked_iter = unsafe { leaked.map(py, |s| s.chars()) };
}

#[test]
fn test_try_borrow_mut_while_leaked_ref() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    assert!(owner.string(py).try_borrow_mut().is_ok());
    let leaked = owner.string(py).leak_immutable();
    {
        let _leaked_ref = unsafe { leaked.try_borrow(py) }.unwrap();
        assert!(owner.string(py).try_borrow_mut().is_err());
        {
            let _leaked_ref2 = unsafe { leaked.try_borrow(py) }.unwrap();
            assert!(owner.string(py).try_borrow_mut().is_err());
        }
        assert!(owner.string(py).try_borrow_mut().is_err());
    }
    assert!(owner.string(py).try_borrow_mut().is_ok());
}

#[test]
fn test_try_borrow_mut_while_leaked_ref_mut() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    assert!(owner.string(py).try_borrow_mut().is_ok());
    let leaked = owner.string(py).leak_immutable();
    let mut leaked_iter = unsafe { leaked.map(py, |s| s.chars()) };
    {
        let _leaked_ref = unsafe { leaked_iter.try_borrow_mut(py) }.unwrap();
        assert!(owner.string(py).try_borrow_mut().is_err());
    }
    assert!(owner.string(py).try_borrow_mut().is_ok());
}

#[test]
fn test_try_leak_while_borrow_mut() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let _mut_ref = owner.string(py).borrow_mut();
    assert!(owner.string(py).try_leak_immutable().is_err());
}

#[test]
#[should_panic(expected = "already mutably borrowed")]
fn test_leak_while_borrow_mut() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let _mut_ref = owner.string(py).borrow_mut();
    owner.string(py).leak_immutable();
}

#[test]
fn test_try_borrow_mut_while_borrow() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let _ref = owner.string(py).borrow();
    assert!(owner.string(py).try_borrow_mut().is_err());
}

#[test]
#[should_panic(expected = "already borrowed")]
fn test_borrow_mut_while_borrow() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let _ref = owner.string(py).borrow();
    owner.string(py).borrow_mut();
}

#[test]
fn test_try_borrow_while_borrow_mut() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let _mut_ref = owner.string(py).borrow_mut();
    assert!(owner.string(py).try_borrow().is_err());
}

#[test]
#[should_panic(expected = "already mutably borrowed")]
fn test_borrow_while_borrow_mut() {
    let (gil, owner) = prepare_env();
    let py = gil.python();
    let _mut_ref = owner.string(py).borrow_mut();
    owner.string(py).borrow();
}

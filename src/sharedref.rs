// Copyright (c) 2019 Raphaël Gomès <rgomes@octobus.net>,
//                    Yuya Nishihara <yuya@tcha.org>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this
// software and associated documentation files (the "Software"), to deal in the Software
// without restriction, including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons
// to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
// PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
// FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Utility to share Rust reference across Python objects.

use std::cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::result;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::err::{PyErr, PyResult};
use crate::objects::{exc, PyObject};
use crate::python::{PyClone, Python};

/// A mutable memory location shareable immutably across Python objects.
///
/// This is a [RefCell] that can also be borrowed immutably by another Python
/// object.
///
/// The primary use case is to implement a Python iterator over a Rust
/// iterator. Since a Python object cannot hold a lifetime-bound object,
/// `Iter<'a, T>` cannot be a data field of the Python iterator object.
/// [PySharedRef::leak_immutable()] provides a way around this issue.
///
/// ```
/// # use cpython::*;
/// # use std::cell::RefCell;
/// # use std::slice::Iter;
/// py_class!(pub class List |py| {
///     @shared data rust_vec: Vec<i32>;
///
///     def __iter__(&self) -> PyResult<ListIterator> {
///         let leaked = self.rust_vec(py).leak_immutable();
///         ListIterator::create_instance(
///             py,
///             RefCell::new(unsafe { leaked.map(py, |o| o.iter()) }),
///         )
///     }
/// });
///
/// py_class!(pub class ListIterator |py| {
///     data rust_iter: RefCell<UnsafePyLeaked<Iter<'static, i32>>>;
///
///     def __next__(&self) -> PyResult<Option<PyInt>> {
///         let mut leaked = self.rust_iter(py).borrow_mut();
///         let mut iter = unsafe { leaked.try_borrow_mut(py)? };
///         Ok(iter.next().map(|v| v.to_py_object(py)))
///     }
///
///     def __iter__(&self) -> PyResult<Self> {
///         Ok(self.clone_ref(py))
///     }
/// });
/// ```
///
/// The borrow rules are enforced dynamically in a similar manner to the
/// Python iterator.
///
/// `PySharedRefCell` is merely a data struct to be stored in a Python object.
/// Any further operation will be performed through [PySharedRef], which is
/// a lifetime-bound reference to the `PySharedRefCell`.
///
/// [RefCell]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
/// [PySharedRef]: struct.PySharedRef.html
/// [PySharedRef::leak_immutable()]: struct.PySharedRef.html#method.leak_immutable
#[derive(Debug)]
pub struct PySharedRefCell<T: ?Sized> {
    state: PySharedState,
    data: RefCell<T>,
}

impl<T> PySharedRefCell<T> {
    /// Creates a new `PySharedRefCell` containing `value`.
    // can be "const fn" since Rust 1.31.0
    pub fn new(value: T) -> PySharedRefCell<T> {
        Self {
            state: PySharedState::new(),
            data: RefCell::new(value),
        }
    }
}

/// A reference to `PySharedRefCell` owned by a Python object.
///
/// This is a lifetime-bound reference to the [PySharedRefCell] data field,
/// and will be created by the automatically generated accessor function.
///
/// ```ignore
/// impl MyType {
///     fn data_name<'a>(&'a self, py: Python<'a>) -> PySharedRef<'a, T> { ... }
/// }
/// ```
///
/// [PySharedRefCell]: struct.PySharedRefCell.html
pub struct PySharedRef<'a, T: 'a + ?Sized> {
    py: Python<'a>,
    owner: &'a PyObject,
    state: &'a PySharedState,
    data: &'a RefCell<T>,
}

impl<'a, T: ?Sized> PySharedRef<'a, T> {
    /// Creates a reference to the given `PySharedRefCell` owned by the
    /// given `PyObject`.
    ///
    /// # Safety
    ///
    /// The `data` must be owned by the `owner`. Otherwise, `leak_immutable()`
    /// would create an invalid reference.
    #[doc(hidden)]
    pub unsafe fn new(py: Python<'a>, owner: &'a PyObject, data: &'a PySharedRefCell<T>) -> Self {
        Self {
            py,
            owner,
            state: &data.state,
            data: &data.data,
        }
    }

    /// Immutably borrows the wrapped value.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed.
    pub fn borrow(&self) -> Ref<'a, T> {
        self.try_borrow().expect("already mutably borrowed")
    }

    /// Immutably borrows the wrapped value, returning an error if the value
    /// is currently mutably borrowed.
    pub fn try_borrow(&self) -> result::Result<Ref<'a, T>, BorrowError> {
        // state isn't involved since
        // - data.try_borrow() would fail if self is mutably borrowed,
        // - and data.try_borrow_mut() would fail while self is borrowed.
        self.data.try_borrow()
    }

    /// Mutably borrows the wrapped value.
    ///
    /// Any existing leaked references will be invalidated.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently borrowed.
    pub fn borrow_mut(&self) -> RefMut<'a, T> {
        self.try_borrow_mut().expect("already borrowed")
    }

    /// Mutably borrows the wrapped value, returning an error if the value
    /// is currently borrowed.
    pub fn try_borrow_mut(&self) -> result::Result<RefMut<'a, T>, BorrowMutError> {
        // the value may be immutably borrowed through UnsafePyLeaked
        if self.state.current_borrow_count(self.py) > 0 {
            // propagate borrow-by-leaked state to data to get BorrowMutError
            let _dummy = self.data.borrow();
            self.data.try_borrow_mut()?;
            unreachable!("BorrowMutError must be returned");
        }

        let data_ref = self.data.try_borrow_mut()?;
        self.state.increment_generation(self.py);
        Ok(data_ref)
    }

    /// Creates an immutable reference which is not bound to lifetime.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed.
    pub fn leak_immutable(&self) -> UnsafePyLeaked<&'static T> {
        self.try_leak_immutable().expect("already mutably borrowed")
    }

    /// Creates an immutable reference which is not bound to lifetime,
    /// returning an error if the value is currently mutably borrowed.
    pub fn try_leak_immutable(&self) -> result::Result<UnsafePyLeaked<&'static T>, BorrowError> {
        // make sure self.data isn't mutably borrowed; otherwise the
        // generation number wouldn't be trusted.
        let data_ref = self.try_borrow()?;

        // keep reference to the owner so the data and state are alive,
        // but the data pointer can be invalidated by borrow_mut().
        // the state wouldn't since it is immutable.
        let state_ptr: *const PySharedState = self.state;
        let data_ptr: *const T = &*data_ref;
        Ok(UnsafePyLeaked::<&'static T> {
            owner: self.owner.clone_ref(self.py),
            state: unsafe { &*state_ptr },
            generation: self.state.current_generation(self.py),
            data: unsafe { &*data_ptr },
        })
    }
}

/// The shared state between Python and Rust
///
/// `PySharedState` is owned by `PySharedRefCell`, and is shared across its
/// derived references. The consistency of these references are guaranteed
/// as follows:
///
/// - The immutability of `py_class!` object fields. Any mutation of
///   `PySharedRefCell` is allowed only through its `borrow_mut()`.
/// - The `py: Python<'_>` token, which makes sure that any data access is
///   synchronized by the GIL.
/// - The underlying `RefCell`, which prevents `PySharedRefCell` value from
///   being directly borrowed or leaked while it is mutably borrowed.
/// - The `borrow_count`, which is the number of references borrowed from
///   `UnsafePyLeaked`. Just like `RefCell`, mutation is prohibited while
///   `UnsafePyLeaked` is borrowed.
/// - The `generation` counter, which increments on `borrow_mut()`.
///   `UnsafePyLeaked` reference is valid only if the `current_generation()`
///   equals to the `generation` at the time of `leak_immutable()`.
#[derive(Debug)]
struct PySharedState {
    // The counter variable could be Cell<usize> since any operation on
    // PySharedState is synchronized by the GIL, but being "atomic" makes
    // PySharedState inherently Sync. The ordering requirement doesn't
    // matter thanks to the GIL. That's why Ordering::Relaxed is used
    // everywhere.
    /// The number of immutable references borrowed through leaked reference.
    borrow_count: AtomicUsize,
    /// The mutation counter of the underlying value.
    generation: AtomicUsize,
}

impl PySharedState {
    // can be "const fn" since Rust 1.31.0
    fn new() -> PySharedState {
        PySharedState {
            borrow_count: AtomicUsize::new(0),
            generation: AtomicUsize::new(0),
        }
    }

    fn current_borrow_count(&self, _py: Python) -> usize {
        self.borrow_count.load(Ordering::Relaxed)
    }

    fn increase_borrow_count(&self, _py: Python) {
        // this wraps around if there are more than usize::MAX borrowed
        // references, which shouldn't happen due to memory limit.
        self.borrow_count.fetch_add(1, Ordering::Relaxed);
    }

    fn decrease_borrow_count(&self, _py: Python) {
        let prev_count = self.borrow_count.fetch_sub(1, Ordering::Relaxed);
        assert!(prev_count > 0);
    }

    fn current_generation(&self, _py: Python) -> usize {
        self.generation.load(Ordering::Relaxed)
    }

    fn increment_generation(&self, py: Python) {
        assert_eq!(self.current_borrow_count(py), 0);
        // this wraps around to the same value if mutably borrowed
        // usize::MAX times, which wouldn't happen in practice.
        self.generation.fetch_add(1, Ordering::Relaxed);
    }
}

/// Helper to keep the borrow count updated while the shared object is
/// immutably borrowed without using the `RefCell` interface.
struct BorrowPyShared<'a> {
    py: Python<'a>,
    state: &'a PySharedState,
}

impl<'a> BorrowPyShared<'a> {
    fn new(py: Python<'a>, state: &'a PySharedState) -> BorrowPyShared<'a> {
        state.increase_borrow_count(py);
        BorrowPyShared { py, state }
    }
}

impl<'a> Drop for BorrowPyShared<'a> {
    fn drop(&mut self) {
        self.state.decrease_borrow_count(self.py);
    }
}

/// An immutable reference to `PySharedRefCell` value, not bound to lifetime.
///
/// The reference will be invalidated once the original value is mutably
/// borrowed.
///
/// # Safety
///
/// Even though `UnsafePyLeaked` tries to enforce the real lifetime of the
/// underlying object, the object having the artificial `'static` lifetime
/// may be exposed to your Rust code. You must be careful to not make a bare
/// reference outlive the actual object lifetime.
///
/// ```ignore
/// let outer;
/// unsafe { leaked.map(py, |o| { outer = o }) };  // Bad
/// ```
///
/// ```ignore
/// let outer;
/// let mut leaked_iter = leaked.map(py, |o| o.iter());
/// {
///     let mut iter = unsafe { leaked_iter.try_borrow_mut(py) };
///     let inner = iter.next();  // Good, in borrow scope
///     outer = inner;            // Bad, &'static T may outlive
/// }
/// ```
pub struct UnsafePyLeaked<T: ?Sized> {
    owner: PyObject,
    state: &'static PySharedState,
    /// Generation counter of data `T` captured when UnsafePyLeaked is created.
    generation: usize,
    /// Underlying data of artificial lifetime, which is valid only when
    /// state.generation == self.generation.
    data: T,
}

// DO NOT implement Deref for UnsafePyLeaked<T>! Dereferencing UnsafePyLeaked
// without taking Python GIL wouldn't be safe. Also, the underling reference
// is invalid if generation != state.generation.

impl<T: ?Sized> UnsafePyLeaked<T> {
    // No panicking version of borrow() and borrow_mut() are implemented
    // because the underlying value is supposed to be mutated in Python
    // world, and the Rust library designer can't prevent it.

    // try_borrow() and try_borrow_mut() are unsafe because self.data may
    // have a function returning the inner &'static reference.
    // If T is &'static U, its lifetime can be easily coerced to &'a U, but
    // how could we do that for Whatever<'static> in general?

    /// Immutably borrows the wrapped value.
    ///
    /// Borrowing fails if the underlying reference has been invalidated.
    ///
    /// # Safety
    ///
    /// The lifetime of the innermost object is artificial. Do not obtain and
    /// copy it out of the borrow scope.
    pub unsafe fn try_borrow<'a>(&'a self, py: Python<'a>) -> PyResult<PyLeakedRef<'a, T>> {
        self.validate_generation(py)?;
        Ok(PyLeakedRef {
            _borrow: BorrowPyShared::new(py, self.state),
            data: &self.data,
        })
    }

    /// Mutably borrows the wrapped value.
    ///
    /// Borrowing fails if the underlying reference has been invalidated.
    ///
    /// Typically `T` is an iterator. If `T` is an immutable reference,
    /// `get_mut()` is useless since the inner value can't be mutated.
    ///
    /// # Safety
    ///
    /// The lifetime of the innermost object is artificial. Do not obtain and
    /// copy it out of the borrow scope.
    pub unsafe fn try_borrow_mut<'a>(
        &'a mut self,
        py: Python<'a>,
    ) -> PyResult<PyLeakedRefMut<'a, T>> {
        self.validate_generation(py)?;
        Ok(PyLeakedRefMut {
            _borrow: BorrowPyShared::new(py, self.state),
            data: &mut self.data,
        })
    }

    fn validate_generation(&self, py: Python) -> PyResult<()> {
        if self.state.current_generation(py) == self.generation {
            Ok(())
        } else {
            Err(PyErr::new::<exc::RuntimeError, _>(
                py,
                "Cannot access to leaked reference after mutation",
            ))
        }
    }
}

impl<T> UnsafePyLeaked<T> {
    /// Converts the inner value by the given function.
    ///
    /// Typically `T` is a static reference to a collection, and `U` is an
    /// iterator of that collection.
    ///
    /// # Panics
    ///
    /// Panics if the underlying reference has been invalidated.
    ///
    /// This is typically called immediately after the `UnsafePyLeaked` is
    /// obtained. At this time, the reference must be valid and no panic
    /// would occur.
    ///
    /// # Safety
    ///
    /// The lifetime of the object passed in to the function `f` is artificial.
    /// It's typically a static reference, but is valid only while the
    /// corresponding `UnsafePyLeaked` is alive. Do not copy it out of the
    /// function call.
    pub unsafe fn map<U>(self, py: Python, f: impl FnOnce(T) -> U) -> UnsafePyLeaked<U> {
        // Needs to test the generation value to make sure self.data reference
        // is still intact.
        self.validate_generation(py)
            .expect("map() over invalidated leaked reference");

        // f() could make the self.data outlive. That's why map() is unsafe.
        // In order to make this function safe, maybe we'll need a way to
        // temporarily restrict the lifetime of self.data and translate the
        // returned object back to Something<'static>.
        let new_data = f(self.data);
        UnsafePyLeaked {
            owner: self.owner,
            state: self.state,
            generation: self.generation,
            data: new_data,
        }
    }
}

/// An immutably borrowed reference to a leaked value.
pub struct PyLeakedRef<'a, T: 'a + ?Sized> {
    _borrow: BorrowPyShared<'a>,
    data: &'a T,
}

impl<'a, T: ?Sized> Deref for PyLeakedRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

/// A mutably borrowed reference to a leaked value.
pub struct PyLeakedRefMut<'a, T: 'a + ?Sized> {
    _borrow: BorrowPyShared<'a>,
    data: &'a mut T,
}

impl<'a, T: ?Sized> Deref for PyLeakedRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T: ?Sized> DerefMut for PyLeakedRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

/// A wrapper around `Rc<RefCell<T>>`
#[derive(Debug, PartialEq, Eq)]
pub struct RcCell<T> {
    value: Rc<RefCell<T>>
}

impl<T> RcCell<T> {
    /// Create a new `Rc<RefCell<T>>` from T
    pub fn new(value: T) -> Self {
        RcCell {
            value: Rc::new(RefCell::new(value))
        }
    }

    /// The count of strong references to this data
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.value)
    }

    /// Get T as ref
    pub fn as_ref(&self) -> Ref<'_, T> {
        self.value.as_ref().borrow()
    }

    /// Get T as mut ref
    pub fn as_mut(&self) -> RefMut<'_, T> {
        self.value.as_ref().borrow_mut()
    }

    /// Get T as *const T
    pub fn as_ptr(&self) -> *const T {
        RefCell::as_ptr(&self.value)
    }
}

impl<T> Clone for RcCell<T> {
    /// Clone `Rc<RefCell<T>>`
    fn clone(&self) -> Self {
        RcCell {
            value: self.value.clone()
        }
    }
}
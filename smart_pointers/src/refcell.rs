use crate::cell::MyCell;
use std::cell::UnsafeCell;

/// Possible states of a reference.
#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct MyRefCell<T> {
    value: UnsafeCell<T>,
    state: MyCell<RefState>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> MyRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: MyCell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            // get first share
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
                //return Some(unsafe { &*self.value.get() });
            }
            // increment shares
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
                //return Some(unsafe { &*self.value.get() });
            }
            RefState::Exclusive => None,
        }
    }
    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        // Don't give multiple mutable references out
        if let RefState::Unshared = self.state.get() {
            // Update state to Exclusive
            self.state.set(RefState::Exclusive);
            Some(RefMut { refcell: self })
            //return Some(unsafe { &mut *self.value.get() });
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    // lifetime because when the reference goes away this struct must go away
    refcell: &'refcell MyRefCell<T>,
}

/// deref into the inner type T.
// something like auto  arrow operator from C.
impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            // Only shared reference
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            // Decrement one
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}
pub struct RefMut<'refcell, T> {
    refcell: &'refcell MyRefCell<T>,
}

// something like auto  arrow operator from C
impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // This is given out only when we have no other references. After this set the reference to exclusive
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            // Cannot give Shared references
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            // Must be in the exclusive state
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}

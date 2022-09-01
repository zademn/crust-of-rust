#![allow(unused_variables)]

use std::cell::UnsafeCell;

pub struct MyCell<T> {
    value: UnsafeCell<T>,
}

// This is implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else is concurrently mutating self.value (because !Sync)
        // SAFETY: we know we're not invalidating any references, because we never give any out
        unsafe { *self.value.get() = value };
    }

    /// Return a *copy*, not a reference.
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is modifying this value, since only this thread can mutate
        // (because !Sync), and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::MyCell;

    #[test]
    fn bad() {
        use std::sync::Arc;
        let x = Arc::new(MyCell::new(42));
        let x1 = Arc::clone(&x);

        // This code is rejected because UnsafeCell implements !Sync already
        // std::thread::spawn(move || {
        //     x1.set(43);
        // });
        // let x2 = Arc::clone(&x);
        // std::thread::spawn(move || {
        //     x2.set(44);
        // });
    }
}

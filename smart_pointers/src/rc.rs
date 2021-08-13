// Struct that holds the value
use crate::cell::Cell;
use std::ptr::NonNull;
use std::marker::PhantomData;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>, // Treat this type as we have one of it
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });

        // This code is not good because when the function ends the `inner` Box is dropped
        // and Rc{inner: } points to nothing
        //Rc {inner: &*inner}
        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData
        }
    }
}

impl<T> Clone for Rc<T> {
    // We don't need the `T` to implement clone
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc { inner: self.inner , _marker: PhantomData}
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a box that is only deallocated when the last Rc goes away.
        // Since we have an Rc deref is fine
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // We are the only reference then is safe to drop the Box
            drop(inner);
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) }; // This drops the box
        } else {
            // There are other Rc's, don't drop is
            inner.refcount.set(c + 1);
        }
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn rc_bad(){

    }
}
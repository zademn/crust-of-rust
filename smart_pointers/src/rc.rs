// Struct that holds the value
use crate::cell::MyCell;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// This keeps the count common to all Rc. If each Rc would have an individual count,
///  when we increment we would only modify that 1 Rc.
struct RcInner<T> {
    value: T,
    refcount: MyCell<usize>,
}

pub struct MyRc<T> {
    inner: NonNull<RcInner<T>>,
    // Treat this type as we have one of it.
    // This makes the compiler know that you have one of these types and when we drop MyRc it will drop what's inside too. 
    _marker: PhantomData<RcInner<T>>, 
}

impl<T> MyRc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: MyCell::new(1),
        });

        // This code is not good because when the function ends the `inner` Box is dropped
        // and Rc{inner: } points to nothing
        //Rc {inner: &*inner}

        // SAFETY: NonNull is fine because Box gives a heap allocation and can't be null.
        MyRc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

// Idea is to increase the count.
impl<T> Clone for MyRc<T> {
    // We don't need the `T` to implement clone
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        MyRc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a box that is only deallocated when the last Rc goes away.
        // Since we have an Rc deref is fine
        &unsafe { self.inner.as_ref() }.value
    }
}

/// Drop logic for Rc. If count is 1 drop it, otherwise decrement the count.
impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // We are the only reference then is safe to drop the Box
            drop(inner); // This drops the value
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) }; // This drops the box.
        } else {
            // There are other Rc's, don't drop is
            inner.refcount.set(c + 1);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn rc_bad() {}
}

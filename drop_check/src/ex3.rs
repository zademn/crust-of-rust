#![feature(dropck_eyepatch)]

use std::marker::PhantomData;

/// Internally is the same as a Box
pub struct MyBox<T> {
    p: *mut T,
    _t: PhantomData<T>, // This tells the compiler that we may drop a `T`
}

/// #[may_dangle] tells the compiler that even though `MyBox` holds `T` I promise that
/// The code inside `drop` does *not* access the T.
/// It may drop(T) but not access it.
unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // Create a box and drop it. This will deallocate the box.
        // Safety: p was constructed from a Box and hws not been freed  since self exists.
        // Otherwise, drop could not be called.
        unsafe { Box::from_raw(self.p) };
        // This will drop the T but will not free the box
        // unsafe {
        //     std::ptr::drop_in_place(self.p);
        // }
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety : is valid since it was constructed from a valid T
        // and turned into a pointer through Box which turnes into aligned pointers
        // and hasn't been freed since self is alive.
        unsafe { &*self.p }
    }
}
impl<T> std::ops::DerefMut for MyBox<T> {
    // Here we can refer to Self::Target since DerefMut is a suubtrait of Deref.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.p }
    }
}
impl<T> MyBox<T> {
    pub fn new(t: T) -> Self {
        MyBox {
            p: Box::into_raw(Box::new(t)),
            _t: PhantomData,
        }
    }
}

use std::fmt::Debug;
struct Oops<T: Debug>(T);

impl<T: Debug> Drop for Oops<T> {
    fn drop(&mut self) {
        // Access the inner T when dropped.
        println!("{:?}", self.0);
    }
}

fn main() {
    let x = 42;
    let b = MyBox::new(x);
    println!("{:?}", *b);

    //  Our Box is not Covariant in T because it's invariant in the current form. 
    // This leads to a compiler error although it shouldn't.
    // A fix is to change *mut T to std::ptr::NonNull since NonNull is covariant. 
    let s = String::from("hello");
    let mut b1 = MyBox::new(&*s); // &*s is a &str
    let b2: MyBox<&'static str> = MyBox::new("aaa");
    b1 = b2;

    // This compiles fine with box because Box is covariant in T
    let s = String::from("hello");
    let mut b1 = Box::new(&*s);
    let b2: Box<&'static str> = Box::new("aaa");
    b1 = b2;
}

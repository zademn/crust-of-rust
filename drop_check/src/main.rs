#![feature(dropck_eyepatch)]

use std::marker::PhantomData;
use std::ptr::NonNull;

/// Internally is the same as a Box

pub struct MyBox<T> {
    // NonNull Makes the type covariant. Also we know it's NonNull because it comes from MyBox.
    p: NonNull<T>,
    // This tells the compiler that we may drop a `T`
    _t: PhantomData<T>,
}

struct Deserializer<T> {
    // To make a type covariant you can
    _t: PhantomData<T>, //
}

/// #[may_dangle] tells the compiler that even though `MyBox` holds `T` I promise that
/// the code inside `drop` does *not* access the T.
/// It may drop(T) but not access it.
/// #[may_dangle] is a temporary fix atm. 
unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // Create a box and drop it. This will deallocate the box.
        // Safety: p was constructed from a Box and hws not been freed  since self exists.
        // Otherwise, drop could not be called.
        unsafe { Box::from_raw(self.p.as_mut()) };
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety : is valid since it was constructed from a valid T
        // and turned into a pointer through Box which turnes into aligned pointers
        // and hasn't been freed since self is alive.
        unsafe { &*self.p.as_ref() }
    }
}
impl<T> std::ops::DerefMut for MyBox<T> {
    // Here we can refer to Self::Target since DerefMut is a suubtrait of Deref.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.p.as_mut() }
    }
}
impl<T> MyBox<T> {
    pub fn new(t: T) -> Self {
        MyBox {
            // Safety: Box never creates a null pointer.
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
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

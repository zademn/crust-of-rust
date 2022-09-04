#![feature(dropck_eyepatch)]

/// Internally is the same as a Box
pub struct MyBox<T> {
    p: *mut T,
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

    let mut y = 43;
    let b = MyBox::new(&mut y);
    println!("{:?}", y);

    // This compiles but shouldn't. 
    let mut z = 44;
    // When we drop the Box, we drop Oops. But the drop implementation for Oops access T
    let b = MyBox::new(Oops(&mut z));
    // let b = Box::new(Oops(&mut z));
    println!("{:?}", z);
    // drop(b) -- this accesses &mut z.
    // This means that there is a borrow in `Oops` that is used in drop(b) and z still gets accessed in the print.
    // This does not work with the std `Box`
    // This is solved by adding PhantomData
    
}

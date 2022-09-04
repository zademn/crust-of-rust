/// Internally is the same as a Box
pub struct MyBox<T> {
    p: *mut T,
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // Weird access of the insides of MyBox
        let _: u8 = unsafe { std::ptr::read(self.p as *const u8) };
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

fn main() {
    let x = 42;
    let b = MyBox::new(x);
    println!("{:?}", *b);

    // Although b holds a mutable reference to y it doesn't do anything with it.
    // So we should be able to still have mutablea access to y
    // With the current implementation the borrow checker does not allow it.
    let mut y = 43;
    let b = MyBox::new(&mut y);
    println!("{:?}", y); // read of y is not ok
    y = 44;
    // Since we defined our own drop implementation, the compiler doesn't know if y gets accessed
    // and wants to protect us against this possibility
    // Because of this we would have 2 uses of &mut y and this is not ok.
    drop(b)
}

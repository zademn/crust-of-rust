#[derive(Debug)]
struct MyRc<T> {
    // `*mut` is !Send to MyRc is !Send
    inner: *mut Inner<T>,
}

// All MyRc point to this same inner.
struct Inner<T> {
    count: usize,
    value: T,
}

//impl !Send for MyRc<T>;

impl<T> MyRc<T> {
    pub fn new(v: T) -> Self {
        MyRc {
            inner: Box::into_raw(Box::new(Inner { count: 1, value: v })),
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        // SAFETY: Since RC is !Send and we know that 1 thread can do one thing at a time
        // it means that there cannot be another thread that increments this, making this safe.

        // increment count when cloning
        unsafe { &mut *self.inner }.count += 1;
        MyRc { inner: self.inner }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let cnt = &mut unsafe { &mut *self.inner }.count;
        if *cnt == 1 {
            // drop the inner thing
            // SAFETY: MyRc is !Send => since we are the only thread, there is no concurrency here, therefore this is safe
            let _ = unsafe { Box::from_raw(self.inner) };
        } else {
            // Decrement count otherwise
            *cnt -= 1;
        }
    }
}

impl<T> std::ops::Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.value
    }
}
fn main() {
    let x = MyRc::new(1);
    // let y = x.clone();
    // std::thread::spawn(move || {
    //     let _ = y;
    //     drop(y);
    // });
    drop(x);
}

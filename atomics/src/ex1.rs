use std::{
    cell::UnsafeCell,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

const LOCKED: bool = true;
const UNLOCKED: bool = false;
pub struct MyMutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for MyMutex<T> where T: Send {}
impl<T> MyMutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }
    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // PROBLEM: Maybe 2 threads see "UNLOCKED", they lock the thread at the same time, do f() on the same value and then unlock.
        // Taking the lock at the same time causes a data race. 
        // A first solution is to use `compare_exchange` - However this is an expensive operation

        // If the thread is locked SPIN
        while self.locked.load(Ordering::Relaxed) != UNLOCKED {}
        // Otherwise, lock, call the function, unlock.
        self.locked.store(LOCKED, Ordering::Relaxed);
        // Safety: We hold the lock therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
}
use std::thread::spawn;
fn main() {
    let l: &'static _ = Box::leak(Box::new(MyMutex::new(0)));
    let handles: Vec<_> = (0..100)
        .map(|_| {
            spawn(move || {
                for _ in 0..1000 {
                    l.with_lock(|v| {
                        *v += 1;
                    })
                }
            })
        })
        .collect();

    // wait for the handles
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(l.with_lock(|v| *v), 100 * 1000);
}

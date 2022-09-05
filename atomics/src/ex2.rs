use std::{
    cell::UnsafeCell,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
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
        // PROBLEM: With Ordering::Relaxed instructions that don't depend on ceah otherc an move up and down.
        // Consider the labels. We can execute B A C or A C B which would break our locking mechanism!. 
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {} // A
        // Safety: We hold the lock therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() }); // B
        self.locked.store(UNLOCKED, Ordering::Relaxed); // C
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

#[test]
fn too_relaxed() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let thread1 = spawn(move || {
        let r1 = y.load(Ordering::Relaxed); // A
        x.store(r1, Ordering::Relaxed); // B
        r1
    });

    let thread2 = spawn(move || {
        let r2 = x.load(Ordering::Relaxed); // C
        y.store(42, Ordering::Relaxed); // D
        r2
    });

    let r1 = thread1.join().unwrap();
    let r2 = thread2.join().unwrap();

    println!("{r1}, {r2}");

    // With Ordering::Relaxed we can get r1 == r2 == 42
    // With Ordering::Relaxed there are almost no guarantess of the order of reading / writing.
    // D can happen before A and B can happen before C.
}

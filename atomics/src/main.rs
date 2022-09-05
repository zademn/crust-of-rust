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
        // Change to Ordering::Acquire to see the all operations that were stored with Release by another thread. 

        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {}
        // Safety: We hold the lock therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        // Change to `Release` to ensure the next thread that reads will see this operation. 
        self.locked.store(UNLOCKED, Ordering::Release); 
        ret
    }
}
use std::thread::spawn;

fn main(){
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn (move ||{
        x.store(true, Ordering::Release); // A
    });

    let ty = spawn (move ||{
        y.store(true, Ordering::Release); // B
    });

    let t1 = spawn (move ||{
        while !x.load(Ordering::Acquire){} // C
        if y.load(Ordering::Acquire){   // D
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn (move ||{
        while !y.load(Ordering::Acquire){} // E
        if x.load(Ordering::Acquire){ // F
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let z = z.load(Ordering::SeqCst); // Just read whatever z is 
    // What are the possible values for z?
    // is 2 possible?
    // A B C D E F -> 2
    // is 1 possible?
    // A C D -> 0, B E F -> 1 => 0 + 1
    // is 0 possible?
    // If we consider any ordering of letters then we can't get 0. But this is now how CPU works
    // We need to think in Happens before relationships
    // A  must happen before C & D.
    // B must happen before E & F.
    // But there is no relationship between A and F or B and D. 
    // Since there is no relationship they can happen *at the same time*
    // So we have the sequences  A C D -> 0 and B E F -> 0 happening at the same time => we get 0. 

    // If we change everything to SeqCst then 0 is not possible.
    // This means that if 1 thread sees A happened then ALL threads must see A happen

    println!("{z}");



}

#[test]
fn mutex_test() {
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

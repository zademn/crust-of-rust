# Atomics

[docs](https://doc.rust-lang.org/std/sync/atomic/)
[nomicon](https://doc.rust-lang.org/nomicon/atomics.html)


Atomic equivalents to types. They provide primitive shared-memory communication between threads. This primitivesa re he building blocks of other concurrent types. 

When you use atomics you issue different instructions to the CPU and place constraints on the code the compiler is allowed to do.  
This is important for threading. Ex:  which order do threads read values in, which values are visible to some thread.


Rust follows the [C++ memory model](https://en.cppreference.com/w/cpp/atomic/memory_order) (at the time of writing). 


**Remarks**
- Atomics are not inherently shared (they are placed on the stack). The easiest way to share them is by putting them on heap by wrapping them with `Box` or `Arc`. If you share using the stack you would usually go into lifetime hell. 
- Functions that modify atomics (like `load()`) use shared references `&self` instead of `&mut self` because the compiler will generate special atomic instructions to safely access and modify the values. 

**Methods**
- `new(v: T) -> AtomicT`
- `load(&self, order: Ordering) -> T` and `store(&self, val: T, order: Ordering)`. Accessing and storing memory
- `compare_exchange(...)`. Compare and exchange values if the comparison passes. 
- `fetch_foo()` - fetch and do the `foo` without another thread interfering. 

## Ordering

Which set of guarantees you expect for this memory access with respect to other threads. 
```rust
#[non_exhaustive]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```

```rust
fn too_Relaxed() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let thread1 = spawn(move || {
        let r1 = y.load(Ordering::Relaxed);
        x.store(r1, Ordering::Relaxed);
        r1
    });

    let thread2 = spawn(move || {
        let r2 = x.load(Ordering::Relaxed);
        y.store(42, Ordering::Relaxed);
        r2
    });

    let r1 = thread1.join().unwrap();
    let r2 = thread2.join().unwrap();



    println!("{r1}, {r2}");

    // With Ordering::Relaxed we can get r1 == r2 == 42
    // With Ordering::Relaxed there are almost no guarantess of the order of reading / writing.
}
```

`Relaxed`  
The compiler is allowed to shuffle the order of some operations to get more performance and optimize the code. In the above example we have `Ordering::Relaxed` - this means that we have no guarantees that the `load()` happens before the `store()` between the threads or even in the same thread. 
[c++ docs](https://en.cppreference.com/w/cpp/atomic/memory_order#Relaxed_ordering)

`Relaxed` is used for basic operations where ordering doesn't matter. Ex: keeping a count between threads. 


`Acquire` / `Release`  
- `Release` - Only available for operations that perform a store. All previous operations become ordered before any load of this value with `Acquire` (or stronger). All previous writes become visible to all threads that perform an `Acquire` load of *this* value.
  - *Intuition*: If we do a store a value with `Release`, any load of the same value that uses the `Acquire` must see all operations that happened *before* the store as having *happened before the store*
- `Acquire` - Only available for operations that perform a load.  When coupled with a load, if the loaded value was written by a store operation with `Release` (or stronger) ordering, then all subsequent operations become ordered after that store. In particular, all subsequent loads will see data written before the store.
    - *Intuition*: If a store happened with `Release`, everything that happened before that store will be seen by a load with `Acquire`. 


The `Acquire-Release` pair establishes a **happens-before relationship** between the thread that writes the value and the next thread that reads the value. 


`AcqRel`
- Do the load with Acquire semantics and the store with Release semantics. Usually for operations that load and store Ex: `fetch_add()`

**Remark**
- On some architectures (Ex: x86) will ensure `Acquire-Release` operations by default and you can't opt out. This is not the case for other architectures (like ARM). THis means that running tests on your machine only is not enough to test concurrent code. 

`SeqCst` - Sequential Consistent ordering. 
- **All** threads see **all** (not only the current memory) operations in the **same** order. 
- `SeqCst` interacts only with `SeqCst`. 
- `Acquire/Release` interact with `SeqCst`. 


## Fetch operations

Instead of telling the CPU what the new value will be tell the CPU how to compute it .This way the operation will not fail. The methods return the **previous** value.

```rust
fetch_add(), fetch_sub(), fetch_and(), ...
```

**Remark**
- `fetch_update(&self, ..., fn)` is the odd one out. The CPU has atomic add, sub etc but does not have atomic closures. So inside it there is a `compare_exchange` loop. 
- If the architecture does not have innate atomic equivalents for `fetch_foo` it will use a `compare_exchange ()` loop by default. 



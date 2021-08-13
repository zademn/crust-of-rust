# Notes

**Shareable mutable containers**
: Mutate with shared references in a controlled fashion

Standard library: `Cell`, `RefCell`, `Mutex`
- They have different restrictions over what you can do

`Cell`
: You can `get()`, `swap()`, `set()` the value 
: You cannot get a pointer / reference to the thing inside the `Cell`. `get()` only gets a **copy** of the item inside
: Implements `!Sync` from `UnsafeCell` -- only safe for single threads

Uses
- Compile time guarantees
- Multiple mutable references to the same object -- Ex: Graphs
- usually with things that have `Copy` -- small things like numbers -- fast to `get` out 

**Remark**
- you are not allowed to cas a shared cell  to an exclusive cell without going through `UnsafeCell`

`RefCell`
: Dynamically checked borrow rules. 
: has `.borrow() -> Option<&T>` and `.borrow_mut() -> Option<&mut T>`. `.borrow_mut()` returns `None` if we already have a mutable reference
: Implements `!Sync`
Example: Check for more mutable references at the same time in a recursive function. 


`Rc`
: **Reference counter pointer**
: Never provides mutability. 
: Deallocates the interior value when the last `Rc` dies
: Not thread-safe

Example: When you want multiple reference to a big data blob.


`std::marker::PhantomData`
: Pretend that you have a data of that type
: Tells Rust that when you drop a type check the drop functions of the PhantomData type

### Syncs
- Counters use atomics
- Block threads if more mutable references are given out
- `Arc` -- thread safe reference count
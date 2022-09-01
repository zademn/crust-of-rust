# Smart Pointers

**Shareable mutable containers**
: Mutate with shared references in a controlled fashion. 

Standard library: `Cell`, `RefCell`, `Mutex`
- They have different restrictions over what you can do and how you can mutate. 

`UnsafeCell`
- Only way to go from a shared reference to an exclusive reference. You are not allowed to cast a shared reference into an exclusive reference. 
- Basis to other cells.
- Unsafe to use.
- Can get the inside *shared* reference to mutate it.


`Cell`
- You can `get()`, `swap()`, `set()` the value. 
- You cannot get a pointer / reference to the thing inside the `Cell`. `get()` only gets a **copy** of the item inside. If you only have access to one reference then it's safe to mutate it.
- Implements `!Sync` from `UnsafeCell` -- only safe for single threads.

Uses
- Compile time guarantees
- Multiple mutable references to the same object -- Ex: Graphs
- usually with things that have `Copy` -- small things like numbers -- fast to `get()` out.

**Remark**
- you are not allowed to cas a shared cell  to an exclusive cell without going through `UnsafeCell`

`RefCell`
: Mutable memory location with dynamically checked borrow rules. 
- has `.borrow() -> Option<&T>` and `.borrow_mut() -> Option<&mut T>`. `.borrow_mut()` returns `None` if we already have a mutable reference
- Implements `!Sync`
Example: Check for more mutable references at the same time in a recursive function. 


`Rc`
- **Reference counter pointer**
- Stored on the heap.
- Never provides mutability. If you need mutability use `RefCell` or `Cell`.
- Deallocates the interior value when the last `Rc` dies.
- Not thread-safe (count is not thread-safe).

Example: When you want multiple reference to a big data blob.


`std::marker::PhantomData`
- Pretend that you have a data of that type
- Tells Rust that when you drop a type you have to check the drop functions of the PhantomData type. 

### Syncs
- `RwLock` -- `RefCell` where counters are kept using *atomics*. `borrow` and `borrow_mut` (called `read` and `write`) don't return an option, rather they return the ref or refmut but they **block** the current thread. 
- `Mutex` -- Simplified `RwLock` that only has `get_mut`.
- `Arc` -- thread safe reference count
- Block threads if more mutable references are given out. 


### Borrow module

`Cow` - Copy on Write
- Either contains a reference to a thing or the thing itself.
- Use: Most times you don't need a copy but *sometimes* you need to modify it. This avoids the ineficiency of cloning when the thing inside is not modified. 
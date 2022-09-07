# Dispatch 


**Monomorphization**  
Process of turning generic code into concrete code by filling in the required types. 

Example
```rust
/// Generic function that returns the length of a string. 
/// Accepts any time that can be turned into impl AsRef<str..
pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn main() {
    strlen("hello world"); // &'static str
    strlen(String::from("hello from a")); // String
}
```
Here the compiler generates 2 copies of `strlen`, one for `&'static str` and one for `String`. It will only generate for types that are **seen at compile time**, not for all existing types. 

**Some downsides**
- This makes it difficult to ship Rust binaries (compiled Rust code) as a library, because you need the source to generate distinct functions for each type. However the user that receives the binary might have a new type that should work but since the binary is already compiled it will not have an implemntation for the user's new type. 
- The binary gets larger. However, we only generate methods that are used, not all possible methods. So for a `Hashmap<String, String>` we will generate only the used methods, not all possible `Hashmap` methods. 

```rust
pub fn bool_then<T>(b: bool, f: impl FnOnce() -> T) -> Option<T> {
    if b {
        Some(f()) // here
    } else {
        None
    }
}
```
In this example `f`'s implementation can be implemented inline in the function during code generation. 


### Static dispatch
At compile time, the compiler knows what the actual type of the generic is. 


Example: In the following example consider the generic function `bar(h: impl Hello)`. 
```rust
pub trait Hello {
    fn hello(&self);
}

impl Hello for &str {
    fn hello(&self) {
        println!("hello {}", self);
    }
}
// This function 
pub fn bar(h: impl Hello) {
    h.hello();
}

pub fn main() {
    // Compiler looks at &str methods, doesn't find `hello()`
    // then he checks for traits in scope and finds it.
   "A".hello();
}
```
The compiler sees that `&str` is used and will generate something like:
```rust
pub fn bar_str(h: &str) {
    h.hello();
}
```

## Dynamically sized types

**Motivation**

Consider the following example: 
```rust
pub fn vec_bar(s: &[impl Hello]) {
    for h in s {
        h.hello();
    }
}
```
Or written differently but with the same meaning:
```rust
pub fn vec_bar<H: Hello> (s: &[H]) {
    for h in s {
        h.hello();
    }
}
```
This will take slices that have elements of the **same** type. So the following will work 
```rust
vec_bar(&["A", "B"]);
vec_bar(&[String::from("A"), String::from("B")]);
```
Howevere maybe we want to take **different** things that `impl Hello`. For example
```rust
vec_bar(&["A", String::from("B")]);
```
Under the current implementation this will not compile. Here [**trait objeccts**](https://doc.rust-lang.org/book/ch17-02-trait-objects.html) can help us.

**Trait object**  
Object that represent a trait. They only behave as the underlying trait. 

However just switching `impl Hello` to `dyn Hello` will not work at compile time:
```rust
 --> dispatch_fat_pointers/src/main.rs:46:24
   |
46 | pub fn vec_bar_dyn(s: &[dyn Hello]) {
   |                        ^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `dyn Hello`
   = note: slice and array elements must have `Sized` type
```


### Sized trait
[Docs](https://doc.rust-lang.org/std/marker/trait.Sized.html)

Marker trait that mark types with a *constant size known at compile time*. Types are always sized if they can be, you needn't implement it. 
Example of unsized types: `[T]`, `str`.

**Motivation**

Let's go back to the `strlen()`  example:
```rust
pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}
```
Here the compiler needs to know exactly how much space the arguments take up. This is because he needs to generate assembly code and in there he needs to know exactly how to play with registers and the stack. When you call a function you need to know how much to allocate for the stack for the arguments, and how much for the return type.

So for the generated concrete `String` implementation:

```rust
pub fn strlen(s: String) -> usize {
    s.as_ref().len()
}
```
the compiler will know we will have 3 usizes: length of the string, size of the allocation and a pointer to the first element of the string. Now the compiler knows how much space to allocate on the stack. 


**Sizing by indirection** - You can use `&dyn ...` or `Box<dyn ...>` to indirectly size the unsized dynamic objects. This is becuase the references `&` and `Box` have a known size at the compile time: the size of 1 (or 2) pointers. 

The `Box` definision is something like
```rust
struct Box<T: ?Sized> {}
```
whre `?` opts out of the *everything must be sized* constraint. 

So the `strlen` example will look as follows:
```rust
pub fn strlen_dyn(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}
``` 
or
```rust
pub fn strlen_dyn(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().len()
}
```

These are all ways to construct a **trait object**.

**Remark** - Trait objects are type erasors: When you use a `&dyn SomeTrait` you can only use the methods that `SomeTrait` provide. For example if you use `&dyn Clone` the only function you can use is `clone()`.

### Dynamic distpatch. 
Statically we know what code to generate. However when we have `dyn` objects we don't know what type we have and we don't know what code to generate because we will know what type we'll receive only at **runtime**. 

Pointers to dynamically sized objects (*trait objects*) will have double the size (Fat pointers)
1. Pointer to the concrete implemented type. 
2. Pointer to the vtable for the referenced trait. 

**Vtable**  
A data structure that has pointers to the methods of the trait that some type implements. 

So for each concrete type a different vtable will be constructed. Vtables are built at compile time (this is not enforced, they can be constructed on the fly). 

For the hello example a following vtable schemacan be constructed
```rust
//dyn Hello, vtable:
struct HelloVtable{
    hello: *mut Fn(*mut ()),
}

// And when we receive a &str we have:
// &str -> &dyn Hello
// 1. Pointer to str
// 2. HelloVtable{
//     hello: &<str as Hello>::hello    }
// }
```


**Remark** -- Every vtable contains drop

Consider the following example.
```rust
fn say_hello(s: Box<dyn AsRef<str>>){
    // what happens when `s` goes out of scope?
}
```
When `Box` goes out of scope, the interior of `Box` must go out of scope so a drop is needed. For this every trait object contains `Drop`

## Limitations

### Multiple traits

Consider the following example:
```rust
pub fn baz(s: &(dyn Hello + AsRef<str>)){ // Compiler error on this line 
    s.hello();
    let s = s.as_ref();
    s.len();
}
```
```rust
error[E0225]: only auto traits can be used as additional traits in a trait object
  --> dispatch_fat_pointers/src/main.rs:69:29
   |
69 | pub fn baz(s: &(dyn Hello + AsRef<str>)) {
   |                     -----   ^^^^^^^^^^ additional non-auto trait
   |                     |
   |                     first non-auto trait
   |
   = help: consider creating a new trait with all of these as supertraits and using that trait here instead: `trait NewTrait: Hello + AsRef<str> {}`
```

This means that we need a fat pointer that consists of 3 pointers: 1 for the type, 1 for `Hello`'s vtable and one for `AsRef<str>`'s vtable. This means that we need to handle arbitrary length fat pointers. 

The solution is to combine the traits under a single trait to keep only 1 vtable:
```rust
// This combines the vtables of Hello and AsRef<str>
pub trait HelloAsRef: Hello + AsRef<str> {}
```

**Remark**: Some Rust marker traits (like `Send` and `Sync`) do not have vtables and they can be used without creating a supertrait. This does not work, yet, with user implemented marker traits.

### Associated types

Consider the following example with an associated type:
```rust
pub trait Hello {
    type Name; // here
    fn hello(&self);
}

impl Hello for &str {
    type Name = (); // here
    fn hello(&self) {
        println!("hello {}", self);
    }
}


pub fn say_hello(s: &dyn Hello) { // Error here: The value of the associated type must be specified
    s.hello()
}
```
Instead we need to add the `Name` type by hand:
```rust
pub fn say_hello(s: &dyn Hello<Name = ()>) {
    s.hello()
}
```


### Static trait methods

Consider the following example where we try to make  a static trait function:
```rust
pub trait Hello {
    fn hello(&self);
    fn weird(){ /*default implementaton */} // here
}

impl Hello for &str {
    fn hello(&self) {
        println!("hello {}", self);
    }

    fn weird() {/*different implementation*/}
}
```
And then we try to call it:
```rust
pub fn say_weird(s: &dyn Hello) { // error here: `Hello` cannot be made into an object
    s.weird();
    // is something like:
    (dyn Hello)::weird(); // Which type are we calling weird on?
}
```
The error is a bit vague. This doesn't work becuase the compiler doesn't know which vtable to look at. The `weird` method is not associated with any type, so there is no `&self` to identify which type it comes from. 

A trick that you can do is to force types that `impl Hello` to be sized. This basically tells us to opt out of the vtable (and don't call this function from a trait object). 
```rust
pub trait Hello {
    fn hello(&self);
    fn weird(&self) where Self: Sized {} //here
}
```

You can also disallow the whole trait to be used on trait objects (rare case)
```rust
pub trait Hello  where Self: Sized { // here
    fn hello(&self);
    fn weird(&self){} 
}
```

### Other unsized types
The following types you must put behind a pointer:
- `dyn Trait` -> * -> `(*mut data, *mut vtable)`
- `[u8]` -> * -> `(*mut data, usize length)`
- `str` -> * -> `(*mut data, usize length)`

You can make your own dynamically sized types. But you must put the unknown sized attribute at the end:
```rust
/// A dynamically size type. 
struct Foo{
    f: bool,
    x: bool,
    // This works because it's last. If x was last we wouldn't know the offset of `x` because `t` is dynamic. 
    t: [u8], 
}
```

Example of a manually constructed vtable: [RawWakerVtable](https://doc.rust-lang.org/std/task/struct.RawWakerVTable.html).




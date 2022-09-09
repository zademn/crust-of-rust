# Functions, Closures and their traits

## Function items and pointers

Let's start easy. Conside the following example:
```rust
fn main() {
    let x /*: fn bar()*/ = bar
    println!("{}", std::mem::size_of_val(&x)) // 0 
    
}
fn bar(){}
```
Maybe surprisingly, `x` here is not a function pointer. It's a **function item**. 

**Function item**  
A 0 sized value that is carried around at compile time and **uniquely** identifies the an instance of a function.  

**Function pointer**  
Pointer to a function with a given signature. Cannot be turned into a function item. 


This means that if we have a generic function we need to specify the type. Otherwise we wouldn't be able to *uniquely* identify (since generics may take different types). 

```rust
fn bar<T>(){}
fn main() {
    // this does not compile
    let x /*: fn {unknown*/ = bar;
    // This compiles
    let x = bar::<i32>

}
```

**Remark**:
- function items are coercable into function pointers.
- By default, assigning a function item without using it does not generate the function body (because there is no use for it). However when you use it as an argument it may get used later so the compiler needs to generate code. Therefore it will coerce the item to a pointer. Example:

```rust
fn main() {
    let x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));

    // This coerces the argument to a function pointer. 
    baz(bar::<u32>); // will print 8
    baz(bar::<i32>); // will print 8
    baz(x); // will print 8

}
fn bar<T>(_: u32) -> u32 {
    0
}

fn baz(f: fn(u32) -> u32) {
    println!("{}", std::mem::size_of_val(&f))
}
```


## Function traits

3 main traits:
- `Fn` - takes a reference to self - `&self`. Can be called multiple times and multiple times at the same time
- `FnMut` - takes an exclusive reference to - `&mut self`. Can be called multiple times but once at a time (since only one variable can hold an exclusive reference to this at a time.). 
- `FnOnce` - takes an owned reference to self - `self`. This can only be called once, because the value of the function gets moved and you cannot call it again. 


Because of these definitions and use cases they implemet a **hierarchy**. `FnMut` and `Fn` are subtraits of `FnOnce` => everything that accepts `FnOnce` can accept `FnMut` and `Fn`.
Similarly, `Fn` is a subtrait of `FnMut` and can be used everywhere where `FnMut` is expected. 


Function pointers and items have **no state**. No lifetimes, they don't reference anything in other stack frames. They are just a bunch of code. Since there is no state they don't really care about `self`. Because of this all function pointers and items implement all 3 traits. 


## Closure

**Closure**  
They are named closures becuase they "close" an environment. Basically they capture things from their environment and generate a unique function that specifically uses / references data from the current environment. 

They are coercable into function pointers. 
```rust
fn baz(f: fn(u32) -> u32) {
    println!("{}", std::mem::size_of_val(&f))
}

fn main() {
    let f = |_: u32| 0;
    baz(f); // 8
}
```

**Remark**: If a closure captures a variable it **cannot** be coerced into a function pointer. This is because you need the extra state of what you capture. Example:

```rust
fn main() {
    let z = String::new();
    let f_consuming = |_: u32| {
        let _ = z; // consume z
        0
    };
    baz(f_consuming); // Compiler error: Function cannot coerce into a function pointer. 
}

fn baz(f: fn(u32) -> u32) {
    println!("baz size: {}", std::mem::size_of_val(&f))
}
```

Now, functions can also take types that implement the trait. Consider the following example
```rust

```rust
fn main() {
    let z = String::new();
    let f_consuming = || {
        let _ = z; // consume z
    };
    //baz(f_consuming); // compiler error: cannot coerce into function pointer
    quox_fn(f_consuming);
    quox_fn_mut(f_consuming);
    quox_fn_once(f_consuming);

    let mut z = String::new();
    let f_clear = || z.clear();
    // quox_fn(f_clear); // Compier error: Expected closure that implements the FnTrait
    quox_fn_mut(f_clear);
    // quox_fn_once(f_clear);

    let f_drop = || drop(z);
    // quox_fn_mut(f_drop); // Compiler error: expected closure that implements FnMut, found FnOnce
    quox_fn_once(f_drop);
}

fn quox_fn(f: impl Fn()) {
    (f)()
}
fn quox_fn_mut(mut f: impl FnMut()) {
    (f)()
}
fn quox_fn_once(f: impl FnOnce()) {
    (f)()
}

```

- `f_consuming` can be taken by a `Fn` because it only needs a shared reference to `z`. 
- `f_clear` needs an exclusive reference to `z` to clear the string, therefore it needs a `FnMut`.
- `f_drop ` needs an owned reference because it consumes the whole string `z`. 



## `dyn Fn*()`

When using `dyn Fn*()` you need to use the specific wrapping reference (with the same hierarchy as above):
- `Fn()` -> `&dyn Fn()`
- `FnMut()` -> `&mut dyn FnMut()`
- `FnOnce()` -> `Box<dyn FnOnce()>`


## For bound

```rust
fn main(){
    quox(|x| x);
}

fn quox<F>(f: F)
where
    // For any lifetime `'a` this function should keep this bound. 
    // Very rare use, usually the compiler can infer. 
    F: for<'a> Fn(&'a str) -> &'a str,
{
}
```
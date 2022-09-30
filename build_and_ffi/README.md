# Build Scripts
[Cargo book](https://doc.rust-lang.org/cargo/reference/build-scripts.html)


**Build script**  
Program that runs before your code is compiled. It's related to cargo, not rust.  
If you have `build.rs` in your project cargo will compile and run that file before building your crate.

**Remarks**:
-  The output of `build.rs` is not included in the target. So your build script must be written in such a way to run on many OSs and architectures. 
- The `build.rs` can be renamed in the `Cargo.toml`. 
- Build script output is only printed when the build *fails*. 

There are different ways to comunicate between the library and the crate / exterior

### `OUT_DIR`
One way to communicate is using a directory where the build script can write too.

The build script has access to the `OUT_DIR` dir (which is a subdir of `target/`).
For example, the build script can generate files in `OUT_DIR`, and the main crate can use these files. 

In `target/debug/build/<crate-name>-<id>/stderr` you can see the output from the build script. this can be used by other crates and can be useful too see how a thing was build. 

```rust
// build.rs
fn main() {
    dbg!(std::env::var("OUT_DIR"));
    // Write to foo.rs the function foo() that prints "Hello from foo"
    std::fs::write(
        std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("foo.rs"),
        "pub fn foo() {println!(\"Hello from foo\");}",
    )
    .unwrap();
}

```

```rust
// main.rs
mod foo {
    // copy paste the file. NOT eval
    include!(concat!(env!("OUT_DIR"), "/foo.rs"));
}

fn main() {
    println!("{}", env!("OUT_DIR"));
    foo::foo();
}
```

**Remark**
- Build scripts are very sharp tools, they can read and write arbitrary files, they can connect to the internet and upload files etc. We put a lot of trust into build tools and we need to be careful with this trust


## Cargo directives

Build scripts can communicate with cargo. This happens by printing to stdout some directive that starts with `cargo:`. Cargo will interpret these lines as instructions that will influence compilation of the package. All other lines are *ignored*.

Directives 

**Config vs features**
- `cfg` is a rustc operation that enables conditional compilation. It has nothing to do with cargo
- `features` are part of cargo and you set in `Cargo.toml` and let's you easily pass `cfg` flags. Cargo will turn features into `cfg`

```rust
// rustc --cfg=feature=foo --cfg=openssl_1_1_0


// Only compile `foo` if openssl version is at least 1_1_0
#[cfg(openssl_1_1_0))]
fn foo() {}
```

If you have a shared library the convention is that there should a single crate with the `*-sys` suffix that binds against the shared library and exposes the **raw** ffi. No safe binding, no ergonomic interface, just binds it. After that, we can have different libraries that wrap the `*-sys` and make it prettier. 


## autocfg
[autocfg](https://docs.rs/autocfg/latest/autocfg/)

Let's you check available features based on Rust version (compiler support). 

# FFI

## Bindgen

Tool that automatically generates rust FFI bindings to C files. If you have a C header file you can call bindgen on it and generate a Rust file that has the equivalent Rust types and `extern fn`s

The `extern` keyword changes the calling convention for that function. 
1. You don't need a body (basically it tells the compiler that the function is not defined here. Check the symbol table of the binary). 
2. Use the C calling convention for the function. 

Opaque types are used when we don't care about the internals. Basically the types are only going to be used with pointers. In Rust they are usually defined as follows:
```rust
pub enum some_type {}
```
They are empty enums instead of unit structs because you don't want users to be able to construct opaque types. You cannot construct an empty enum in Rust. 

Struct types have `#[repr(C)]`.

extern fn functions are inherently `unsafe`. 

**A problem with bindgen**  
bindgen may not be consistent with future updates. This means that for the same C file a future version of bindgen might generate different code. This is important because using bindgen when building is **common** and when we generate `*-sys` with bindgen its API might change, which breaks backwads compatibility. So I, the developer, have to make sure that the public API doesn't change at all. Otherwise we need to do a major release of my `*-sys` crate and make other dependent crates bump their version. 


## Calling rust from C

- Make sure you match the types, calling conventions and representation. 
- `[no_mangle]` ensures that the  compiler won't rename the symbol in binary. This means that the symbol can be found in C

```rust
#[no_mangle]
pub extern "C" fn this_is_rust(arg: std::os::raw::c_int) -> std::os::raw::c_char{
    b'x' as i8
}
```
```c
// Retain the name with no_mangle
extern char this_is_rust(int);
```

[cbindgen](https://github.com/eqrion/cbindgen) let's you generate C bindings from Rust code. This can be useful when other languages don't have a binding generator for Rust but they have for `C`. So if you can't go `Rust -> Language` you can go `Rust -> C -> Language`.

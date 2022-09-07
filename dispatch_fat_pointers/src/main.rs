#![allow(unused)]

/// Generic function that returns the length of a string.
/// Accepts any time that can be turned into impl AsRef<str>
/// At code generation the compler will create a copy of this function for each
/// type that appears in the code.
pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

// In this function
pub fn bool_then<T>(b: bool, f: impl FnOnce() -> T) -> Option<T> {
    if b {
        Some(f())
    } else {
        None
    }
}

pub trait Hello {
    fn hello(&self);
    // fn weird(&self)
    // where
    //     Self: Sized,
    // {
    // }
}

impl Hello for &str {
    fn hello(&self) {
        println!("hello {}", self);
    }
}
impl Hello for String {
    fn hello(&self) {
        println!("hello {}", self);
    }
}

pub fn bar(h: impl Hello) {
    h.hello();
}

/// Can only take slices with elements of the same type
pub fn vec_bar(s: &[impl Hello]) {
    for h in s {
        h.hello();
    }
}

pub fn strlen_dyn(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

// pub fn vec_bar_dyn(s: &[dyn Hello]) {
//     for h in s {
//         h.hello();
//     }
// }

pub fn say_hello(s: &dyn Hello) {
    // dyn Hello, vtable:
    // struct HelloVtable{
    //     hello: *mut Fn(*mut ()),
    // }

    // &str -> &dyn Hello
    // 1. Pointer to str
    // 2. HelloVtable{
    //     hello: &<str as Hello>::hello    }
    // }
    s.hello()
}

// pub fn say_weird(s: &dyn Hello) {
//     s.weird();
// }

pub trait HelloAsRef: Hello + AsRef<str> {}
pub fn baz(s: &(dyn HelloAsRef)) {
    s.hello();
    let s = s.as_ref();
    s.len();
}

/// this can only take function pointers (no closures). It's basically an address. 
/// Since it's not a wide pointer that has a data section you don't have where to pass the data. 
fn fn_fn(f: fn()) {}

/// This can take closures too. This is a vtable (trait obj). 
/// This has 1 pointer calling the function and another pointer to the data passed to the function.  
fn fn_dyn(f: &dyn Fn()) {}

/// Generic over Fn()
/// You generate a copy of `fn_impl`. This gets monomorphised for each type of `Fn()`  
/// You also need to propagate the specific type in structs that hold / use this. 
fn fn_impl(f: impl Fn()) {}

pub fn main() {
    // strlen("hello world"); // &'static str
    // strlen(String::from("hello from a")); // String

    // Compiler looks at &str methods, doesn't find `hello()`
    // then he checks for traits in scope and finds it.
    "A".hello();

    vec_bar(&["A", "B"]);
    vec_bar(&[String::from("A"), String::from("B")]);
    // vec_bar(&["A", String::from("B")]);

    let x = "hello";
    // fn_fn(&|| {
    //     let _ = &x;
    // });

    fn_dyn(&|| {
        let _ = &x;
    });

    fn_impl(&|| {
        let _ = &x;
    });
}

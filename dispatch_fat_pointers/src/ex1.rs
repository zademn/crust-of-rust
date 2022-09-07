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

/// Can only take 
pub fn vec_bar(s: &[impl Hello]) {
    for h in s {
        h.hello();
    }
}

pub fn main() {
    // strlen("hello world"); // &'static str
    // strlen(String::from("hello from a")); // String

    // Compiler looks at &str methods, doesn't find `hello()`
    // then he checks for traits in scope and finds it.
    "A".hello();

    vec_bar(&["A", "B"]);
    vec_bar(&[String::from("A"), String::from("B")]);
    // vec_bar(&["A", String::from("B")]);
}

#![allow(unused)]
//#![allow(dead_code)]

// 2 lifetimes. 1 for the mutable borrow and 1 for the str we're pointing into
// We return the lifetime of the str we're pointing into
pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    // Find the index of the delim
    if let Some(i) = s.find(delimiter) {
        // before the delim.
        let prefix = &s[..i];
        // after the delim.
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn strtok_works() {
    //     let mut s = "hello wolrd";
    //     {
    //         let hello = strtok(&mut s, ' ');
    //         assert_eq!(hello, "hello");
    //         // hello borrow dies here.
    //     }
    //     assert_eq!(s, "world");
    // }

    #[test]
    fn strtok_works() {
        let mut s = "hello world";
        let hello = strtok(&mut s, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(s, "world");
    }
}

use std::marker::PhantomData;
struct Foo<T> {
    // some ields
    _t: PhantomData<T>,
}

struct Bar<T> {
    // some ields
    _t: PhantomData<fn() -> T>,
}
struct Baz<T> {
    // some ields
    _t: PhantomData<fn(T)>,
}
// fn main() {
//     let s = String::new();
//     let x: &'static str = "hello world";
//     let mut y = &*s;
//     y = x;
// }

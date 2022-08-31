// https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa

// # Lifetimes
// the pointers are valid until `'a` dies -> StrSplit struct in our case
// ## Static lifetime
// `'static` lifetime lives util the end of the program
// let x =
// 'a &str  &'static str
// This works because we can assign a longer lifetime to a shorter one

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delim> {
    remainder: Option<&'haystack str>, // part of string we haven't looked at
    delimiter: &'delim str,
}

// # Anonymous lifetimes
// '_ = Anonymous lifetime = Guess what lifetime this is?
// Only works if there is only 1 lifetime he can guess from
//
// ## Example 1:
// impl Foo{
//     fn get_ref(&self) -> &'_ str{} // The compiler can only guess the `&self` lifetime.
// }
// impl Foo{
//     fn get_ref(&'a self) -> &'a str{} // No need for `'a`
// }
//
// ## Example 2:
// fn foo<'x, 'y>(x: &'x str, y: &'y str) -> &'x str{}
// Turns into:
// fn foo(x: &str, y: &'_ str) -> &'_ str{}
// This gets ignored ---^           ^
// This gueses `x`'s lifetime ----- |

impl<'haystack, 'delim> StrSplit<'haystack, 'delim> {
    // pub fn new(haystack: &str, delim: &str)
    // Compiler error: If we don't specify lifetimes, we can have a StrSplit alive
    // but `haystack` and `delim` can be deallocated
    pub fn new(haystack: &'haystack str, delim: &'delim str) -> Self {
        return Self {
            remainder: Some(haystack), // part of string we haven't looked at
            delimiter: delim,
        };
    }
}

impl<'haystack, 'delim> Iterator for StrSplit<'haystack, 'delim> {
    // lifetime of the returned value
    type Item = &'haystack str; // Rust needs to know how long can it keep this pointer. Till the end of the program?
    fn next(&mut self) -> Option<Self::Item> {
        // get a mutable referance to the remainder if it exists
        if let Some(ref mut remainder) = self.remainder {
            // Get a reference to the matched thing, not the thing itself
            if let Some(next_delim) = remainder.find(self.delimiter) {
                // split by delimiter
                let until_delim = &remainder[..next_delim];
                *remainder = &remainder[(next_delim) + self.delimiter.len()..];
                return Some(until_delim);
            } else {
                // fn take(&mut self) -> Option<T>
                return self.remainder.take();
            }
        } else {
            return None;
        }
    }
}

#[allow(unused)]
fn until_char<'s>(s: &'s str, c: char) -> &'s str {
    // Error without lifetimes: "cannot return value referencing temporary value"
    // `c` has a lifetime different than `s`
    // `c` dies this function
    // The rust compiler takes the shorter lifetime (`c`'s lifetime)
    // and it can't return a lifetime that dies in this function
    let delim = &format!("{}", c);
    StrSplit::new(s, &delim)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("Hello world", 'o'), "Hell");
}

#[test]
fn it_works() {
    let s = "a,b,c,d,e";
    let letters = StrSplit::new(s, ",").collect::<Vec<_>>();
    println!("{:?}", letters);
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn it_works2() {
    let s = "a,b,c,d,e,";
    let letters = StrSplit::new(s, ",").collect::<Vec<_>>();
    println!("{:?}", letters);
    assert_eq!(letters, vec!["a", "b", "c", "d", "e", ""]);
}

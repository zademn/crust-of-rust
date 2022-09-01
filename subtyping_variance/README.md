# Subtyping and Variance.

**Subtype -- informal def**
: Let `T`, `U` be to types. `T` is a subtype of `U` is `T` is *at least as useful* as `U`. Denoted `T:U`. You can do as much with `T` as you can with `U`
: Ex: `'static : 'a`

### Variance

**Remark**
- `T` **cannot** be used anywhere `U` can be used. 

```rust
// This works.
fn main(){
    let s = String::new();
    let x: &'static str = "hello world";
    let mut y /*: &'a */ = &*s;
    y = x; // 'static -> 'a
}
```


**Covariance**
: Provide an argument that is more useful. Most things in Rust are covariant
```rust
// You can provide any type that is a subtype of 'a str
fn foo(&'a str){}

// Both work because 'static is a subtype of 'a
foo(&'a str)
foo(&'static str)

```

**Contravariance**
: Need to provide an argument that is less useful. Only in function types today. 
```rust
fn foo(bar: Fn(&'a str) -> ()){
    // If bar takes a 'static str (like below) then calling bar with a 'a str is not possible
    // bar "restricts" all that is called inside to 'static. 
    // So I can't hand `foo` a funtion with stricter requirements. 
    
    bar("") // /*'a */
}

// Here bar restricts all calls to 'static. 
foo(fn(&'static str) {})
```
```rust
//Covariance
&'static str // More useful.
&'a str // Less useful. Lives less. 
&'static str <: &'a str

//Contravariance
Fn(&'static str) // Less useful. Must provide a "long" lifetime str
Fn(&'a str) // More useful.
Fn(&'a T) <: Fn(&'a static T)
```
**Invariance**
:  Must provide something that is the same as what it was specified. Not covariant nor contravariant (not more useful nor less useful).
```rust
fn foo(s: &mut &'a str, x: &'a str){
    *s = x
}


let mut s: &'static str = "hello world";
// z is a local thing
let z = String::new();
// &z is non 'static. 
// fn foo(s: &mut &'a      str, x: &'a str) // signature
// fn foo(s: &mut &'static str, x: &'a str) // what was passed in 
// &mut is invariant. Since the signature is different to what was passed in it will not compile
// If &mut was covariant &mut s will get downgraded to 'a (since 'static is more useful).
foo(&mut s, &z);
// We drop z
drop(z);
// Since in foo we changed *s = x. now s will point to a `'a` lifetime (z) which was dropped.
// `s` was supposed to be a 'static str but now it's pointing to 'a z which was dropped
println!("{}", s); ->
```





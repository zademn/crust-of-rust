# Declarative macros

[Rust reference](https://doc.rust-lang.org/reference/macros-by-example.html)
[Little book of rust macros](https://danielkeep.github.io/tlborm/book/index.html)
[logrocket blog](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/)

**Macros**
: Macros are code that writes other code

When you declare macro instead of having a variable followed by a type you have a "syntax type".

Decalrative macros are decalred using `macro_rules!`. They are in essence just substitution based on patterns. 

Identifiers in macro world live in macro world.
```rust
macro_rules! identif {
    ($z: ident) => {
        let x = 42; // This lives only inside this macro. Cannot access from inside. 

        //y += 1; //Error: y not found in scope
        $z += 1; // This can be accessed from outside
    };
}
```

Uses for declarative macros
- It's handy to generate repeated code. 
- Easier to work with than proc macros.
- Lightweight compared to proc macros (avoids the extra step introduced by proc macros).

Be careful with `$e: expr` in iterations. If the `expr` is expensive to compute it will get computed each time it gets substituted. It's better to compute it once at the start of the macro, keep it and clone when needed. 



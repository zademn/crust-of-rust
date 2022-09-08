# Async

In the following example `foo1` is equivalent to `foo2`
```rust
async fn foo1() -> usize {
    0
}
fn foo2() -> impl Future<Output = usize> {
    async { 0 }
}
```
This says that at some point in he future `foo1` will get turned into a future. 

**await**  
Don't run the following instructions until the thing that is awaited finishes executing.  
A future **does nothing** until it's awaited.

The idea of async is that it executes code in *chunks*. If we have multiple `await`s, we execute code until we reach the next `await`. 


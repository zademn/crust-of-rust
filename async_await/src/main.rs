#![allow(dead_code)]
use std::future::Future;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello world");
    let _ = foo1().await;
}

/// foo1 is equivalent to foo2
async fn foo1() -> usize {
    println!("foo1");
    0
}
fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo2");
        foo1().await;
        0
    }
}

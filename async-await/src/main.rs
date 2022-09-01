#![allow(dead_code, unused_variables)]



fn main() {
    println!("Hello world");
    let x = foo1();
}

async fn foo1() -> usize {
    println!("foo");
    0
}

async fn foo2() -> usize {
    println!("foo");
    0
}




mod foo {
    // copy paste the file. NOT eval
    include!(concat!(env!("OUT_DIR"), "/foo.rs"));
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

}

fn main() {
    println!("{}", env!("OUT_DIR"));

    foo::foo();
}

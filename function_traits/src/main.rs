fn main() {
    let x = bar::<i32>;
    println!("Function item size: {}", std::mem::size_of_val(&x));

    // This coerces the argument to a function pointer.
    baz(x);
    baz(bar::<i32>);

    // Closures

    let f = || {};
    println!("Closure size: {}", std::mem::size_of_val(&f));
    baz(f);

    let z = String::new();
    let f_consuming = || {
        let _ = z; // consume z
    };
    //baz(f_consuming); // compiler error: cannot coerce into function pointer
    quox_fn(f_consuming);
    quox_fn_mut(f_consuming);
    quox_fn_once(f_consuming);

    let mut z = String::new();
    let f_clear = || z.clear();
    // quox_fn(f_clear); // Compier error: Expected closure that implements the FnTrait
    quox_fn_mut(f_clear);
    // quox_fn_once(f_clear);

    let f_drop = || drop(z);
    // quox_fn_mut(f_drop); // Compiler error: expected closure that implements FnMut, found FnOnce
    quox_fn_once(f_drop);

    let t = make_fn();
    t();
}
fn bar<T>() {}

fn baz(f: fn()) {
    println!("baz size: {}", std::mem::size_of_val(&f))
}

fn quox_fn(f: impl Fn()) {
    (f)()
}
fn quox_fn_mut(mut f: impl FnMut()) {
    (f)()
}
fn quox_fn_once(f: impl FnOnce()) {
    (f)()
}

fn make_fn() -> impl Fn() {
    // Everytime you call make_fn you allocate a new string and "move" it to the closure lifetime.
    // WHen the closure will get dropped the state of the closure gets dropped and z will drop.
    let z = String::new();
    move || {
        println!("{}", z);
        // let _ = z;
    }
}

fn hello(f: Box<dyn Fn()>) {
    f();
}




#![allow(unused_variables)]
#[allow(unused)]
macro_rules! my_add {
    ($a: expr, $b: expr) => {
        $a + $b
    };
}
#[allow(unused)]
macro_rules! identif {
    ($z: ident) => {
        let x = 42; // This lives only inside this macro

        //y += 1; // y not found in scope
        $z += 1;
    };
}

#[macro_export] // pub formacro
macro_rules! my_vec{
    // Empty vector
    // Ex: `my_vec![]`
    () => {
        Vec::new()
    };
    // Multiple elements. * means none or more.
    // Ex: `my_vec![1, 2]
    ($($element: expr),*) => {
        // Don't forget the extra block `{}` becase we write code.
        {
            let mut vs = Vec::new();
            $(
                vs.push($element);
            )* // Add elements multiple times.
            vs
        }
    };
    ($($element: expr,)*) => {
        {
            crate::my_vec![$($element),*]
        }
    };
    // Element and  count
    // Ex: `my_vec![1 ; 10]
    ($element:expr; $count:expr) => {
        {
            let mut vs = Vec::new();
            vs.resize($count, $element);
            vs
        }
    }
}

/// Trait that returns max value
trait MaxValue {
    fn max_value() -> Self;
}
/// Implement a trait that returns the max value
macro_rules! max_impl {
    // Give the type
    ($t: ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}
max_impl!(u16);
max_impl!(u32);
max_impl!(u64);

#[test]
fn test_my_add() {
    assert_eq!(my_add!(1, 2), 3);
}

#[test]
fn test_identif() {
    let y = 42;
    let mut z = 43;
    identif!(z);
    // x + 1; // x is not found in scope
    assert_eq!(z, 44);
}
#[test]
fn empty_vec() {
    let x: Vec<u32> = my_vec![];
    assert!(x.is_empty());
}

#[test]
fn one_vec() {
    let x: Vec<u32> = my_vec![2];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 2);
}

#[test]
fn more_vec() {
    let x: Vec<u32> = my_vec![2, 3];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 2);
    assert_eq!(x[1], 3);
}

#[test]
fn trailing_comma() {
    let x: Vec<u32> = my_vec![2, 3,];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 2);
    assert_eq!(x[1], 3);
}

#[test]
fn count() {
    let x: Vec<u32> = my_vec![42;2 ];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}
#[test]
fn test_max_value() {
    assert_eq!(u16::max_value(), u16::MAX);
    assert_eq!(u32::max_value(), u32::MAX);
    assert_eq!(u64::max_value(), u64::MAX);
}

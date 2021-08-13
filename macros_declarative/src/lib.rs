#[macro_export] // pub formacro
macro_rules! my_vec{
    () => {
        Vec::new()
    };
    ($($element: expr),*) => {
        {
            let mut vs = Vec::new();
            $(
                vs.push($element);
            )*
            vs
        }
    };
    ($($element: expr,)*) => {
        {
            crate::my_vec![$($element),*]
        }
    };
    ($element:expr; $count:expr) => {
        {
            let mut vs = Vec::new();
            vs.resize($count, $element);
            vs
        }
    }
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
    let x: Vec<u32> = my_vec![2, 3, ];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 2);
    assert_eq!(x[1], 3);
}

#[test]
fn count(){
    let x: Vec<u32> = my_vec![42;2 ];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}
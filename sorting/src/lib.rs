pub mod bubblesort;
pub mod insertion;
pub mod selection;
pub mod quicksort;

/// Trait that sorts the slice.
pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

// /// Function that sorts the slice
// fn sort<T, S>(slice: &mut [T])
// where
//     T: Ord,
//     S: Sorter,
// {
//     S.sort(slice)
// }

#[cfg(test)]
mod tests {
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort()
        }
    }

    use super::*;

    #[test]
    fn std_works() {
        let mut v = vec![4, 2, 3, 1];
        StdSorter.sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4]);
    }
}

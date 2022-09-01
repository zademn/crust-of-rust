use crate::Sorter;

pub struct InsertionSort {
    smart: bool,
}

impl Sorter for InsertionSort {
    /// Split into /[sorted | not sorted]
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for i in 1..slice.len() {
            // slice[i..] is not sorted.
            // take slice[i] and place it in to sorted location.
            // Walk backwards from `i` and swap until we get to the right location.
            if !self.smart {
                let mut j = i;
                while j > 0 && slice[j - 1] > slice[j] {
                    slice.swap(j - 1, j);
                    j -= 1;
                }
            } else {
                // Binary search in the ordered part of the array.
                let j = match slice[..i].binary_search(&slice[i]) {
                    // [1, 3, 5].binary_search(3) => Ok(1)
                    Ok(j) => j,
                    // [1, 3, 5].binary_search(2) => Err(1)
                    Err(j) => j,
                };
                // Rotate the ordered part starting from the found index.
                slice[j..=i].rotate_right(1);
            }
        }
    }
}

#[test]
fn insertion_works() {
    let mut v = vec![5, 4, 2, 3, 1];
    InsertionSort { smart: true }.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);

    let mut v = vec![5, 4, 2, 3, 1];
    InsertionSort { smart: false }.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}

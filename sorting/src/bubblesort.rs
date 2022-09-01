use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    /// Walk the array and swap consecutive elements if they are not in order
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        // Keep looping until we don't swap anything.
        while swapped {
            swapped = false;
            for i in 0..slice.len() - 1 {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true
                }
            }
        }
    }
}

#[test]
fn bubble_works() {
    let mut v = vec![5, 4, 2, 3, 1];
    BubbleSort.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}

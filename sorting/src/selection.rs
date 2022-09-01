use crate::Sorter;

pub struct Selection;

impl Sorter for Selection {
    /// Split into /[sorted | not sorted]
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // Find the smallest element of the list and stick it to the front.
        //  Repeat from the remainder of the list
        for i in 0..slice.len() {
            // start from current index
            let mut smallest_idx = i;
            // If we find one smaller save the idx. 
            for j in (i + 1)..slice.len() {
                if slice[j] < slice[smallest_idx] {
                    smallest_idx = j
                }
            }
            if i != smallest_idx {
                slice.swap(i, smallest_idx);
            }
        }
    }
}

#[test]
fn selection_works() {
    let mut v = vec![5, 4, 2, 3, 1];
    Selection.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}

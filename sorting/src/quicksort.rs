use super::Sorter;

fn quicksort<T: Ord>(slice: &mut [T]) {
    // Stop conditions.
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }
    // Steal the first element of the slice.
    // `split_at_mut` gives back 2 mutable slices. [pivot] and rest
    let (pivot, rest) = slice.split_first_mut().expect("slice is not empty");
    let mut left_idx = 0;
    let mut right_idx = rest.len() - 1;

    while left_idx <= right_idx {
        if &rest[left_idx] <= pivot {
            // already on the correct side
            left_idx += 1; // move forward
        } else if &rest[right_idx] > pivot {
            // Right is already on the correct side.
            // Avoid unnecessary swaps back and forth.
            if right_idx == 0 {
                // we must be done
                break;
            }
            right_idx -= 1;
        } else {
            // move element to the right side.
            rest.swap(left_idx, right_idx);
            left_idx += 1; // move forward.
            right_idx -= 1; // move backward.
        }
    }
    // Make left_idx and right_idx point into `slice` not `left`.
    let left_idx = left_idx + 1;
    // Place the pivot at its final location
    slice.swap(0, left_idx - 1);
    // split_at_mut(mid: usize) -> (&mut [..mid], &mut [mid..])
    let (left, right) = slice.split_at_mut(left_idx - 1);
    assert!(left.last() <= right.first());
    quicksort(left);
    quicksort(&mut right[1..]);

    // Pseudocode with allocations.
    // let mut left = vec![];
    // let mut right = vec![];

    // for i in 0..slice.len() {
    //     if slice[i] <= pivot {
    //         left.push(slice[i]);
    //     } else {
    //         right.push(slice[i]);
    //     }
    // }
    // quicksort(&left);
    // quicksort(&right);
    // Then merge them together

}
pub struct QuickSort;

impl Sorter for QuickSort {
    /// Walk the array and swap consecutive elements if they are not in order
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // Pivot can be chosen randomly.
        // [unsorted | pivot | unsorted]
        quicksort(slice);
    }
}

#[test]
fn quick_works() {
    let mut v = vec![5, 4, 2, 3, 1];
    QuickSort.sort(&mut v);
    assert_eq!(v, &[1, 2, 3, 4, 5]);
}

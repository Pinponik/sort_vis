//! # src/sorting.rs
//!
//! The module with sorting algorithms.

/// # `quicksort`.
///
/// Use `quicksort::sort` to sort a `SortingVec` using the quicksort algorithm.
pub mod quicksort {
    /// The sorting fn for quicksort.
    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();
        if len <= 1 {
            return;
        }

        let pivot_index = partition(slice);

        // Rekurencyjne sortowanie lewej i prawej strony
        sort(&mut slice.sub(0..pivot_index));
        sort(&mut slice.sub(pivot_index + 1..len));
    }

    fn partition(slice: &mut crate::SortingVec) -> usize {
        let len = slice.len();
        let pivot_index = len / 2; // Wybór środkowego elementu jako pivot
        slice.swap(pivot_index, len - 1);

        let mut i = 0;
        for j in 0..len - 1 {
            if slice.data.read().unwrap()[slice.start + j]
                <= slice.data.read().unwrap()[slice.start + len - 1]
            {
                slice.swap(i, j);
                i += 1;
            }
        }

        slice.swap(i, len - 1);
        i
    }
}

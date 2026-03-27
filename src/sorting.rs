pub mod quicksort {
    pub fn sort<T: PartialOrd>(slice: &mut crate::SortingVec) {
        if slice.len() <= 1 {
            return;
        }

        let pivot_index = partition(slice);

        // Rekurencyjne sortowanie lewej i prawej strony
        sort(&mut slice[0..pivot_index]);
        sort(&mut slice[pivot_index + 1..]);
    }

    fn partition<T: PartialOrd>(slice: &mut [T]) -> usize {
        let len = slice.len();
        let pivot_index = len / 2; // Wybór środkowego elementu jako pivot
        slice.swap(pivot_index, len - 1);

        let mut i = 0;
        for j in 0..len - 1 {
            if slice[j] <= slice[len - 1] {
                slice.swap(i, j);
                i += 1;
            }
        }

        slice.swap(i, len - 1);
        i
    }
}

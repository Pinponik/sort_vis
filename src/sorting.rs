pub mod quicksort {
    use std::thread;
    use std::time::Duration;

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
        thread::sleep(Duration::from_millis(50));

        let mut i = 0;
        for j in 0..len - 1 {
            if slice.data.read().unwrap()[slice.start + j]
                <= slice.data.read().unwrap()[slice.start + len - 1]
            {
                slice.swap(i, j);
                thread::sleep(Duration::from_millis(50));
                i += 1;
            }
        }

        slice.swap(i, len - 1);
        thread::sleep(Duration::from_millis(50));
        i
    }
}

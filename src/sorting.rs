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
        sort(&mut slice.sub(0..pivot_index, true));
        sort(&mut slice.sub(pivot_index + 1..len, true));
    }

    fn partition(slice: &mut crate::SortingVec) -> usize {
        let len = slice.len();
        let pivot_index = len / 2;
        slice.swap(pivot_index, len - 1);
        thread::sleep(Duration::from_millis(50));

        let mut i = 0;
        for j in 0..len - 1 {
            if slice.get(j) <= slice.get(len - 1) {
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

pub mod insertion_sort {
    use std::thread;
    use std::time::Duration;

    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();

        for i in 1..len {
            let key = slice.get(i);
            let mut j = i;

            while j > 0 && slice.get(j - 1) > key {
                slice.swap(j - 1, j);
                j -= 1;
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

pub mod binary_insertion_sort {
    use std::thread;
    use std::time::Duration;

    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();

        for i in 1..len {
            let key = slice.get(i);
            let pos = binary_search(slice, 0, i, key);

            for j in (pos..i).rev() {
                slice.swap(j, j + 1);
                thread::sleep(Duration::from_millis(50));
            }
        }
    }

    fn binary_search(slice: &crate::SortingVec, left: usize, right: usize, key: u16) -> usize {
        let mut l = left;
        let mut r = right;

        while l < r {
            let mid = (l + r) / 2;
            if slice.get(mid) > key {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}

pub mod selection_sort {
    use std::thread;
    use std::time::Duration;

    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();

        for i in 0..len - 1 {
            let mut min_idx = i;
            for j in i + 1..len {
                if slice.get(j) < slice.get(min_idx) {
                    min_idx = j;
                }
            }
            if min_idx != i {
                slice.swap(i, min_idx);
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

pub mod bubble_sort {
    use std::thread;
    use std::time::Duration;

    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();

        for i in 0..len {
            for j in 0..len - i - 1 {
                if slice.get(j) > slice.get(j + 1) {
                    slice.swap(j, j + 1);
                    thread::sleep(Duration::from_millis(50));
                }
            }
        }
    }
}

pub mod bubble_sort_optimized {
    use std::thread;
    use std::time::Duration;

    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();

        for i in 0..len {
            let mut swapped = false;
            for j in 0..len - i - 1 {
                if slice.get(j) > slice.get(j + 1) {
                    slice.swap(j, j + 1);
                    swapped = true;
                    thread::sleep(Duration::from_millis(50));
                }
            }
            if !swapped {
                break;
            }
        }
    }
}

pub mod cocktail_sort {
    use std::thread;
    use std::time::Duration;

    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();
        let mut left = 0;
        let mut right = len - 1;

        while left < right {
            for j in left..right {
                if slice.get(j) > slice.get(j + 1) {
                    slice.swap(j, j + 1);
                    thread::sleep(Duration::from_millis(50));
                }
            }
            right -= 1;

            for j in (left..right).rev() {
                if slice.get(j) > slice.get(j + 1) {
                    slice.swap(j, j + 1);
                    thread::sleep(Duration::from_millis(50));
                }
            }
            left += 1;
        }
    }
}

pub mod shell_sort {
    use std::thread;
    use std::time::Duration;

    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();
        let mut gap = len / 2;

        while gap > 0 {
            for i in gap..len {
                let key = slice.get(i);
                let mut j = i;

                while j >= gap && slice.get(j - gap) > key {
                    slice.swap(j - gap, j);
                    j -= gap;
                    thread::sleep(Duration::from_millis(50));
                }
            }
            gap /= 2;
        }
    }
}

pub mod heap_sort {
    use std::thread;
    use std::time::Duration;

    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();

        for i in (0..len / 2).rev() {
            heapify(slice, len, i);
        }

        for i in (1..len).rev() {
            slice.swap(0, i);
            thread::sleep(Duration::from_millis(50));
            heapify(slice, i, 0);
        }
    }

    fn heapify(slice: &mut crate::SortingVec, n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && slice.get(left) > slice.get(largest) {
            largest = left;
        }
        if right < n && slice.get(right) > slice.get(largest) {
            largest = right;
        }

        if largest != i {
            slice.swap(i, largest);
            thread::sleep(Duration::from_millis(50));
            heapify(slice, n, largest);
        }
    }
}

pub mod merge_sort {
    use std::thread;
    use std::time::Duration;

    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();
        if len <= 1 {
            return;
        }

        let mid = len / 2;
        sort(&mut slice.sub(0..mid, true));
        sort(&mut slice.sub(mid..len, true));
        merge(slice, 0, mid, len);
    }

    fn merge(slice: &mut crate::SortingVec, left: usize, mid: usize, right: usize) {
        let mut temp = vec![];

        let mut i = left;
        let mut j = mid;

        while i < mid && j < right {
            if slice.get(i) <= slice.get(j) {
                temp.push(slice.get(i));
                i += 1;
            } else {
                temp.push(slice.get(j));
                j += 1;
            }
        }

        while i < mid {
            temp.push(slice.get(i));
            i += 1;
        }

        while j < right {
            temp.push(slice.get(j));
            j += 1;
        }

        for (k, &val) in temp.iter().enumerate() {
            slice.data.write().unwrap()[slice.start + left + k] = val;
            thread::sleep(Duration::from_millis(50));
        }
    }
}

pub mod counting_sort {
    use std::thread;
    use std::time::Duration;

    pub fn sort(slice: &mut crate::SortingVec) {
        let len = slice.len();
        if len == 0 {
            return;
        }

        let mut max_val = 0u16;
        for i in 0..len {
            let val = slice.get(i);
            if val > max_val {
                max_val = val;
            }
        }
        let max = max_val as usize;

        let mut count = vec![0; max + 1];

        for i in 0..len {
            let val = slice.get(i) as usize;
            count[val] += 1;
        }

        let mut idx = 0;
        for num in 0..=max {
            while count[num] > 0 {
                slice.data.write().unwrap()[slice.start + idx] = num as u16;
                thread::sleep(Duration::from_millis(50));
                idx += 1;
                count[num] -= 1;
            }
        }
    }
}

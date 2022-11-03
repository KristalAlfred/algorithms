use std::cmp::Ord;

pub fn sort<T: Ord + std::fmt::Debug>(collection: &mut [T]) {
    if collection.is_empty() {
        return;
    }

    let mut iter_max = collection.len() - 1;
    loop {
        let mut swaps = 0;
        for i in 0..iter_max {
            if collection[i] > collection[i + 1] {
                collection.swap(i, i + 1);
                swaps += 1;
            }
        }

        if swaps == 0 {
            break;
        }
        iter_max -= 1;
    }
}
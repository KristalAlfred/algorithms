/**
 * Bubble sort is an algorithm that compares the adjacent elements and 
 * swaps their positions if they are not in the intended order. The
 * pass through the unsorted portion of the data is repeated until no
 * swaps are needed, which indicates that the data is sorted.
*/

use std::cmp::Ord;

pub fn sort<T: Ord>(collection: &mut [T]) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T>(collection: &[T]) -> bool 
    where
        T: Ord,
    {
        collection.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];
        sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn one_element() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn already_sorted() {
        let mut vec = vec![1, 2, 3];
        sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn reverse_sorted() {
        let mut vec = vec![3, 2, 1];
        sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn random() {
        let mut vec = vec![3, 2, 1, 5, 4];
        sort(&mut vec);
        assert!(is_sorted(&vec));
    }
}
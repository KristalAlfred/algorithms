/**
 * Quick sort is a sorting algorithm that uses a divide and conquer strategy.
 * It works by selecting a 'pivot' element from the collection and partitioning
 * the other elements into two subcollections, according to whether they are
 * less than or greater than the pivot. The subcollections are then sorted
 * recursively. This can be done in-place, requiring small additional amounts
 * of memory to perform the sorting. This implementation, however, copies the 
 * pivot element once per recursion.
 */

use std::cmp::Ord;

pub fn sort<T>(collection: &mut [T])
where
    T: Ord + Copy,
{
    if collection.len() <= 1 {
        return;
    }

    // Use last element as the pivot element.
    let pivot = collection[collection.len() - 1];

    let mut cursor = 0;
    let mut smaller_than_pivot = 0;
    let mut larger_than_pivot = collection.len() - 1;

    loop {
        if collection[cursor] < pivot {
            if cursor > smaller_than_pivot {
                collection.swap(cursor, smaller_than_pivot + 1);
                smaller_than_pivot += 1;
            }
            cursor += 1;
        } else if collection[cursor] == pivot {
            cursor += 1;
        } else if collection[cursor] > pivot {
            // If the element is larger, swap it with the next from the back.
            // In this case, we do not want to increment the cursor!
            collection.swap(cursor, larger_than_pivot);
            larger_than_pivot -= 1;
        }
        if cursor >= larger_than_pivot {
            break;
        }
    }

    let (less_than, larger_than) = collection.split_at_mut(larger_than_pivot);
    sort(less_than);
    sort(larger_than);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort;

    fn is_sorted<T>(collection: &[T]) -> bool
    where
        T: Ord,
    {
        collection.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn random() {
        let mut vec = vec![1, 4, 2, 9, 5, 2, 1, 8, 3, 3, 6, 6, 7, 3, 2];
        sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];
        sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn one() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn two() {
        let mut vec = vec![1, 2];
        sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn two_reverse() {
        let mut vec = vec![2, 1];
        sort(&mut vec);
        assert!(is_sorted(&vec));
    }
}

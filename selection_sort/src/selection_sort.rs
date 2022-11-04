use std::cmp::Ord;

pub fn sort<T>(collection: &mut [T])
where T: Ord + std::fmt::Debug
{
    let mut index = 0;
    let mut sorted_index = 0;

    while sorted_index < collection.len() {
        let mut smallest = sorted_index;

        while index < collection.len() {
            if collection[index] < collection[smallest] {
                smallest = index;
            }
            index += 1;
        }
        collection.swap(smallest, sorted_index);
        sorted_index += 1;
        index = sorted_index;
    }
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
        let mut v = vec![1, 3, 2, 5, 3, 6, 8, 3, 32, 362, 643, 743, 43, 4, 3, 2, 1, 0];
        sort(&mut v);
        assert!(is_sorted(&v));
        println!("{:?}", v);
    }

    #[test]
    fn empty() {
        let mut v: Vec<i32> = vec![];
        sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn one() {
        let mut v = vec![1];
        sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn two() {
        let mut v = vec![2, 1];
        sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn reverse_sorted() {
        let mut v = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn same_elements() {
        let mut v = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        sort(&mut v);
        assert!(is_sorted(&v));
    }
}
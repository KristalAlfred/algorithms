pub fn sort<T>(collection: &mut [T])
where
    T: Ord,
{
    let len = collection.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && collection[j] < collection[j - 1] {
            collection.swap(j - 1, j);
            j -= 1;
        }
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
    fn random() {
        let mut collection = [4, 2, 5, 3, 1, 6, 7, 11, 21, 9, 8, 10];
        sort(&mut collection);
        assert!(is_sorted(&collection));
    }

    #[test]
    fn empty() {
        let mut collection: [i32; 0] = [];
        sort(&mut collection);
        assert!(is_sorted(&collection));
    }

    #[test]
    fn already_sorted() {
        let mut collection = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        sort(&mut collection);
        assert!(is_sorted(&collection));
    }

    #[test]
    fn reverse_sorted() {
        let mut collection = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        sort(&mut collection);
        assert!(is_sorted(&collection));
    }

    #[test]
    fn same_element() {
        let mut collection = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        sort(&mut collection);
        assert!(is_sorted(&collection));
    }
}
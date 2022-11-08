fn sort<T>(collection: &mut [T])
    where T: Ord
{
    // ...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random() {
        let mut collection = [4, 2, 5, 3, 1];
        sort(&mut collection);
        assert_eq!([1, 2, 3, 4, 5], collection);
    }
}
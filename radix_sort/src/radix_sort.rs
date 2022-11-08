use std::ops::{Rem, Div};

pub fn sort<T>(collection: &mut [T])
    where T: Rem<i32, Output = i32> + Div<i32, Output = i32> + Copy
{
    let mut iteration = 1;
    let mut last_iteration = false;
    loop {
        // Divide the collection into buckets
        let mut buckets = vec![0; 10];
        for item in collection.iter() {
            let bucket = ((*item / iteration) % 10) as usize;
            buckets[bucket] += 1;
        }

        if buckets[0] == collection.len() {
            last_iteration = true;
        }

        // Calculate the last index for each bucket
        for i in 1..buckets.len() {
            buckets[i] += buckets[i - 1];
        }
        
        for item in collection.to_owned().iter().rev() {
            let bucket = ((*item / iteration) % 10) as usize;
            buckets[bucket] -= 1;
            collection[buckets[bucket]] = *item;
        }

        iteration *= 10;
        if last_iteration {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random() {
        let mut collection = vec![1, 5124, 323, 21111, 4];
        sort(&mut collection);
        assert_eq!(collection, vec![1, 4, 323, 5124, 21111]);
    }

    #[test]
    fn already_sorted() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        sort(&mut v);
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn reversed() {
        let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        sort(&mut v);
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
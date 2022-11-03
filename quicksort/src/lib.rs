use std::cmp::Ord;

pub fn sort<T>(collection: &mut [T]) 
where 
    T: Ord + std::fmt::Debug,
{
    if collection.is_empty() {
        return;
    }

    // Use last element as the pivot element.
    let pivot_index = collection.len() - 1;

    let mut front = 0;
    let mut back = collection.len() - 1;

    loop {
        while collection[front] < collection[pivot_index] {
            front += 1;
        }

        while collection[back] > collection[pivot_index] {
            back -= 1;
        }

        if front >= back {
            break;
        } else {
            collection.swap(front, back);
            front += 1;
            back -= 1;
        }
        println!("Front: {front}, Back: {back}");
        println!("{:?}", collection);
    }
}

#[cfg(test)]
mod tests {
    use crate::sort;

    #[test]
    fn it_works() {
        let mut vec = vec![1, 4, 6, 7, 3, 3, 5, 6, 5];
        sort(&mut vec);
    }
}
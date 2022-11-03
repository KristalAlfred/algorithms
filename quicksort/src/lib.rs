use std::cmp::Ord;

pub fn sort<T>(collection: &mut [T]) 
where 
    T: Ord + std::fmt::Debug + Copy,
{
    if collection.is_empty() {
        return;
    }

    // Use last element as the pivot element.
    let mut pivot_index = collection.len() - 1;
    let pivot = collection[pivot_index];

    let mut cursor = 0;
    let mut less_than_index = 0;
    let mut larger_than_index = collection.len() - 1;

    loop {
        let test = &collection[cursor];
        println!("Next element: {:?}", test);
        if collection[cursor] < pivot {
            println!("Less than! This needs to go to the front.");
            if cursor <= less_than_index {
                // If the element is already in the start, it's great. Do nothing.
            } else {
                collection.swap(cursor, less_than_index + 1);
                less_than_index += 1;
            }
            cursor += 1;

        } else if collection[cursor] == pivot {
            println!("Equal to pivot! Moving on..");
            cursor += 1;
        } else if collection[cursor] > pivot {
            // If the element is larger than the pivot, move it to the back and
            // don't advance the cursor.
            println!("Larger! Swapping to the back..");
            collection.swap(cursor, larger_than_index);
            larger_than_index -= 1;
        }
        println!("{:?}", collection);
        if cursor >= larger_than_index {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort;

    #[test]
    fn it_works() {
        let mut vec = vec![1, 4, 2, 9, 5, 2, 1, 8, 3, 3, 6, 6, 7, 3, 5];
        sort(&mut vec);
    }
}
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
    let mut less_than_index = 0;
    let mut larger_than_index = collection.len() - 1;

    loop {
        if collection[cursor] < pivot {
            if cursor > less_than_index {
                collection.swap(cursor, less_than_index + 1);
                less_than_index += 1;
            }
            cursor += 1;
        } else if collection[cursor] == pivot {
            cursor += 1;
        } else if collection[cursor] > pivot {
            // If the element is larger, swap it with the next from the back.
            // In this case, we do not want to increment the cursor!
            collection.swap(cursor, larger_than_index);
            larger_than_index -= 1;
        }
        if cursor >= larger_than_index {
            break;
        }
    }

    let (less_than, larger_than) = collection.split_at_mut(less_than_index + 1);
    sort(less_than);
    sort(larger_than);
}

#[cfg(test)]
mod tests {
    use crate::sort;

    #[test]
    fn random() {
        let mut vec = vec![1, 4, 2, 9, 5, 2, 1, 8, 3, 3, 6, 6, 7, 3, 2];
        sort(&mut vec);
        println!("{:?}", vec);
    }
}
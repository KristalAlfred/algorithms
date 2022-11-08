pub fn sort<T>(collection: &mut [T])
    where T: Ord + std::fmt::Debug
{
    if collection.len() > 1 {
        let (front, back) = collection.split_at_mut(collection.len() / 2);
        sort(front);
        sort(back);
    } else {
        return;
    }

    let mut front_index = 0;
    let mut back_index = 0;
    let mid_point = collection.len() / 2;

    let mut sorted: Vec<&T> = Vec::with_capacity(collection.len());

    println!("Pre sorted: {:?}", collection);
    // Merge front and back into sorted.
    for _ in 0..collection.len() {
        if mid_point + back_index == collection.len() {
            sorted.push(&collection[front_index]);
            front_index += 1;
            continue;
        }
        if collection[front_index] < collection[mid_point + back_index] {
            sorted.push(&collection[front_index]);
            front_index += 1;
        } else {
            sorted.push(&collection[mid_point + back_index]);
            back_index += 1;
        }
    }
    println!("front_index: {}, back_index: {}, mid_point: {}", front_index, back_index, mid_point);
    println!("Post sorted: {:?}", sorted);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut collection = vec![5, 4, 1, 3, 2];
        sort(&mut collection);
        assert_eq!(collection, vec![1, 2, 3, 4, 5]);
    }
}


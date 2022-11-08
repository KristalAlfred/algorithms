pub fn sort<T>(collection: &mut [T])
    where T: Ord + Copy
{
    if collection.len() > 1 {
        let (front, back) = collection.split_at_mut(collection.len() / 2);
        sort(front);
        sort(back);
    } else {
        return;
    }
    let mid = collection.len() / 2;

    let front = collection[..mid].to_vec();
    let back = collection[mid..].to_vec();
    let mut left = 0;
    let mut right = 0;

    for i in 0..collection.len() {
        if left < front.len() && right < back.len() {
            if front[left] < back[right] {
                collection[i] = front[left];
                left += 1;
            } else {
                collection[i] = back[right];
                right += 1;
            }
        } else if left < front.len() {
            collection[i] = front[left];
            left += 1;
        } else if right < back.len() {
            collection[i] = back[right];
            right += 1;
        }
    }
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


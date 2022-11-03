use bubble_sort::sort;

fn main() {
    let mut v = vec![5, 4, 8, 4, 3, -1, 3, 2, 1];
    let mut x: Vec<i32> = vec![];
    sort(&mut v);
    sort(&mut x);
    println!("{:?}", v);
    println!("{:?}", x);
}

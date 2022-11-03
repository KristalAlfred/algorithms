use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bubble_sort::sort;

fn criterion_benchmark(c: &mut Criterion) {
    let mut vec = vec![1, 4, 2, 9, 5, 2, 1, 8, 3, 3, 6, 6, 7, 3, 2];
    c.bench_function("quicksort", |b| b.iter(|| sort(black_box(&mut vec))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
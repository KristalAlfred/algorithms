use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bubble_sort::sort;
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut vec = (0..5000).map(|_| rng.gen_range(0, 1000)).collect::<Vec<_>>();
    c.bench_function("bubbles-sort", |b| b.iter(|| sort(black_box(&mut vec))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
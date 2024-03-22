use standard_library::sort;
use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers = vec![1, 3, 5, 7, 9, 11, 2, 4, 6, 8, 10];

    c.bench_function("Sort Algorithm", |b| {
        b.iter(|| sort(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);




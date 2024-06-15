#[macro_use]
extern crate criterion;
use criterion::Criterion;
use mytest::fab;

fn criterion_benchmark(c: &mut Criterion) {
    Criterion::default().bench_function("fib 20", |b| b.iter(|| fab(20)));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

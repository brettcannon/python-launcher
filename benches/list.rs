use criterion::{criterion_group, criterion_main, Criterion};
use python_launcher;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("List executables", |b| {
        b.iter(|| python_launcher::all_executables())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

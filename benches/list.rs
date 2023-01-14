use std::hint;

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("List executables", |b| {
        b.iter(|| {
            let executables = python_launcher::all_executables();
            hint::black_box(executables);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

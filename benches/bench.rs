use criterion::{black_box, criterion_group, criterion_main, Criterion};

use perf_log::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("log1", |b| b.iter(|| log_1("asdasd", |s| { black_box(s); } )));
    c.bench_function("log2", |b| b.iter(|| log_2("asdasd", |s| { black_box(s); } )));
    c.bench_function("log3", |b| b.iter(|| log_3("asdasd", |s| { black_box(s); } )));
    c.bench_function("log4", |b| b.iter(|| log_4("asdasd", |s| { black_box(s); } )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

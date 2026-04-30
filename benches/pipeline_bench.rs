use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use stock_pipeline::{run_aggregation, run_aggregation_parallel};

fn bench_full_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("stock_pipeline");
    group.throughput(Throughput::Elements(100 * 22500));

    group.bench_function("hashmap_sequential", |b| {
        b.iter(|| run_aggregation("stock_data.bin").unwrap())
    });

    group.bench_function("hashmap_parallel", |b| {
        b.iter(|| run_aggregation_parallel("stock_data.bin").unwrap())
    });

    group.finish();
}

criterion_group!(benches, bench_full_pipeline);
criterion_main!(benches);

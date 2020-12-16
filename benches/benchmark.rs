use criterion::{criterion_group, criterion_main, Criterion};
use rust_remote_desktop::main::{get_edge_paths, read_png};

fn benchmark(c: &mut Criterion) {
    let image = read_png("input.png");
    c.bench_function("get edge paths", |b| b.iter(|| get_edge_paths(&image, 0..image.width, 0..image.height)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
use criterion::{criterion_group, criterion_main, Criterion};
use s7::algorithm::encode::encode;
use s7::read_png::read_png;

fn benchmark(c: &mut Criterion) {
    let image = read_png("input/input.png");

    c.bench_function("get edge paths", |b| {
        b.iter_with_setup(|| image.clone(), |img| encode(img))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

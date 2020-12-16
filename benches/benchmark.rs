use criterion::{criterion_group, criterion_main, Criterion, BenchmarkGroup};
use rust_remote_desktop::main::{get_edge_paths, read_png};

fn benchmark(c: &mut Criterion) {
    let mut image = read_png("input/input.png");

    let mut group = c.benchmark_group("bench group");
    group.sample_size(10);

    let width = image.width;
    let height = image.height;

    group.bench_function(
        "get edge paths", 
        |b| b.iter(|| 
            get_edge_paths(&mut image, 0..width, 0..height)
        )
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use movingai::Map3D;
use movingai::parser::{parse_3dmap, parse_3dmap_file};
use std::fs;
use std::path::Path;

fn parse_3dmap_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("tests/A1.3dmap").expect("failed to read A1.3dmap");
    let mut group = c.benchmark_group("3dmap_parsing");
    group.throughput(Throughput::Bytes(contents.len() as u64));
    group.bench_function("A1_from_str", |b| {
        b.iter(|| parse_3dmap(black_box(&contents)).expect("failed to parse A1.3dmap"));
    });
    group.finish();
}

fn octree_query_benchmark(c: &mut Criterion) {
    let map = parse_3dmap_file(Path::new("tests/A1.3dmap")).expect("failed to parse A1.3dmap");
    let coords = (101, 109, 191);

    c.bench_function("octree_free_neighbors", |b| {
        b.iter(|| black_box(map.neighbors(black_box(coords))));
    });
    c.bench_function("octree_traversable_transition", |b| {
        b.iter(|| {
            black_box(map.is_traversable_from(black_box(coords), black_box((102, 109, 191))))
        });
    });
}

criterion_group!(
    octree_benches,
    parse_3dmap_benchmark,
    octree_query_benchmark
);
criterion_main!(octree_benches);

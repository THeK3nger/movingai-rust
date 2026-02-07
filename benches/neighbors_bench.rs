use criterion::{black_box, criterion_group, criterion_main, Criterion};
use movingai::{parser::parse_map_file, Map2D};
use std::path::Path;

fn neighbors_benchmark(c: &mut Criterion) {
    // Load the large test map
    let map = parse_map_file(Path::new("tests/maze512-32-9.map")).expect("Failed to load test map");

    // Collect all traversable coordinates once
    let traversable_coords: Vec<_> = map
        .coords()
        .filter(|&coord| map.is_traversable(coord))
        .collect();

    c.bench_function("neighbors_all_traversable", |b| {
        b.iter(|| {
            // Call neighbors() on every traversable tile
            for &coord in &traversable_coords {
                let neighbors = map.neighbors(black_box(coord));
                black_box(neighbors);
            }
        });
    });

    // Also benchmark a single typical call
    if let Some(&sample_coord) = traversable_coords.first() {
        c.bench_function("neighbors_single_call", |b| {
            b.iter(|| {
                let neighbors = map.neighbors(black_box(sample_coord));
                black_box(neighbors);
            });
        });
    }
}

criterion_group!(benches, neighbors_benchmark);
criterion_main!(benches);

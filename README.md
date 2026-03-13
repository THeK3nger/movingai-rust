# movingai-rust

![Cargo Version](https://img.shields.io/crates/v/movingai.svg)
[![](https://tokei.rs/b1/github/THeK3nger/movingai-rust)](https://github.com/THeK3nger/movingai-rust)

Map/Scenario Parser for the [MovingAI benchmark](http://www.movingai.com/benchmarks) format. It offers a quick way to parse scenario and map files, with the addition of some utilities to manage and query information from the maps.

## Features

The crate parses map and scene files and provides several functions for easy interaction and query.

### 2D Maps

- Easy idiomatic access to the map data such as width, height and tiles at a specific coordinate.
- Check if a tile is traversable or not according the MovingAI format rules.
- Get the list of accessible neighbors from a specific tile.
- `map_type` exposed as a `MapType` enum (`Octile`, `FourConnected`).
- [TO DO] Convert bitmaps into `.map` files.
- Serialize/Deserialize `.map` and `.scen` files into JSON/YAML using serde (activate `--features serde`)

### 3D Maps (Voxel/Octree)

- Parse `.3dmap` and `.3dscen` benchmark files.
- `VoxelMap` type backed by an octree for memory-efficient 3D voxel maps.
- `Map3D` trait with `is_traversable`, `is_out_of_bounds`, and `neighbors` for 26-connected grids.
- Utility methods to build obstacle geometry (`create_box_obstacle`, `create_sphere_obstacle`).
- Serialize/Deserialize 3D types with serde (activate `--features serde`)

## How to use

### 2D Maps

```rust
use std::path::Path;
use movingai::parser::parse_map_file;

fn main() {
    let map = parse_map_file(Path::new("./tests/arena.map")).unwrap();
    let width = map.width();
    let tile = map[(4,5)]; // Access map location at row 4 and column 5.
}
```

As an example, you can see how we can use this crate to easily implement the A\* pathfinding algorithm.

```rust
// A* shortest path algorithm.

fn shortest_path(map: &MovingAiMap, start: Coords2D, goal: Coords2D) -> Option<f64> {

    let mut heap = BinaryHeap::new();
    let mut visited = Vec::<Coords2D>::new();

    // We're at `start`, with a zero cost
    heap.push(SearchNode { f: 0.0, g:0.0, h: distance(start, goal), current: start });

    while let Some(SearchNode { f: _f, g, h: _h, current }) = heap.pop() {

        if current == goal { return Some(g); }

        if visited.contains(&current) {
            continue;
        }

        visited.push(current);

        for neigh in map.neighbors(current) {
            let new_h = distance(neigh, goal);
            let i = distance(neigh, current);
            let next = SearchNode { f: g+i+new_h, g: g+i, h: new_h, current: neigh };
            heap.push(next);
        }
    }

    // Goal not reachable
    None
}
```

And in this example we can see how to write a benchmark over a scen file.

```rust
fn main() {
    let map = parse_map_file(Path::new("./tests/arena.map")).unwrap();
    let scenes = parse_scen_file(Path::new("./tests/arena.map.scen")).unwrap();
    for scene in scenes {
        let start = scene.start_pos;
        let goal = scene.goal_pos;
        let t = Instant::now();
        match shortest_path(&map, (1,3), (4,3)) {
            Some(x) => {
                let duration = t.elapsed();
                let seconds = duration.as_secs();
                let ms = (duration.subsec_nanos() as f64) / 1_000_000.0;
                println!("{:?} -> {:?} \tin {:?} seconds and {:?} ms", start, goal, seconds, ms);
            }
            None => println!("None"),
        }
    }
}
```

### 3D Maps

```rust
use std::path::Path;
use movingai::parser::{parse_3dmap_file, parse_3dscen_file};
use movingai::{Coords3D, Map3D, VoxelMap, VoxelState};

fn main() {
    let map = parse_3dmap_file(Path::new("./tests/A1.3dmap")).unwrap();
    let scenes = parse_3dscen_file(Path::new("./tests/A1.3dmap.3dscen")).unwrap();

    for scene in &scenes {
        let start = scene.start_pos;
        let goal = scene.goal_pos;
        println!("Optimal length: {}", scene.optimal_length);
        // map.neighbors(start) returns all 26-connected free voxels
        let neighbors = map.neighbors(start);
    }
}
```

You can also build a `VoxelMap` programmatically:

```rust
use movingai::{Coords3D, VoxelMap, VoxelState};

fn main() {
    let mut vmap = VoxelMap::new(64, (0, 0, 0), VoxelState::Free);
    vmap.create_box_obstacle((10, 10, 10), (20, 20, 20));
    vmap.create_sphere_obstacle((40, 40, 40), 5.0);

    let free_neighbors = vmap.get_free_neighbors((5, 5, 5));
}
```

## Breaking Changes

### Since previous release

- **`is_out_of_bound` renamed to `is_out_of_bounds`** — update all call sites.
- **`map_type` is now a `MapType` enum** — replace string comparisons with `MapType::Octile` / `MapType::FourConnected`.
- **`ParseError` is now a proper `std::error::Error`** — the parser functions now return a structured `ParseError` type instead of a plain string.

## Why `cargo test` is failing?

Note that tests need to be compiled with the `serde` feature enabled.

```sh
cargo test --features serde
```

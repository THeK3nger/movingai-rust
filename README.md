# movingai-rust

![Travis Build](https://api.travis-ci.org/THeK3nger/movingai-rust.svg?branch=master)
![Cargo Version](https://img.shields.io/crates/v/movingai.svg)
[![](https://tokei.rs/b1/github/THeK3nger/movingai-rust)](https://github.com/THeK3nger/movingai-rust)

Map/Scenario Parser for the [MovingAI benchmark](http://www.movingai.com/benchmarks) format. It offers a quick way to parse scenario and map files, with the addition of some utilities to manage and query information from the maps.

## Map Features

The crate parses map and scene files and provides several function for easy interaction and query.

Some of the functionalities are:

- Easy idiomatic access to the map data such as width, height and tiles at a specific coordinate.
- Check if a tile is traversable or not according the MovingAI format rules.
- Get the list of accessible neighbors from a specific tile.
- [TO DO] Convert bitmaps into `.map` files.
- Serialize/Deserialzie `.map` and `.scen` files into JSON/YAML using serde (activate `--features serde`)

## How to use

```rust
extern crate movingai;

use std::path::Path;
use movingai::parser::parse_map_file;

fn main() {
    let map = parse_map_file(Path::new("./test/arena.map")).unwrap();
    let width = map.width();
    let tile = map[(4,5)]; // Access map location at row 4 and column 5.
}
```

As an example, you can see how we can use this crate to easily implement the A* pathfinding algorithm.

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

## Why `cargo test` is failing?

Note that tests need to be compiled with the `serde` feature enabled.

```sh
cargo test --feature=serde
```

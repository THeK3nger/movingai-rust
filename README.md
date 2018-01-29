# movingai-rust
![Travis Build](https://api.travis-ci.org/THeK3nger/movingai-rust.svg?branch=master)
![Cargo Version](https://img.shields.io/crates/v/movingai.svg)

**Still in development! Help is encouraged for stabilizing the API.**

Map/Scenario Parser for the [MovingAI benchmark](http://www.movingai.com/benchmarks) format. It offers a quick way to parse scenario and map files, plus some map utility to manage and query information from them.

## Map Features

The create not only parse a map and scene file, it also provide an object for easy interacting with it.

Some of the functionalities are:

 - Easy idiomatic access to the map date, such as width, height and tiles at specific coordinate.
 - Check if a tile is traversable or not according the MovingAI format rules.
 - Get the list of neighbors coordinates of a specific tile.
 - Convert bitmaps into `.map` files.
 - And more things I still need to decide. :D

## How to use

```rust
extern crate movingai;

use movingai::parser::parse_map_file;

fn main() {
    let map = parse_map_file("./test/arena.map").unwrap();
    let width = map.get_width();
    let tile = map[(4,5)]; // Access map location at row 4 and column 5.
}
```

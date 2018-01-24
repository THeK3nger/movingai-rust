# movingai-rust
![Travis Build](https://api.travis-ci.org/THeK3nger/movingai-rust.svg?branch=master)
![Cargo Version](https://img.shields.io/crates/v/movingai.svg)

**Still a very experimental crate!**

Map/Scenario Parser for the [MovingAI benchmark](http://www.movingai.com/benchmarks) format. It offers a quick way to parse scenario and map files, plus some map utility to manage and query information from them.

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

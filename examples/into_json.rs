use serde_json;


use std::path::Path;

use movingai::parser::parse_map_file;

fn main() {
    let map = parse_map_file(Path::new("./tests/arena.map")).unwrap();

    let serialized = serde_json::to_string(&map).unwrap();
    println!("serialized = {}", serialized);
}
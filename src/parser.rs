/// Contains all the parser functions.

use std::fs::File;
use std::io::prelude::*;
use MovingAiMap;
use SceneRecord;

/// Parse a MovingAI `.map` file.
/// 
/// # Arguments
///  * `path` represents the path to the file location.
///
/// # Returns
///  It returns the parsed map as a `MovingAiMap` or an `Err`.
///
/// # Panics
///  For the time, it panics if the map format it is not correct.
///  TODO: Catch all these errors and encode them into `Result`.
///
/// # Errors
///  Return errors if it is not possible to open the specified file.
///
/// # Examples
///
/// ```
/// use movingai::parser::parse_map_file;
///
/// let map = parse_map_file("./tests/arena.map").unwrap();
/// ```
pub fn parse_map_file(path: &str) -> Result<MovingAiMap, &'static str> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            panic!("Errore opening file {}", err);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut height: usize = 0;
    let mut width: usize = 0;
    let mut map_type: String = String::from("empty");
    let mut map: Vec<char> = Vec::new();

    let mut parse_map = false;
    for line in contents.lines() {
        if parse_map {
            for c in line.chars() {
                map.push(c);
            }
            continue;
        }
        if line.trim() == "map" {
            parse_map = true;
        }
        else {
            let param: Vec<&str> = line.split(" ").collect();
            if param.len() == 2 {
                let key = param[0];
                let value = param[1];
                if key == "type" { map_type = String::from(value); }
                else if key == "height" { height = value.parse::<usize>().unwrap(); }
                else if key == "width" { width = value.parse::<usize>().unwrap(); }
            }
        }
    }
    return Ok(MovingAiMap::new(
        map_type, height, width, map
    ));
}

/// Parse a MovingAI `.scen` file.
/// 
/// # Arguments
///  * `path` represents the path to the file location.
///
/// # Returns
///  It returns the parsed map as a `Vec<SceneRecord>` or an `Err`.
///
/// # Panics
///  For the time, it panics if the map format it is not correct.
///  TODO: Catch all these errors and encode them into `Result`.
///
/// # Errors
///  Return errors if it is not possible to open the specified file.
///
/// # Examples
///
/// ```
/// use movingai::parser::parse_scen_file;
///
/// let scen = parse_scen_file("./tests/arena2.map.scen").unwrap();
/// ```
pub fn parse_scen_file(path: &str) -> Result<Vec<SceneRecord>, &'static str> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            panic!("Errore opening file {}", err);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut table: Vec<SceneRecord> = Vec::new();

    for line in contents.lines() {
        if line.starts_with("version") {
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let record: Vec<&str> = line.split("\t").collect();
        table.push(SceneRecord {
            bucket:  record[0].parse::<u32>().unwrap(),
            map_file: String::from(record[1]),
            map_width: record[2].parse::<usize>().unwrap(),
            map_height: record[3].parse::<usize>().unwrap(),
            start_pos: (record[4].parse::<usize>().unwrap(), record[5].parse::<usize>().unwrap()),
            goal_pos: (record[6].parse::<usize>().unwrap(), record[7].parse::<usize>().unwrap()),
            optimal_length: record[8].parse::<f64>().unwrap()
        })
    }

    return Ok(table);

}

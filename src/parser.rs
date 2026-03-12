#![allow(clippy::tabs_in_doc_comments)]
use crate::map2d::MovingAiMap;
use crate::map2d::SceneRecord;
use crate::map3d::SceneRecord3D;

/// Contains all the parser functions.
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;

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
/// use std::path::Path;
/// use movingai::parser::parse_map_file;
///
/// let map = parse_map_file(Path::new("./tests/arena.map")).unwrap();
/// ```
pub fn parse_map_file(path: &path::Path) -> io::Result<MovingAiMap> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    parse_map(&contents)
}

/// Parse a string representing a MovingAI `.map`.
///
/// # Arguments
///  * `contents` a string in the `.map` format.
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
/// use std::path::Path;
/// use movingai::parser::parse_map;
///
/// let map = parse_map("type octile\nheight 1\nwidth 1\nmap\nT").unwrap();
/// ```
pub fn parse_map(contents: &str) -> io::Result<MovingAiMap> {
    let mut height: usize = 0;
    let mut width: usize = 0;
    let mut map_type = String::from("empty");
    let mut map: Vec<char> = Vec::new();

    let mut parse_map = false;
    for line in contents.lines() {
        if parse_map {
            map.extend(line.chars());
            continue;
        }
        if line.trim() == "map" {
            parse_map = true;
        } else {
            let mut parts = line.split_whitespace();
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                if parts.next().is_none() {
                    match key {
                        "type" => map_type = value.to_string(),
                        "height" => {
                            height = value.parse::<usize>().map_err(|_| {
                                io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Error parsing map height.",
                                )
                            })?
                        }
                        "width" => {
                            width = value.parse::<usize>().map_err(|_| {
                                io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Error parsing map width.",
                                )
                            })?
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if height == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Map height is missing or zero.",
        ));
    }
    if width == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Map width is missing or zero.",
        ));
    }

    MovingAiMap::new(map_type, height, width, map)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
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
///
/// # Errors
///  Return errors if it is not possible to open the specified file.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use movingai::parser::parse_scen_file;
///
/// let scen = parse_scen_file(Path::new("./tests/arena2.map.scen")).unwrap();
/// ```
pub fn parse_scen_file(path: &path::Path) -> io::Result<Vec<SceneRecord>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    parse_scen(&contents)
}

/// Parse a string representing a MovingAI `.scen`.
///
/// # Arguments
///  * `contents` the string representing the `.scen` file.
///
/// # Returns
///  It returns the parsed map as a `Vec<SceneRecord>` or an `Err`.
///
/// # Panics
///  For the time, it panics if the map format it is not correct.
///
/// # Errors
///  Return errors if it is not possible to open the specified file.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use movingai::parser::parse_scen;
///
/// let scen = parse_scen("version 1\n0	maps/dao/arena.map	49	49	1	11	1	12	1").unwrap();
/// ```
pub fn parse_scen(contents: &str) -> io::Result<Vec<SceneRecord>> {
    let mut table: Vec<SceneRecord> = Vec::new();

    for line in contents.lines() {
        if line.starts_with("version") {
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let record: Vec<&str> = line.split('\t').collect();
        if record.len() < 9 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Expected 9 fields in scene record, found {}", record.len()),
            ));
        }
        table.push(SceneRecord {
            bucket: record[0].parse::<u32>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Error parsing bucket size.")
            })?,
            map_file: String::from(record[1]),
            map_width: record[2].parse::<usize>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Error parsing map width.")
            })?,
            map_height: record[3].parse::<usize>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Error parsing map height.")
            })?,
            start_pos: (
                record[4].parse::<usize>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Error parsing start x.")
                })?,
                record[5].parse::<usize>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Error parsing start y.")
                })?,
            ),
            goal_pos: (
                record[6].parse::<usize>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Error parsing goal x")
                })?,
                record[7].parse::<usize>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Error parsing goal y")
                })?,
            ),
            optimal_length: record[8].parse::<f64>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Error parsing optimal length.")
            })?,
        })
    }

    Ok(table)
}

/// Parse a MovingAI `.3dscen` file.
///
/// # Arguments
///  * `path` represents the path to the file location.
///
/// # Returns
///  It returns the parsed scenarios as a `Vec<SceneRecord3D>` or an `Err`.
///
/// # Errors
///  Return errors if it is not possible to open the specified file.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use movingai::parser::parse_3dscen_file;
///
/// let scen = parse_3dscen_file(Path::new("./tests/A1.3dmap.3dscen")).unwrap();
/// ```
pub fn parse_3dscen_file(path: &path::Path) -> io::Result<Vec<SceneRecord3D>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    parse_3dscen(&contents)
}

/// Parse a string representing a MovingAI `.3dscen`.
///
/// # Arguments
///  * `contents` the string representing the `.3dscen` file.
///
/// # Returns
///  It returns the parsed scenarios as a `Vec<SceneRecord3D>` or an `Err`.
///
/// # Errors
///  Return errors if it is not possible to parse the contents.
///
/// # Examples
///
/// ```
/// use movingai::parser::parse_3dscen;
///
/// let scen = parse_3dscen("version 1\nA1.3dmap\n101 109 191 577 273 142 562.04094761 1.005").unwrap();
/// assert_eq!(scen.len(), 1);
/// ```
pub fn parse_3dscen(contents: &str) -> io::Result<Vec<SceneRecord3D>> {
    let mut lines = contents.lines();

    // First line must be the version header.
    let first = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "3dscen file is empty."))?;
    if !first.starts_with("version") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "3dscen file missing version header.",
        ));
    }

    // Second line is the map filename.
    let map_file = lines
        .next()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "3dscen file missing map filename.",
            )
        })?
        .to_string();

    let mut table: Vec<SceneRecord3D> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 8 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Expected 8 fields in 3D scene record, found {}",
                    parts.len()
                ),
            ));
        }
        let parse_i32 = |s: &str, field: &str| {
            s.parse::<i32>().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Error parsing {}.", field),
                )
            })
        };
        let parse_f64 = |s: &str, field: &str| {
            s.parse::<f64>().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Error parsing {}.", field),
                )
            })
        };
        table.push(SceneRecord3D {
            map_file: map_file.clone(),
            start_pos: (
                parse_i32(parts[0], "start x")?,
                parse_i32(parts[1], "start y")?,
                parse_i32(parts[2], "start z")?,
            ),
            goal_pos: (
                parse_i32(parts[3], "goal x")?,
                parse_i32(parts[4], "goal y")?,
                parse_i32(parts[5], "goal z")?,
            ),
            optimal_length: parse_f64(parts[6], "optimal length")?,
            heuristic_ratio: parse_f64(parts[7], "heuristic ratio")?,
        });
    }

    Ok(table)
}

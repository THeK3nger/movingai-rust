#![allow(clippy::tabs_in_doc_comments)]
use crate::map2d::{MapType, MovingAiMap, SceneRecord};
use crate::map3d::{SceneRecord3D, VoxelMap, VoxelState};
use crate::octree::Octree3D;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path;

/// Errors returned by parser functions.
#[derive(Debug)]
pub enum ParseError {
    /// I/O failure while reading from disk in `*_file` functions.
    Io(io::Error),
    /// Required field or section is missing.
    MissingField(&'static str),
    /// A field has an invalid value.
    InvalidField {
        /// Field name.
        field: &'static str,
        /// Original value.
        value: String,
    },
    /// The record has too few fields.
    InvalidFieldCount {
        /// Record kind (e.g. "scene record").
        kind: &'static str,
        /// Expected minimum number of fields.
        expected: usize,
        /// Actual number of fields found.
        found: usize,
    },
    /// Header line does not match the expected format.
    InvalidHeader(&'static str),
    /// Construction of the parsed map failed.
    InvalidMap(crate::ParseError),
    /// A voxel line points outside declared 3D map dimensions.
    OutOfBoundsVoxel {
        /// X coordinate.
        x: i32,
        /// Y coordinate.
        y: i32,
        /// Z coordinate.
        z: i32,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io(e) => write!(f, "I/O error while reading file: {}", e),
            ParseError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ParseError::InvalidField { field, value } => {
                write!(f, "Invalid value for {}: {}", field, value)
            }
            ParseError::InvalidFieldCount {
                kind,
                expected,
                found,
            } => write!(
                f,
                "Expected at least {} fields in {}, found {}",
                expected, kind, found
            ),
            ParseError::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
            ParseError::InvalidMap(e) => write!(f, "Invalid map: {}", e),
            ParseError::OutOfBoundsVoxel { x, y, z } => {
                write!(
                    f,
                    "Voxel coordinates out of declared 3D map bounds: ({}, {}, {})",
                    x, y, z
                )
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::Io(e) => Some(e),
            ParseError::InvalidMap(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        ParseError::Io(value)
    }
}

impl From<crate::ParseError> for ParseError {
    fn from(value: crate::ParseError) -> Self {
        ParseError::InvalidMap(value)
    }
}

/// Parse a MovingAI `.map` file.
pub fn parse_map_file(path: &path::Path) -> Result<MovingAiMap, ParseError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse_map(&contents)
}

/// Parse a string representing a MovingAI `.map`.
pub fn parse_map(contents: &str) -> Result<MovingAiMap, ParseError> {
    let mut height: Option<usize> = None;
    let mut width: Option<usize> = None;
    let mut map_type: Option<MapType> = None;
    let mut map: Vec<char> = Vec::new();

    let mut parse_map_body = false;
    for line in contents.lines() {
        if parse_map_body {
            map.extend(line.chars());
            continue;
        }
        if line.trim() == "map" {
            parse_map_body = true;
            continue;
        }

        let mut parts = line.split_whitespace();
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            if parts.next().is_none() {
                match key {
                    "type" => {
                        map_type = Some(value.parse::<MapType>().map_err(|_| {
                            ParseError::InvalidField {
                                field: "map type",
                                value: value.to_string(),
                            }
                        })?)
                    }
                    "height" => {
                        let parsed =
                            value
                                .parse::<usize>()
                                .map_err(|_| ParseError::InvalidField {
                                    field: "map height",
                                    value: value.to_string(),
                                })?;
                        height = Some(parsed);
                    }
                    "width" => {
                        let parsed =
                            value
                                .parse::<usize>()
                                .map_err(|_| ParseError::InvalidField {
                                    field: "map width",
                                    value: value.to_string(),
                                })?;
                        width = Some(parsed);
                    }
                    _ => {}
                }
            }
        }
    }

    let height = height.ok_or(ParseError::MissingField("map height"))?;
    if height == 0 {
        return Err(ParseError::InvalidField {
            field: "map height",
            value: "0".to_string(),
        });
    }
    let width = width.ok_or(ParseError::MissingField("map width"))?;
    if width == 0 {
        return Err(ParseError::InvalidField {
            field: "map width",
            value: "0".to_string(),
        });
    }
    let map_type = map_type.ok_or(ParseError::MissingField("map type"))?;

    Ok(MovingAiMap::new(map_type, height, width, map)?)
}

/// Parse a MovingAI `.scen` file.
pub fn parse_scen_file(path: &path::Path) -> Result<Vec<SceneRecord>, ParseError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse_scen(&contents)
}

/// Parse a string representing a MovingAI `.scen`.
pub fn parse_scen(contents: &str) -> Result<Vec<SceneRecord>, ParseError> {
    let mut table: Vec<SceneRecord> = Vec::new();

    for line in contents.lines() {
        if line.starts_with("version") || line.is_empty() {
            continue;
        }

        let record: Vec<&str> = line.split('\t').collect();
        if record.len() < 9 {
            return Err(ParseError::InvalidFieldCount {
                kind: "scene record",
                expected: 9,
                found: record.len(),
            });
        }

        table.push(SceneRecord {
            bucket: record[0]
                .parse::<u32>()
                .map_err(|_| ParseError::InvalidField {
                    field: "bucket size",
                    value: record[0].to_string(),
                })?,
            map_file: String::from(record[1]),
            map_width: record[2]
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidField {
                    field: "map width",
                    value: record[2].to_string(),
                })?,
            map_height: record[3]
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidField {
                    field: "map height",
                    value: record[3].to_string(),
                })?,
            start_pos: (
                record[4]
                    .parse::<usize>()
                    .map_err(|_| ParseError::InvalidField {
                        field: "start x",
                        value: record[4].to_string(),
                    })?,
                record[5]
                    .parse::<usize>()
                    .map_err(|_| ParseError::InvalidField {
                        field: "start y",
                        value: record[5].to_string(),
                    })?,
            ),
            goal_pos: (
                record[6]
                    .parse::<usize>()
                    .map_err(|_| ParseError::InvalidField {
                        field: "goal x",
                        value: record[6].to_string(),
                    })?,
                record[7]
                    .parse::<usize>()
                    .map_err(|_| ParseError::InvalidField {
                        field: "goal y",
                        value: record[7].to_string(),
                    })?,
            ),
            optimal_length: record[8]
                .parse::<f64>()
                .map_err(|_| ParseError::InvalidField {
                    field: "optimal length",
                    value: record[8].to_string(),
                })?,
        });
    }

    Ok(table)
}

/// Parse a MovingAI `.3dmap` file into a [`VoxelMap`].
pub fn parse_3dmap_file(path: &path::Path) -> Result<VoxelMap, ParseError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse_3dmap(&contents)
}

/// Parse a string representing a MovingAI `.3dmap` into a [`VoxelMap`].
pub fn parse_3dmap(contents: &str) -> Result<VoxelMap, ParseError> {
    let mut lines = contents.lines();

    let header = lines
        .next()
        .ok_or(ParseError::InvalidHeader("3dmap file is empty"))?;

    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() != 4 || parts[0] != "voxel" {
        return Err(ParseError::InvalidHeader(
            "3dmap file must start with: voxel <width> <height> <depth>",
        ));
    }

    let width = parts[1]
        .parse::<i32>()
        .map_err(|_| ParseError::InvalidField {
            field: "width",
            value: parts[1].to_string(),
        })?;
    let height = parts[2]
        .parse::<i32>()
        .map_err(|_| ParseError::InvalidField {
            field: "height",
            value: parts[2].to_string(),
        })?;
    let depth = parts[3]
        .parse::<i32>()
        .map_err(|_| ParseError::InvalidField {
            field: "depth",
            value: parts[3].to_string(),
        })?;
    if width <= 0 {
        return Err(ParseError::InvalidField {
            field: "width",
            value: width.to_string(),
        });
    }
    if height <= 0 {
        return Err(ParseError::InvalidField {
            field: "height",
            value: height.to_string(),
        });
    }
    if depth <= 0 {
        return Err(ParseError::InvalidField {
            field: "depth",
            value: depth.to_string(),
        });
    }

    let max_dim = width.max(height).max(depth);
    let mut size = 1;
    while size < max_dim {
        size <<= 1;
    }

    let mut octree = Octree3D::new(size, (0, 0, 0), VoxelState::Free);

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let coords: Vec<&str> = line.split_whitespace().collect();
        if coords.len() < 3 {
            return Err(ParseError::InvalidFieldCount {
                kind: "3dmap voxel record",
                expected: 3,
                found: coords.len(),
            });
        }

        let x = coords[0]
            .parse::<i32>()
            .map_err(|_| ParseError::InvalidField {
                field: "x",
                value: coords[0].to_string(),
            })?;
        let y = coords[1]
            .parse::<i32>()
            .map_err(|_| ParseError::InvalidField {
                field: "y",
                value: coords[1].to_string(),
            })?;
        let z = coords[2]
            .parse::<i32>()
            .map_err(|_| ParseError::InvalidField {
                field: "z",
                value: coords[2].to_string(),
            })?;

        if x < 0 || y < 0 || z < 0 || x >= width || y >= height || z >= depth {
            return Err(ParseError::OutOfBoundsVoxel { x, y, z });
        }

        octree.set_voxel((x, y, z), VoxelState::Occupied);
    }

    let s = size - 1;
    if width < size {
        octree.create_box_obstacle((width, 0, 0), (s, s, s));
    }
    if height < size {
        octree.create_box_obstacle((0, height, 0), (width - 1, s, s));
    }
    if depth < size {
        octree.create_box_obstacle((0, 0, depth), (width - 1, height - 1, s));
    }

    Ok(VoxelMap(octree))
}

/// Parse a MovingAI `.3dscen` file.
pub fn parse_3dscen_file(path: &path::Path) -> Result<Vec<SceneRecord3D>, ParseError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse_3dscen(&contents)
}

/// Parse a string representing a MovingAI `.3dscen`.
pub fn parse_3dscen(contents: &str) -> Result<Vec<SceneRecord3D>, ParseError> {
    let mut lines = contents.lines();

    let first = lines
        .next()
        .ok_or(ParseError::InvalidHeader("3dscen file is empty"))?;
    if !first.starts_with("version") {
        return Err(ParseError::InvalidHeader(
            "3dscen file missing version header",
        ));
    }

    let map_file = lines
        .next()
        .ok_or(ParseError::MissingField("3dscen map filename"))?
        .to_string();

    let mut table: Vec<SceneRecord3D> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 8 {
            return Err(ParseError::InvalidFieldCount {
                kind: "3D scene record",
                expected: 8,
                found: parts.len(),
            });
        }

        table.push(SceneRecord3D {
            map_file: map_file.clone(),
            start_pos: (
                parts[0]
                    .parse::<i32>()
                    .map_err(|_| ParseError::InvalidField {
                        field: "start x",
                        value: parts[0].to_string(),
                    })?,
                parts[1]
                    .parse::<i32>()
                    .map_err(|_| ParseError::InvalidField {
                        field: "start y",
                        value: parts[1].to_string(),
                    })?,
                parts[2]
                    .parse::<i32>()
                    .map_err(|_| ParseError::InvalidField {
                        field: "start z",
                        value: parts[2].to_string(),
                    })?,
            ),
            goal_pos: (
                parts[3]
                    .parse::<i32>()
                    .map_err(|_| ParseError::InvalidField {
                        field: "goal x",
                        value: parts[3].to_string(),
                    })?,
                parts[4]
                    .parse::<i32>()
                    .map_err(|_| ParseError::InvalidField {
                        field: "goal y",
                        value: parts[4].to_string(),
                    })?,
                parts[5]
                    .parse::<i32>()
                    .map_err(|_| ParseError::InvalidField {
                        field: "goal z",
                        value: parts[5].to_string(),
                    })?,
            ),
            optimal_length: parts[6]
                .parse::<f64>()
                .map_err(|_| ParseError::InvalidField {
                    field: "optimal length",
                    value: parts[6].to_string(),
                })?,
            heuristic_ratio: parts[7]
                .parse::<f64>()
                .map_err(|_| ParseError::InvalidField {
                    field: "heuristic ratio",
                    value: parts[7].to_string(),
                })?,
        });
    }

    Ok(table)
}

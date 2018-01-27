#![doc(html_logo_url = "https://www.movingai.com/images/mai3.png", html_favicon_url = "https://www.movingai.com/images/mai3.png")]
#![deny(missing_docs)]

//!
//! The MovingAI Benchmark Parser
//!
//! # Overview
//!
//! Things.

use std::ops::Index;

/// Store coorinates in the (x,y) format.
pub type Coords2D = (usize, usize);

/// A trait representing common operations that can be performed on 2D Maps 
/// representations.
pub trait Map2D<T> {
    
    /// Every Map2D must have an height.
    fn get_height(&self) -> usize;

    /// Every Map2D must have a width.
    fn get_width(&self) -> usize;

    /// In every Map2D must be possible to get an item.
    ///
    /// ## Arguments:
    ///  * `coords` (Coords2D) : A tuple representing the desired coordinates.
    ///
    /// ## Examples:
    ///
    /// ```rust
    /// use movingai::Map2D;
    /// use movingai::MovingAiMap;
    ///
    /// let mm = MovingAiMap::new(
    ///        String::from("test"),
    ///        54,
    ///        56,
    ///        vec!['.'; 54*56]
    ///    );
    /// let result = mm.get_cell((23,4));
    /// assert_eq!(*result, '.')
    /// ```
    fn get_cell(&self, coords: Coords2D) -> &T;

    /// Check if the given coordinates are out of bound.
    ///
    /// # Examples
    ///
    /// ```
    /// # use movingai::Map2D;
    /// # use movingai::MovingAiMap;
    /// #
    /// # let mm = MovingAiMap::new(
    /// #       String::from("test"),
    /// #       54,
    /// #       56,
    /// #       vec!['.'; 54*56]
    /// #   );
    /// assert!(mm.is_out_of_bound((76,3)));
    /// assert!(!mm.is_out_of_bound((23,23)));
    /// ```
    ///  
    fn is_out_of_bound(&self, coords: Coords2D) -> bool;

    /// Check if a tile in the map can be traversed.
    ///
    /// This check if a tile can be traversed by an agent **in some situation**.
    /// For instance, a water tile `W` is traversable if coming from another
    /// water tile, so this function will return `true`.
    ///
    /// The only things that can not be traversed are trees (`T`), out of bounds,
    /// and other unpassable obstacles (`@` and `O``).
    ///
    fn is_traversable(&self, tile: Coords2D) -> bool;

    /// Check if a tile in the map can be traversed coming from the `from` tile.
    ///
    /// # Arguments
    ///  - `from` The tile from which the agent starts moving.
    ///  - `to` The destination tile.
    ///
    /// # Details
    /// For instance, in `MovingAIMap` the implementation encodes all the MovingAI
    /// rules about traversability.
    ///
    /// In particular:
    ///  - A water tile (`W`) can be traversed but only if the agent does not
    ///    comes from regular terrain (`.` and `G`).
    ///  - A swamp tile (`S`) can be traversed only if the agent comes from
    ///    regular terrain.
    ///
    /// For example, I can move from `W` to `W` or form `W` to `.`, 
    /// but not from `.` to `W`. Or I can move from `.` to `S` or 
    /// from `S` to `.`, or from `S` to `S` but not from `S` to `W` 
    /// (and vice versa).
    fn is_traversable_from(&self, from: Coords2D, to: Coords2D) -> bool;

}

/// An immutable representation of a MovingAI map.
pub struct MovingAiMap {
    map_type: String,
    height: usize,
    width: usize,
    map: Vec<char>
}

impl MovingAiMap {

    /// Create a new `MovingAIMap` object from basic components.
    ///
    /// # Arguments
    ///  * `map_type`: The type of map you are registering. Usually `octile`.
    ///  * `height`: the height of the map.
    ///  * `width`: the width of the map.
    ///  * `map`: A vector representing the map in row-major order.
    ///
    /// # Panics
    /// 
    /// The `new` call will panic id the size of the map vector is different
    /// from `heigth*width`.
    pub fn new(map_type: String, height: usize, width: usize, map: Vec<char>) -> MovingAiMap {
        if map.len() != height*width {
            panic!("Given vector is not compatible with passed `width` and `height`.");
        }
        MovingAiMap {
            map_type, height, width, map
        }
    }

    fn coordinates_connect(&self, coordsA: Coords2D, coordsB: Coords2D) -> bool {
        let x1 = coordsA.0 as i32;
        let x2 = coordsB.0 as i32;
        let y1 = coordsA.1 as i32;
        let y2 = coordsB.1 as i32;
        if self.map_type == "octile" {
            (x1-x2).abs() <= 1 && (y1-y2).abs() <= 1
        } else {
            (y2 == y1 && (x2 == x1 + 1 || x2 == x1 - 1)) || (x2 == x1 && (y2 == y1 + 1 || y2 == y1 - 1))
        }
    }

}

impl Map2D<char> for MovingAiMap {

    fn get_width(&self) -> usize { self.width }

    fn get_height(&self) -> usize { self.height }
    
    fn get_cell(&self, coords: Coords2D) -> &char {
        &self.map[coords.1*self.get_width() + coords.0]
    }

    fn is_out_of_bound(&self, coords: Coords2D) -> bool {
        coords.0 >= self.width || coords.1 >= self.height
    }

    fn is_traversable(&self, tile: Coords2D) -> bool {
        if self.is_out_of_bound(tile) { return false; }
        let tile_char = self.get_cell(tile);
        match *tile_char {
            '.' => true,
            'G' => true,
            '@' => false,
            'O' => false,
            'T' => false,
            'S' => true,
            'W' => true,
            _   => false, // Not recognized char.
        }
    }

    fn is_traversable_from(&self, from: Coords2D, to: Coords2D) -> bool {
        if self.is_out_of_bound(to) { return false; }
        if self.is_out_of_bound(from) { return false; }
        if !self.coordinates_connect(to, from) { return false; }
        let tile_char = *(self.get_cell(to));
        let from_char = *(self.get_cell(from));
        match (tile_char, from_char) {
            ('.', _) => true,
            ('G', _) => true,
            ('@', _) => false,
            ('O', _) => false,
            ('T', _) => false,
            ('S', '.') => true,
            ('S', 'S') => true,
            ('W', 'W') => true,
            _ => false,
        }
    }

}

impl Index<Coords2D> for MovingAiMap  {

    type Output = char;

    fn index(&self, coords: Coords2D) -> &char {
        self.get_cell(coords)
    }
    
}

/// Represent a row (scene) in a scene file.
pub struct SceneRecord {
    bucket: u32,
    map_file: String,
    map_width: usize,
    map_height: usize,
    start_pos: Coords2D,
    goal_pos: Coords2D,
    optimal_length: f64
}

/// Contains all the parser functions.
pub mod parser {

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
    /// let map = parse_map_file("./test/arena.map").unwrap();
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
    /// let scen = parse_scen_file("./test/arena2.map.scen").unwrap();
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

}

#[cfg(test)]
mod tests {

    use Map2D;
    use MovingAiMap;
    use parser::parse_map_file;
    use parser::parse_scen_file;

    #[test]
    fn indexing() {
        let test = MovingAiMap {
            map_type: String::from("test"),
            height: 4,
            width: 6,
            map: vec!['.'; 4*6]
        };
        assert_eq!(test[(0,3)], '.');
        assert_eq!(test[(3,0)], '.');
    }

    #[test]
    fn parsing_map() {
        let map = parse_map_file("./test/arena.map").unwrap();
        assert_eq!(map.get_width(), 49 );
        assert_eq!(*map.get_cell((3,0)), 'T');
    }

    #[test]
    fn parsing_scene() {
        let scen = parse_scen_file("./test/arena2.map.scen").unwrap();
        assert_eq!(scen[3].start_pos,(102, 165));
    }

    #[test]
    fn traversability() {
        let map = parse_map_file("./test/arena.map").unwrap();
        assert!(!map.is_traversable((0,0)));
        assert!(map.is_traversable((5,2)));
        assert!(!map.is_traversable_from((3,1),(3,0)));
        assert!(!map.is_traversable_from((3,1),(3,7)));      
    }
}

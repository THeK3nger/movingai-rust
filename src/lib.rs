#![doc(html_logo_url = "https://www.movingai.com/images/mai3.png", html_favicon_url = "https://www.movingai.com/images/mai3.png")]
#![deny(missing_docs)]

//!
//! The MovingAI Benchmark Parser
//!
//! # Overview
//!
//! Things.

/// Contains all the parser functions.
pub mod parser;

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

    /// Return an iterator returning all the coordinates in the map
    /// in row-major order.
    fn coords_iter(&self) -> Map2DCoordsIter;

    /// Return the number of free states of a map.
    ///
    /// For "free state" we means _any_ tile that can _potentially_
    /// be traversed.
    fn free_states(&self) -> u32;

    /// Return the list of accessible neighbors of a tile.
    fn neighbors(&self, tile: Coords2D) -> Vec<Coords2D>;

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

/// This represents a coordinate iterator for a Map2D.
pub struct Map2DCoordsIter {
    width: usize,
    height: usize,
    curr_x: usize,
    curr_y: usize,
}

impl Iterator for Map2DCoordsIter {

    type Item = Coords2D;

    fn next(&mut self) -> Option<Self::Item> {
        // We save the current value.
        let x = self.curr_x;
        let y = self.curr_y;
        // If y is out of bound, we stop.
        if self.curr_y >= self.height {
            return None;
        }
        // We compute the next pair of values.
        self.curr_x += 1;
        if self.curr_x >= self.width {
            self.curr_x = 0;
            self.curr_y += 1;
        }
        // But we return the current one!
        Some((x, y))
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

    fn coords_iter(&self) -> Map2DCoordsIter {
        Map2DCoordsIter { width: self.width, height: self.height, curr_x: 0, curr_y: 0 }
    }

    fn free_states(&self) -> u32 {
        let mut counter = 0;
        for c in self.coords_iter() {
            if self.is_traversable(c) {
                counter+=1;
            }
        }
        return counter;
    }

    fn neighbors(&self, tile: Coords2D) -> Vec<Coords2D> {
        let x = tile.0;
        let y = tile.1;
        let all = vec![(x+1,y), (x+1, y+1), (x+1, y-1), 
            (x,y+1), (x, y-1), (x-1, y), (x-1, y-1), (x-1, y+1)];
        return all.into_iter().filter(|x| self.is_traversable_from(tile, *x)).collect();
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
    /// Used to cluster pqth queries in the benchmark.
    pub bucket: u32,

    /// Neme of the map file associated to the scene.
    pub map_file: String,

    /// Width of the map.
    pub map_width: usize,

    /// Height of the map.
    pub map_height: usize,

    /// Starting position.
    pub start_pos: Coords2D,

    /// Goal position.
    pub goal_pos: Coords2D,

    /// Optimal lenght of the path.
    pub optimal_length: f64
}

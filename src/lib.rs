#![doc(html_logo_url = "https://www.movingai.com/images/mai3.png",
       html_favicon_url = "https://www.movingai.com/images/mai3.png")]
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
    fn height(&self) -> usize;

    /// Every Map2D must have a width.
    fn width(&self) -> usize;

    /// In every Map2D must be possible to get a tile.
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
    /// let result = mm.get((23,4));
    /// assert_eq!(*result, '.')
    /// ```
    fn get(&self, coords: Coords2D) -> &T;

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
    fn coords(&self) -> CoordsIter;

    /// Return the number of free states of a map.
    ///
    /// For "free state" we means _any_ tile that can _potentially_
    /// be traversed.
    fn free_states(&self) -> usize;

    /// Return the list of accessible neighbors of a tile.
    fn neighbors(&self, tile: Coords2D) -> Vec<Coords2D>;
}

/// An immutable representation of a MovingAI map.
pub struct MovingAiMap {
    map_type: String,
    height: usize,
    width: usize,
    map: Vec<char>,
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
        assert_eq!(map.len(), height * width);
        MovingAiMap {
            map_type,
            height,
            width,
            map,
        }
    }

    fn coordinates_connect(&self, coords_a: Coords2D, coords_b: Coords2D) -> bool {
        let (x1, y1) = (coords_a.0 as isize, coords_a.1 as isize);
        let (x2, y2) = (coords_b.0 as isize, coords_b.1 as isize);
        if self.map_type == "octile" {
            (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1
        } else {
            (y2 == y1 && (x2 == x1 + 1 || x2 == x1 - 1))
                || (x2 == x1 && (y2 == y1 + 1 || y2 == y1 - 1))
        }
    }
}

/// This represents a coordinate iterator for a `Map2D`.
pub struct CoordsIter {
    width: usize,
    height: usize,
    curr_x: usize,
    curr_y: usize,
}

impl Iterator for CoordsIter {
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
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, coords: Coords2D) -> &char {
        &self.map[coords.1 * self.width() + coords.0]
    }

    fn is_out_of_bound(&self, coords: Coords2D) -> bool {
        coords.0 >= self.width || coords.1 >= self.height
    }

    fn is_traversable(&self, tile: Coords2D) -> bool {
        if self.is_out_of_bound(tile) {
            return false;
        }
        let tile_char = self.get(tile);
        match *tile_char {
            '.' | 'G' | 'S' | 'W' => true,
            '@' | 'O' | 'T' => false,
            _ => false, // Not recognized char.
        }
    }

    fn is_traversable_from(&self, from: Coords2D, to: Coords2D) -> bool {
        if self.is_out_of_bound(to) {
            return false;
        }
        if self.is_out_of_bound(from) {
            return false;
        }
        if !self.coordinates_connect(to, from) {
            return false;
        }
        let diagonal = from.0 != to.0 && from.1 != to.1;
        let octile = self.map_type == "octile";
        let tile_char = *(self.get(to));
        let from_char = *(self.get(from));
        if !octile || !diagonal {
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
        } else {
            // When connecting diagonals we need to check that the step is
            // not cutting corner.
            //
            // xb.
            // a..
            // ...
            //
            // In the above example a cannot traverse from a to b because it
            // would cut the corner `x`.
            let (x, y) = from;
            let (p, q) = to;
            let intermediate_a = (x, q);
            let intermediate_b = (p, y);
            // A corner is not cut only if it is possible to reach the diagonal
            // With a ANY double-step in a non-diagonal path.
            self.is_traversable_from(from, intermediate_a)
                && self.is_traversable_from(intermediate_a, to)
                && self.is_traversable_from(from, intermediate_b)
                && self.is_traversable_from(intermediate_b, to)
        }
    }

    fn coords(&self) -> CoordsIter {
        CoordsIter {
            width: self.width,
            height: self.height,
            curr_x: 0,
            curr_y: 0,
        }
    }

    fn free_states(&self) -> usize {
        self.coords()
            .filter(|c| self.is_traversable(*c))
            .count()
    }

    fn neighbors(&self, tile: Coords2D) -> Vec<Coords2D> {
        let (x, y) = tile;
        let all = vec![
            (x + 1, y),
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x, y + 1),
            (x, y - 1),
            (x - 1, y),
            (x - 1, y - 1),
            (x - 1, y + 1),
        ];
        all.into_iter()
            .filter(|x| self.is_traversable_from(tile, *x))
            .collect()
    }
}

impl Index<Coords2D> for MovingAiMap {
    type Output = char;

    fn index(&self, coords: Coords2D) -> &char {
        self.get(coords)
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
    pub optimal_length: f64,
}

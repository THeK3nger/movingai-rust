use std::ops::Index;

type Coords2D = (usize, usize);

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
    /// let mm = MovingAiMap::create(
    ///        String::from("test"),
    ///        54,
    ///        56,
    ///        vec!['.'; 54*56]
    ///    );
    /// let result = mm.get_cell((23,4));
    /// assert_eq!(*result, '.')
    /// ```
    fn get_cell(&self, coords: Coords2D) -> &T;

}


/// An immutable representation of a MovingAI map.
pub struct MovingAiMap {
    map_type: String,
    height: usize,
    width: usize,
    map: Vec<char>
}

impl MovingAiMap {

    pub fn create(map_type: String, height: usize, width: usize, map: Vec<char>) -> MovingAiMap {
        if (map.len() != height*width) {
            panic!("Given vector is not compatible with passed `width` and `height`.");
        }
        MovingAiMap {
            map_type, height, width, map
        }
    }

}

impl Map2D<char> for MovingAiMap {

    fn get_width(&self) -> usize { self.width }

    fn get_height(&self) -> usize { self.height }
    
    fn get_cell(&self, coords: Coords2D) -> &char {
        &self.map[coords.1*self.get_width() + coords.0]
    }

}

impl Index<Coords2D> for MovingAiMap  {

    type Output = char;

    fn index(&self, coords: Coords2D) -> &char {
        self.get_cell(coords)
    }
    
}


mod parser {

    use MovingAiMap;

    // pub fn parse_file(path: String) -> MovingAiMap {

    // }

}

#[cfg(test)]
mod tests {

    use MovingAiMap;

    #[test]
    fn indexing() {
        let test = MovingAiMap {
            map_type: String::from("test"),
            height: 4,
            width: 6,
            map: vec!['.'; 4*6]
        };
        assert_eq!(test[(0,3)], '.');
    }
}

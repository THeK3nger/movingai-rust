extern crate movingai;

use std::cmp::Ordering;

use movingai::Map2D;
use movingai::MovingAiMap;
use movingai::Coords2D;
use movingai::parser::parse_map_file;
use movingai::parser::parse_scen_file;

#[derive(Debug)]
struct SearchNode {
    pub f: f64,
    pub h: f64,
    pub g: f64,
    pub current: Coords2D,
    pub parent: Coords2D
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &SearchNode) -> bool{
        self.current == other.current
    }
}

impl Eq for SearchNode {
    // add code here
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &SearchNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &SearchNode) -> Ordering {
        if self.f > other.f { Ordering::Greater }
        else if self.f < other.f { Ordering::Less }
        else { Ordering::Equal }
    }
}

fn astar(start: Coords2D, goal: Coords2D) {

}
 
fn main() {
    let map = parse_map_file("./tests/arena.map").unwrap();
    let scenes = parse_scen_file("./tests/arena.map.scen").unwrap();


}
extern crate movingai;

use std::time::{Duration, Instant};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

use movingai::Map2D;
use movingai::MovingAiMap;
use movingai::Coords2D;
use movingai::parser::parse_map_file;
use movingai::parser::parse_scen_file;

// Let's define the search nodes.

#[derive(Debug)]
struct SearchNode {
    pub f: f64,
    pub h: f64,
    pub g: f64,
    pub current: Coords2D,
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
        // This is reversed on purpose to make the max-heap into min-heap.
        if self.f < other.f { Ordering::Greater }
        else if self.f > other.f { Ordering::Less }
        else { Ordering::Equal }
    }
}

fn distance(a: Coords2D, b: Coords2D) -> f64 {
    let (x,y) = (a.0 as f64, a.1 as f64);
    let (p,q) = (b.0 as f64, b.1 as f64);
    ((x-p)*(x-p) + (y-q)*(y-q)).sqrt()
}

// A* shortest path algorithm.

// This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue.
fn shortest_path(map: &MovingAiMap, start: Coords2D, goal: Coords2D) -> Option<f64> {

    let mut heap = BinaryHeap::new();
    let mut visited = Vec::<Coords2D>::new();

    // We're at `start`, with a zero cost
    heap.push(SearchNode { f: 0.0, g:0.0, h: distance(start, goal), current: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(SearchNode { f: _f, g, h: _h, current }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths

        // println!("G: {:?} H.size: {:?}", g, heap.len());

        if current == goal { return Some(g); }

        if visited.contains(&current) {
            continue;
        }

        visited.push(current);

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for neigh in map.neighbors(current) {
            // println!("Current {:?} Neigh {:?}", current,  neigh);
            let new_h = distance(neigh, goal);
            let i = distance(neigh, current);
            let next = SearchNode { f: g+i+new_h, g: g+i, h: new_h, current: neigh };
            heap.push(next);
        }
    }

    // Goal not reachable
    None
}
 
fn main() {
    let map = parse_map_file("./tests/arena.map").unwrap();
    let scenes = parse_scen_file("./tests/arena.map.scen").unwrap();
    for scene in scenes {
        let start = scene.start_pos;
        let goal = scene.goal_pos;
        let t = Instant::now();
        match shortest_path(&map, (1,3), (4,3)) {
            Some(x) => {
                let duration = t.elapsed();
                let seconds = duration.as_secs();
                let ms = (duration.subsec_nanos() as f64) / 1_000_000.0;
                println!("{:?} -> {:?} \tin {:?} seconds and {:?} ms", start, goal, seconds, ms);
            }
            None => println!("None"),
        }
    }
}
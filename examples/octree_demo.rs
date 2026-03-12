use movingai::parser::{parse_3dmap_file, parse_3dscen_file};
use movingai::{Coords3D, Map3D, VoxelMap, VoxelState};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::path::Path;

// -- A* -----------------------------------------------------------------------

#[derive(Clone, PartialEq)]
struct State {
    f: f64,
    g: f64,
    coords: Coords3D,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// 3D octile distance — admissible heuristic for a 26-connected grid.
fn heuristic(a: Coords3D, b: Coords3D) -> f64 {
    let dx = (a.0 - b.0).abs() as f64;
    let dy = (a.1 - b.1).abs() as f64;
    let dz = (a.2 - b.2).abs() as f64;
    let mut dims = [dx, dy, dz];
    dims.sort_by(|x, y| y.partial_cmp(x).unwrap());
    let (s, m, l) = (dims[2], dims[1], dims[0]);
    (l - m) + (m - s) * std::f64::consts::SQRT_2 + s * 3f64.sqrt()
}

fn move_cost(a: Coords3D, b: Coords3D) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    let dz = (a.2 - b.2) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn astar(octree: &VoxelMap, start: Coords3D, goal: Coords3D) -> Option<f64> {
    if !matches!(octree.get_voxel(start), Some(VoxelState::Free)) {
        return None;
    }
    if !matches!(octree.get_voxel(goal), Some(VoxelState::Free)) {
        return None;
    }

    let mut open = BinaryHeap::new();
    let mut g_score: HashMap<Coords3D, f64> = HashMap::new();

    g_score.insert(start, 0.0);
    open.push(State {
        f: heuristic(start, goal),
        g: 0.0,
        coords: start,
    });

    while let Some(State { g, coords, .. }) = open.pop() {
        if coords == goal {
            return Some(g);
        }
        if g > *g_score.get(&coords).unwrap_or(&f64::INFINITY) {
            continue;
        }
        for neighbor in octree.neighbors(coords) {
            let new_g = g + move_cost(coords, neighbor);
            if new_g < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                g_score.insert(neighbor, new_g);
                open.push(State {
                    f: new_g + heuristic(neighbor, goal),
                    g: new_g,
                    coords: neighbor,
                });
            }
        }
    }

    None
}

// -- main ---------------------------------------------------------------------

fn main() {
    println!("Loading map...");
    let octree = parse_3dmap_file(Path::new("./tests/A1.3dmap")).expect("Failed to load A1.3dmap");
    println!("Octree size: {}^3\n", octree.size());

    let scenarios = parse_3dscen_file(Path::new("./tests/A1.3dmap.3dscen"))
        .expect("Failed to load A1.3dmap.3dscen");
    println!(
        "Loaded {} scenarios. Running A* on the 5 shortest...\n",
        scenarios.len()
    );

    // Pick the 5 scenarios with the shortest optimal path to keep the demo fast.
    let mut indexed: Vec<(usize, _)> = scenarios.iter().enumerate().collect();
    indexed.sort_by(|a, b| a.1.optimal_length.partial_cmp(&b.1.optimal_length).unwrap());

    for (i, scen) in indexed.iter().take(5) {
        let start = scen.start_pos;
        let goal = scen.goal_pos;
        print!(
            "Scenario {:>3}  {:?} -> {:?}  expected {:.5}  ",
            i + 1,
            start,
            goal,
            scen.optimal_length
        );
        match astar(&octree, start, goal) {
            Some(length) => println!(
                "found {:.5}  ratio {:.3}",
                length,
                length / scen.optimal_length
            ),
            None => println!("no path found"),
        }
    }
}

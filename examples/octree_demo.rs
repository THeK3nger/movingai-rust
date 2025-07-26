use movingai::octree::{Octree3D, VoxelState};

fn main() {
    // Create a new octree with size 16x16x16, starting at origin
    let mut octree = Octree3D::new(16, (0, 0, 0), VoxelState::Free);

    // Set some voxels as occupied to create obstacles
    octree.set_voxel((5, 5, 5), VoxelState::Occupied);
    octree.set_voxel((6, 5, 5), VoxelState::Occupied);
    octree.set_voxel((5, 6, 5), VoxelState::Occupied);
    octree.set_voxel((7, 8, 9), VoxelState::Occupied);

    // Query a specific voxel
    let center = (5, 5, 5);
    match octree.get_voxel(center) {
        Some(VoxelState::Occupied) => println!("Voxel at {:?} is occupied", center),
        Some(VoxelState::Free) => println!("Voxel at {:?} is free", center),
        None => println!("Voxel at {:?} is out of bounds", center),
    }

    // Get all neighbors of a voxel
    let neighbors = octree.get_neighbors(center);
    println!("Voxel at {:?} has {} neighbors within bounds", center, neighbors.len());

    // Get only free neighbors (useful for pathfinding)
    let free_neighbors = octree.get_free_neighbors(center);
    println!("Free neighbors: {:?}", free_neighbors);

    // Get only occupied neighbors (useful for collision detection)
    let occupied_neighbors = octree.get_occupied_neighbors(center);
    println!("Occupied neighbors: {:?}", occupied_neighbors);

    // Example of pathfinding-like usage
    let start = (1, 1, 1);
    let goal = (10, 10, 10);
    
    println!("\nPathfinding example:");
    println!("Start: {:?} - {:?}", start, octree.get_voxel(start).unwrap());
    println!("Goal: {:?} - {:?}", goal, octree.get_voxel(goal).unwrap());
    
    // Get valid moves from start position
    let valid_moves = octree.get_free_neighbors(start);
    println!("Valid moves from start: {} options", valid_moves.len());
    
    // Example of checking a path segment
    let path_point = (5, 5, 6); // Right above the occupied voxel
    let neighbors_around_path = octree.get_neighbors(path_point);
    let obstacles_nearby = octree.get_occupied_neighbors(path_point);
    
    println!("\nPath point {:?}:", path_point);
    println!("Total neighbors: {}", neighbors_around_path.len());
    println!("Obstacles nearby: {}", obstacles_nearby.len());
    
    if !obstacles_nearby.is_empty() {
        println!("Warning: Obstacles detected near path point!");
        for obstacle in obstacles_nearby {
            println!("  Obstacle at: {:?}", obstacle);
        }
    }
}

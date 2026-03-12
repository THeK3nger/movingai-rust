use movingai::{VoxelMap, VoxelState};

fn main() {
    // Create a simple 8x8x8 3D map
    let mut map = VoxelMap::new(8, (0, 0, 0), VoxelState::Free);

    // Create a simple obstacle
    map.set_voxel((3, 3, 3), VoxelState::Occupied);
    map.set_voxel((3, 3, 4), VoxelState::Occupied);
    map.set_voxel((3, 4, 3), VoxelState::Occupied);
    map.set_voxel((4, 3, 3), VoxelState::Occupied);

    // Query a position and its neighbors
    let position = (3, 3, 3);
    println!("Checking position {:?}", position);

    match map.get_voxel(position) {
        Some(VoxelState::Free) => println!("Position is free"),
        Some(VoxelState::Occupied) => println!("Position is occupied"),
        None => println!("Position is out of bounds"),
    }

    // Get free neighbors for pathfinding
    let free_neighbors = map.get_free_neighbors(position);
    println!("Free neighbors: {} found", free_neighbors.len());

    // Get occupied neighbors for collision detection
    let occupied_neighbors = map.get_occupied_neighbors(position);
    println!("Occupied neighbors: {} found", occupied_neighbors.len());

    // Example usage for a pathfinding scenario
    let agent_position = (2, 2, 2);
    let possible_moves = map.get_free_neighbors(agent_position);

    println!(
        "\nAgent at {:?} can move to {} positions:",
        agent_position,
        possible_moves.len()
    );
    for (i, pos) in possible_moves.iter().take(5).enumerate() {
        println!("  Move {}: {:?}", i + 1, pos);
    }
    if possible_moves.len() > 5 {
        println!("  ... and {} more", possible_moves.len() - 5);
    }
}

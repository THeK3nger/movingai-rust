use movingai::{VoxelMap, VoxelState};

fn main() {
    println!("=== Octree 3D Map Utility Demo ===\n");

    // Create a larger 3D map for demonstration
    let mut map = VoxelMap::new(16, (0, 0, 0), VoxelState::Free);

    println!(
        "Created {}x{}x{} 3D map",
        map.size(),
        map.size(),
        map.size()
    );
    println!(
        "Bounds: {:?} to ({}, {}, {})\n",
        map.min_coords(),
        map.min_coords().0 + map.size() - 1,
        map.min_coords().1 + map.size() - 1,
        map.min_coords().2 + map.size() - 1
    );

    // Demonstrate box obstacle creation
    println!("Creating a 3x3x3 box obstacle at (5,5,5)...");
    let box_count = map.create_box_obstacle((5, 5, 5), (7, 7, 7));
    println!("Set {} voxels as occupied", box_count);

    // Demonstrate sphere obstacle creation
    println!("\nCreating a sphere obstacle (radius 2.5) at (10,10,10)...");
    let sphere_count = map.create_sphere_obstacle((10, 10, 10), 2.5);
    println!("Set {} voxels as occupied", sphere_count);

    // Set multiple voxels at once
    println!("\nSetting multiple scattered obstacles...");
    let scattered_obstacles = vec![
        ((2, 2, 2), VoxelState::Occupied),
        ((2, 2, 3), VoxelState::Occupied),
        ((13, 13, 13), VoxelState::Occupied),
        ((1, 8, 15), VoxelState::Occupied),
    ];
    let scattered_count = map.set_voxels(scattered_obstacles);
    println!("Set {} scattered obstacles", scattered_count);

    // Analyze a region
    println!("\n=== Region Analysis ===");
    let region_min = (4, 4, 4);
    let region_max = (8, 8, 8);
    let obstacles_in_region = map.count_occupied_in_region(region_min, region_max);
    let total_voxels_in_region = (region_max.0 - region_min.0 + 1)
        * (region_max.1 - region_min.1 + 1)
        * (region_max.2 - region_min.2 + 1);

    println!("Region {:?} to {:?}:", region_min, region_max);
    println!("  Total voxels: {}", total_voxels_in_region);
    println!("  Occupied: {}", obstacles_in_region);
    println!(
        "  Free: {}",
        total_voxels_in_region - obstacles_in_region as i32
    );
    println!(
        "  Occupancy: {:.1}%",
        (obstacles_in_region as f32 / total_voxels_in_region as f32) * 100.0
    );

    // Pathfinding scenario
    println!("\n=== Pathfinding Scenario ===");
    let start_pos = (1, 1, 1);
    let goal_pos = (14, 14, 14);

    println!("Planning path from {:?} to {:?}", start_pos, goal_pos);

    // Check if start and goal are free
    if !map.is_free(start_pos) {
        println!("Warning: Start position is occupied!");
    } else {
        println!("Start position is free ✓");
    }

    if !map.is_free(goal_pos) {
        println!("Warning: Goal position is occupied!");
    } else {
        println!("Goal position is free ✓");
    }

    // Find valid moves from start
    let start_neighbors = map.get_free_neighbors(start_pos);
    println!(
        "From start: {} valid moves available",
        start_neighbors.len()
    );

    // Check a path point near the box obstacle
    let path_point = (6, 6, 8); // Just above the box
    println!("\nAnalyzing path point {:?}:", path_point);

    if map.is_free(path_point) {
        println!("  Path point is free ✓");
        let free_moves = map.get_free_neighbors(path_point);
        let blocked_neighbors = map.get_occupied_neighbors(path_point);

        println!("  Free neighbors: {}", free_moves.len());
        println!("  Blocked neighbors: {}", blocked_neighbors.len());

        if blocked_neighbors.len() > 0 {
            println!("  Nearby obstacles at:");
            for obstacle in blocked_neighbors.iter().take(5) {
                println!("    {:?}", obstacle);
            }
        }
    } else {
        println!("  Path point is blocked ✗");
    }

    // Demonstrate efficient neighbor checking
    println!("\n=== Navigation Example ===");
    let agent_pos = (8, 8, 2); // Below the box obstacle
    println!("Agent at {:?}", agent_pos);

    // Get all possible moves
    let all_neighbors = map.get_neighbors(agent_pos);
    let free_neighbors = map.get_free_neighbors(agent_pos);
    let blocked_neighbors = map.get_occupied_neighbors(agent_pos);

    println!("  Total neighbors checked: {}", all_neighbors.len());
    println!("  Valid moves: {}", free_neighbors.len());
    println!("  Blocked moves: {}", blocked_neighbors.len());

    // Show movement options in different directions
    let directions = [
        ("Up", (agent_pos.0, agent_pos.1, agent_pos.2 + 1)),
        ("Down", (agent_pos.0, agent_pos.1, agent_pos.2 - 1)),
        ("North", (agent_pos.0, agent_pos.1 + 1, agent_pos.2)),
        ("South", (agent_pos.0, agent_pos.1 - 1, agent_pos.2)),
        ("East", (agent_pos.0 + 1, agent_pos.1, agent_pos.2)),
        ("West", (agent_pos.0 - 1, agent_pos.1, agent_pos.2)),
    ];

    println!("\n  Movement options:");
    for (direction, coords) in directions {
        if map.is_free(coords) {
            println!("    {} to {:?}: Free ✓", direction, coords);
        } else {
            println!("    {} to {:?}: Blocked ✗", direction, coords);
        }
    }

    println!("\n=== Demo Complete ===");
    println!("The octree efficiently stores 3D voxel data and provides fast neighbor queries");
    println!("for pathfinding, collision detection, and spatial analysis applications.");
}

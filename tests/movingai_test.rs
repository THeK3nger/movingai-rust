use std::path::Path;

use movingai::Map2D;
use movingai::MapType;
use movingai::MovingAiMap;
use movingai::parser::parse_3dmap;
use movingai::parser::parse_3dmap_file;
use movingai::parser::parse_3dscen;
use movingai::parser::parse_3dscen_file;
use movingai::parser::parse_map_file;
use movingai::parser::parse_scen;
use movingai::parser::parse_scen_file;

#[test]
fn indexing() {
    let test = MovingAiMap::new(MapType::Octile, 4, 6, vec!['.'; 4 * 6]).unwrap();
    assert_eq!(test[(0, 3)], '.');
    assert_eq!(test[(3, 0)], '.');
}

#[test]
fn parsing_map() {
    let map = parse_map_file(Path::new("./tests/arena.map")).unwrap();
    assert_eq!(map.width(), 49);
    assert_eq!(*map.get((3, 0)), 'T');
}

#[test]
fn parsing_scene() {
    let scen = parse_scen_file(Path::new("./tests/arena2.map.scen")).unwrap();
    assert_eq!(scen[3].start_pos, (102, 165));
}

#[test]
fn traversability() {
    let map = parse_map_file(Path::new("./tests/arena.map")).unwrap();
    assert!(!map.is_traversable((0, 0)));
    assert!(map.is_traversable((5, 2)));
    assert!(!map.is_traversable_from((3, 1), (3, 0)));
    assert!(!map.is_traversable_from((3, 1), (3, 7)));
}

#[test]
fn iterator() {
    let map = parse_map_file(Path::new("./tests/arena.map")).unwrap();
    let arena_w = 49;
    let mut x = 0;
    let mut y = 0;
    for c in map.coords() {
        assert_eq!(c, (x, y));
        x += 1;
        if x >= arena_w {
            x = 0;
            y += 1;
        }
    }
}

#[test]
fn states() {
    let map = parse_map_file(Path::new("./tests/arena.map")).unwrap();
    assert_eq!(map.free_states(), 2054);
}

#[test]
fn neighbours() {
    let map = parse_map_file(Path::new("./tests/arena.map")).unwrap();
    let neigh = map.neighbors((19, 1));
    assert_eq!(neigh.len(), 1);
    assert!(neigh.contains(&(19, 2)));
    assert!(!neigh.contains(&(19, 0)));
}

#[test]
fn neighbors_at_origin_does_not_panic() {
    let map = MovingAiMap::new(
        MapType::Octile,
        3,
        3,
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    )
    .unwrap();
    let neigh = map.neighbors((0, 0));
    assert!(!neigh.is_empty());
    assert!(neigh.contains(&(1, 0)));
    assert!(neigh.contains(&(0, 1)));
    assert!(neigh.contains(&(1, 1)));
}

#[test]
fn neighbors_at_bottom_right_does_not_panic() {
    let map = MovingAiMap::new(
        MapType::Octile,
        3,
        3,
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    )
    .unwrap();
    let neigh = map.neighbors((2, 2));
    assert!(!neigh.is_empty());
    assert!(neigh.contains(&(1, 2)));
    assert!(neigh.contains(&(2, 1)));
    assert!(neigh.contains(&(1, 1)));
}

#[test]
fn parse_scen_malformed_line_returns_error() {
    let malformed = "version 1\n0\tmaps/dao/arena.map\t49";
    let result = parse_scen(malformed);
    assert!(result.is_err());
}

#[test]
fn parse_map_unknown_type_returns_error() {
    let malformed = "type not-a-real-type\nheight 1\nwidth 1\nmap\n.";
    let result = movingai::parser::parse_map(malformed);
    assert!(result.is_err());
}

#[test]
fn parse_scen_empty_line_is_skipped() {
    let input = "version 1\n\n0\tmaps/dao/arena.map\t49\t49\t1\t11\t1\t12\t1\n";
    let result = parse_scen(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

#[test]
fn parsing_3dscen_file() {
    let scen = parse_3dscen_file(Path::new("./tests/A1.3dmap.3dscen")).unwrap();
    assert_eq!(scen[0].map_file, "A1.3dmap");
    assert_eq!(scen[0].start_pos, (101, 109, 191));
    assert_eq!(scen[0].goal_pos, (577, 273, 142));
    assert!((scen[0].optimal_length - 562.04094761).abs() < 1e-6);
    assert!((scen[0].heuristic_ratio - 1.005).abs() < 1e-6);
}

/// Load the actual A1.3dmap and verify that every voxel listed in the file
/// is marked Occupied in the octree — i.e. no voxel is silently dropped
/// during bulk loading (e.g. due to a node-collapse bug).
#[test]
fn parsing_3dmap_no_voxels_lost() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let octree = parse_3dmap_file(Path::new("./tests/A1.3dmap")).unwrap();
    assert_eq!(octree.dimensions(), (896, 390, 255));

    let file = File::open("./tests/A1.3dmap").unwrap();
    let mut lines = BufReader::new(file).lines();
    lines.next(); // skip header

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let coords: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(
            octree.get_voxel((coords[0], coords[1], coords[2])),
            Some(movingai::VoxelState::Occupied),
            "voxel ({},{},{}) was listed as occupied but reads as free",
            coords[0],
            coords[1],
            coords[2]
        );
    }
}

#[test]
fn parsed_3dmap_rejects_body_diagonal_through_obstacle() {
    use movingai::Map3D;

    let map = parse_3dmap_file(Path::new("./tests/A1.3dmap")).unwrap();

    // This transition previously allowed a path shorter than the published
    // optimum for scenario 1884. The occupied face-diagonal intermediate at
    // (550, 254, 136) means a full-voxel agent cannot make this move.
    assert!(!map.neighbors((549, 253, 136)).contains(&(550, 254, 135)));
}

#[test]
fn parsed_3dmap_preserves_declared_dimensions_and_bounds() {
    use movingai::{Map3D, VoxelState};

    let mut map = parse_3dmap("voxel 3 2 1\n2 1 0\n").unwrap();

    assert_eq!(map.dimensions(), (3, 2, 1));
    assert_eq!(map.size(), 4);
    assert_eq!(map.get_voxel((2, 1, 0)), Some(VoxelState::Occupied));

    for outside in [(3, 0, 0), (0, 2, 0), (0, 0, 1)] {
        assert!(map.is_out_of_bounds(outside));
        assert_eq!(map.get_voxel(outside), None);
        assert!(!map.is_free(outside));
        assert!(!map.is_occupied(outside));
        assert!(!map.set_voxel(outside, VoxelState::Occupied));
        assert!(map.get_neighbors(outside).is_empty());
        assert!(map.neighbors(outside).is_empty());
    }

    let edge_neighbors = map.get_neighbors((2, 1, 0));
    assert_eq!(edge_neighbors.len(), 3);
    assert!(
        edge_neighbors
            .iter()
            .all(|(coords, _)| !map.is_out_of_bounds(*coords))
    );
}

#[test]
fn parsed_3dmap_utilities_do_not_modify_padding() {
    use movingai::VoxelState;

    let mut map = parse_3dmap("voxel 3 2 1\n").unwrap();

    assert_eq!(
        map.set_voxels([
            ((0, 0, 0), VoxelState::Occupied),
            ((3, 0, 0), VoxelState::Occupied),
        ]),
        1
    );
    assert_eq!(map.create_box_obstacle((2, 1, 0), (3, 2, 1)), 1);
    assert_eq!(map.count_occupied_in_region((0, 0, 0), (3, 2, 1)), 2);
    assert_eq!(map.get_voxel((2, 1, 0)), Some(VoxelState::Occupied));
    assert_eq!(map.get_voxel((3, 1, 0)), None);

    let mut sphere_map = parse_3dmap("voxel 3 2 1\n").unwrap();
    assert_eq!(sphere_map.create_sphere_obstacle((2, 0, 0), 1.0), 3);
    assert_eq!(sphere_map.get_voxel((3, 0, 0)), None);
}

#[test]
fn parse_3dscen_malformed_line_returns_error() {
    let malformed = "version 1\nA1.3dmap\n101 109";
    let result = parse_3dscen(malformed);
    assert!(result.is_err());
}

use std::path::Path;

use movingai::parser::parse_map_file;
use movingai::parser::parse_scen;
use movingai::parser::parse_scen_file;
use movingai::Map2D;
use movingai::MovingAiMap;

#[test]
fn indexing() {
    let test = MovingAiMap::new(String::from("test"), 4, 6, vec!['.'; 4 * 6]).unwrap();
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
        String::from("octile"),
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
        String::from("octile"),
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
fn parse_scen_empty_line_is_skipped() {
    let input = "version 1\n\n0\tmaps/dao/arena.map\t49\t49\t1\t11\t1\t12\t1\n";
    let result = parse_scen(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

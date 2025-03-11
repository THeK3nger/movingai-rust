# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0 ] - 2025-03-11

### Breaking Changes

- `Map2D:new()` now returns `Result<MovingAiMap, ParseError>` instead of panicking if there are parsing errors or wrong height and width dimensions.

## [1.3.1] - 2024-04-26

### Minor

- Minor mainenance update. I updated the dependencies, the README and documentation.

## [1.3.0] - 2022-12-08

### Changed

- `CoordsIter`'s fields are now public. This allows external crates to implement `Map2D`. Even if currently there is no immediate benefit for that, I don't see why it should not be allowed. (thanks to [Kevin Heavy](https://github.com/kevinheavey)).

## [1.2.0] - 2021-06-16

### Changed

- Now the internal map representation uses a boxed slice. This improve the memory footprint of parsed maps if you are keeping in memory a lot of them.
- New constructor `MovingAIMap::new_from_slice()`. This new constructor allow to directly a boxed slice for the `map` parameter. The old `MovingAIMap::new()` is still the same.

## [1.1.0] - 2019-09-07

### Added

- Add new functions `parse_scen` and `parse_map`. They are just like `parse_scen_file` and `parse_map_file` but they take directly a string as input.

## [0.7.0] - 2018-03-14

### Added

- Add serialization/deserialization using `serde`. You can now serialize/deserialize maps and scen files using `serde` as optional dependency by activating the `serde` feature. Support JSON/YAML and a lot of formats!

## [0.6.0] - 2018-02-27

### Changed

- Breaking. Now parsers need a `path::Path` structure instead of a `&str` for file paths.
- Breaking. Some changes to make movingai-rust compatible to Rust API guidelines.
  - `get_height` renamed into `height`, `get_width` renamed into `width`, `get_cell` renamed into `get`.
  - Rename `coords_iter` into `coords` and `Map2DCoordsIter` into `CoordsIter`.
- Add `Debug`, `PartialEq` and `Clone` to `SceneRecord`.

## [0.5.1] - 2018-02-07

### Changed

- Mostly refactored the examples and documentation.

## [0.5.0] - 2018-02-05

### Changed

- Parser function now returns `Result<T, io::Error>` instead of a static string reference.
- `map.free_states()` returns `usize`.

## [0.4.0] - 2018-01-28

### Added

- New iterator for map coordinates in row-major order.
- New function `map.free_states()` for counting all free states in the map.
- New function `map.neighbors(tile)` returning the accessible neighbors of a tile.

### Fixed

- Scene struct fields are now public.
- `is_traversable_from` now handle corner-cutting and does not consider corner-cut connection as "traversable".

## [0.3.0] - 2018-01-26

### Added

- I add a new API for _traversability_ tests.
  - `is_traversable_from` test if a tile is traversable coming from another tile. This does not perform search, so it works **only** for connecting tiles. We need this because some MovingAI tiles are accessible depending on where the agent comes from (e.g., swamps and water).
  - `is_traversable` just check if a tile is traversable (in some situation) or a plain unpassable object (such as out of bounds or trees).

## [0.2.0] - 2018-01-25

### Changed

- Breaking. `MovingAiMap::create` renamed to `MovingAiMap::new` for Rust consistency.

### Fixed

- Scene parser was reading coordinates in row/column style instead of x/y style.

## [0.1.0] - 2018-01-24

### Added

- Initial Release!

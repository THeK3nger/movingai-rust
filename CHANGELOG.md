# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
 - New iterator for map coordinates in row-major order.
 - New function for counting all free states in the map.

### Fixed
 - Scene struct fields are now public.

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
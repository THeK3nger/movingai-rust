use crate::octree::Octree3D;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Store coordinates in the (x,y,z) format.
pub type Coords3D = (i32, i32, i32);

/// Represents the state of a voxel in the 3D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoxelState {
    /// The voxel is free and can be traversed.
    Free,
    /// The voxel is occupied and cannot be traversed.
    Occupied,
}

/// A read-only interface for 3D voxel maps.
pub trait Map3D {
    /// Returns `true` if the coordinates are outside the map bounds.
    fn is_out_of_bounds(&self, coords: Coords3D) -> bool;
    /// Returns `true` if the voxel at `coords` is traversable (free).
    fn is_traversable(&self, coords: Coords3D) -> bool;
    /// Returns `true` if moving from `from` to `to` is a valid move.
    fn is_traversable_from(&self, from: Coords3D, to: Coords3D) -> bool;
    /// Returns all free neighboring coordinates reachable from `coords`.
    fn neighbors(&self, coords: Coords3D) -> Vec<Coords3D>;
}

/// A 3D voxel map backed by an octree data structure.
///
/// `VoxelMap` is the primary public type for working with 3D maps. It
/// implements the [`Map3D`] trait and exposes additional mutation and
/// query helpers.
pub struct VoxelMap {
    octree: Octree3D,
    dimensions: Coords3D,
}

impl VoxelMap {
    /// Creates a new voxel map with the given size, origin, and default voxel state.
    ///
    /// `size` must be a positive power of 2. The map covers voxels in the
    /// range `[min_coords, min_coords + size)` along each axis.
    pub fn new(size: i32, min_coords: Coords3D, default_state: VoxelState) -> Self {
        Self {
            octree: Octree3D::new(size, min_coords, default_state),
            dimensions: (size, size, size),
        }
    }

    pub(crate) fn from_octree(octree: Octree3D, dimensions: Coords3D) -> Self {
        debug_assert!(dimensions.0 > 0 && dimensions.0 <= octree.size());
        debug_assert!(dimensions.1 > 0 && dimensions.1 <= octree.size());
        debug_assert!(dimensions.2 > 0 && dimensions.2 <= octree.size());
        Self { octree, dimensions }
    }

    /// Sets the state of a voxel at `coords`.
    ///
    /// Returns `true` if the coordinates are within bounds, `false` otherwise.
    pub fn set_voxel(&mut self, coords: Coords3D, state: VoxelState) -> bool {
        self.is_within_bounds(coords) && self.octree.set_voxel(coords, state)
    }

    /// Returns the state of the voxel at `coords`, or `None` if out of bounds.
    pub fn get_voxel(&self, coords: Coords3D) -> Option<VoxelState> {
        self.is_within_bounds(coords)
            .then(|| self.octree.get_voxel(coords))
            .flatten()
    }

    /// Returns the side length of the power-of-two octree storage cube.
    ///
    /// For a parsed rectangular map, use [`VoxelMap::dimensions`] to obtain
    /// the declared map dimensions.
    pub fn size(&self) -> i32 {
        self.octree.size()
    }

    /// Returns the logical map dimensions along the x, y, and z axes.
    pub fn dimensions(&self) -> Coords3D {
        self.dimensions
    }

    /// Returns the minimum coordinates of the map's bounding box.
    pub fn min_coords(&self) -> Coords3D {
        self.octree.min_coords()
    }

    /// Returns `true` if the voxel at `coords` is free and within bounds.
    pub fn is_free(&self, coords: Coords3D) -> bool {
        matches!(self.get_voxel(coords), Some(VoxelState::Free))
    }

    /// Returns `true` if the voxel at `coords` is occupied and within bounds.
    pub fn is_occupied(&self, coords: Coords3D) -> bool {
        matches!(self.get_voxel(coords), Some(VoxelState::Occupied))
    }

    /// Returns all 26 neighbouring voxels (with their states) that are within bounds.
    pub fn get_neighbors(&self, coords: Coords3D) -> Vec<(Coords3D, VoxelState)> {
        if !self.is_within_bounds(coords) {
            return Vec::new();
        }
        let mut neighbors = self.octree.get_neighbors(coords);
        neighbors.retain(|(neighbor, _)| self.is_within_bounds(*neighbor));
        neighbors
    }

    /// Returns the coordinates of all free neighbours reachable from `coords`.
    ///
    /// Diagonal moves are only allowed when every intermediate cardinal step
    /// is also free.
    pub fn get_free_neighbors(&self, coords: Coords3D) -> Vec<Coords3D> {
        if !self.is_within_bounds(coords) {
            return Vec::new();
        }
        let mut neighbors = self.octree.get_free_neighbors(coords);
        neighbors.retain(|&neighbor| self.is_within_bounds(neighbor));
        neighbors
    }

    /// Returns the coordinates of all occupied neighbours of `coords`.
    pub fn get_occupied_neighbors(&self, coords: Coords3D) -> Vec<Coords3D> {
        if !self.is_within_bounds(coords) {
            return Vec::new();
        }
        let mut neighbors = self.octree.get_occupied_neighbors(coords);
        neighbors.retain(|&neighbor| self.is_within_bounds(neighbor));
        neighbors
    }

    /// Sets multiple voxels at once.
    ///
    /// Returns the number of voxels that were successfully set (i.e. in bounds).
    pub fn set_voxels<I>(&mut self, coords_and_states: I) -> usize
    where
        I: IntoIterator<Item = (Coords3D, VoxelState)>,
    {
        coords_and_states
            .into_iter()
            .filter(|&(coords, state)| self.set_voxel(coords, state))
            .count()
    }

    /// Marks all voxels in the axis-aligned box `[min_coords, max_coords]` as
    /// occupied and returns the number of voxels set.
    pub fn create_box_obstacle(&mut self, min_coords: Coords3D, max_coords: Coords3D) -> usize {
        let Some((min_coords, max_coords)) = self.clamp_region(min_coords, max_coords) else {
            return 0;
        };
        self.octree.create_box_obstacle(min_coords, max_coords)
    }

    /// Marks all voxels within `radius` of `center` as occupied and returns
    /// the number of voxels set.
    pub fn create_sphere_obstacle(&mut self, center: Coords3D, radius: f32) -> usize {
        let mut count = 0;
        let (cx, cy, cz) = center;
        let r = radius as i32 + 1;

        for x in (cx - r)..=(cx + r) {
            for y in (cy - r)..=(cy + r) {
                for z in (cz - r)..=(cz + r) {
                    let dx = (x - cx) as f32;
                    let dy = (y - cy) as f32;
                    let dz = (z - cz) as f32;
                    let distance = (dx * dx + dy * dy + dz * dz).sqrt();

                    if distance <= radius && self.set_voxel((x, y, z), VoxelState::Occupied) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Counts the number of occupied voxels inside the region
    /// `[min_coords, max_coords]`.
    pub fn count_occupied_in_region(&self, min_coords: Coords3D, max_coords: Coords3D) -> usize {
        let Some((min_coords, max_coords)) = self.clamp_region(min_coords, max_coords) else {
            return 0;
        };
        self.octree.count_occupied_in_region(min_coords, max_coords)
    }

    fn is_within_bounds(&self, coords: Coords3D) -> bool {
        let min = self.min_coords();
        let local = (
            i64::from(coords.0) - i64::from(min.0),
            i64::from(coords.1) - i64::from(min.1),
            i64::from(coords.2) - i64::from(min.2),
        );
        local.0 >= 0
            && local.0 < i64::from(self.dimensions.0)
            && local.1 >= 0
            && local.1 < i64::from(self.dimensions.1)
            && local.2 >= 0
            && local.2 < i64::from(self.dimensions.2)
    }

    // Clips an inclusive region to the logical map bounds, returning `None` if it is invalid.
    fn clamp_region(
        &self,
        min_coords: Coords3D,
        max_coords: Coords3D,
    ) -> Option<(Coords3D, Coords3D)> {
        if min_coords.0 > max_coords.0 || min_coords.1 > max_coords.1 || min_coords.2 > max_coords.2
        {
            return None;
        }

        let map_min = self.min_coords();
        let map_max = (
            map_min.0.checked_add(self.dimensions.0 - 1)?,
            map_min.1.checked_add(self.dimensions.1 - 1)?,
            map_min.2.checked_add(self.dimensions.2 - 1)?,
        );
        let clipped_min = (
            min_coords.0.max(map_min.0),
            min_coords.1.max(map_min.1),
            min_coords.2.max(map_min.2),
        );
        let clipped_max = (
            max_coords.0.min(map_max.0),
            max_coords.1.min(map_max.1),
            max_coords.2.min(map_max.2),
        );

        (clipped_min.0 <= clipped_max.0
            && clipped_min.1 <= clipped_max.1
            && clipped_min.2 <= clipped_max.2)
            .then_some((clipped_min, clipped_max))
    }
}

impl Map3D for VoxelMap {
    fn is_out_of_bounds(&self, coords: Coords3D) -> bool {
        !self.is_within_bounds(coords)
    }

    fn is_traversable(&self, coords: Coords3D) -> bool {
        self.is_free(coords)
    }

    fn is_traversable_from(&self, from: Coords3D, to: Coords3D) -> bool {
        self.is_within_bounds(from)
            && self.is_within_bounds(to)
            && self.octree.is_traversable_from(from, to)
    }

    fn neighbors(&self, coords: Coords3D) -> Vec<Coords3D> {
        self.get_free_neighbors(coords)
    }
}

/// Represent a row (scene) in a 3D scene file.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SceneRecord3D {
    /// Name of the map file associated to the scene.
    pub map_file: String,

    /// Starting position.
    pub start_pos: Coords3D,

    /// Goal position.
    pub goal_pos: Coords3D,

    /// Optimal length of the path.
    pub optimal_length: f64,

    /// Ratio between the optimal path length and the heuristic.
    pub heuristic_ratio: f64,
}

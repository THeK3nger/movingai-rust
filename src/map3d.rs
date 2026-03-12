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
pub struct VoxelMap(pub(crate) Octree3D);

impl VoxelMap {
    /// Creates a new voxel map with the given size, origin, and default voxel state.
    ///
    /// `size` must be a positive power of 2. The map covers voxels in the
    /// range `[min_coords, min_coords + size)` along each axis.
    pub fn new(size: i32, min_coords: Coords3D, default_state: VoxelState) -> Self {
        VoxelMap(Octree3D::new(size, min_coords, default_state))
    }

    /// Sets the state of a voxel at `coords`.
    ///
    /// Returns `true` if the coordinates are within bounds, `false` otherwise.
    pub fn set_voxel(&mut self, coords: Coords3D, state: VoxelState) -> bool {
        self.0.set_voxel(coords, state)
    }

    /// Returns the state of the voxel at `coords`, or `None` if out of bounds.
    pub fn get_voxel(&self, coords: Coords3D) -> Option<VoxelState> {
        self.0.get_voxel(coords)
    }

    /// Returns the size of the map along each axis.
    pub fn size(&self) -> i32 {
        self.0.size()
    }

    /// Returns the minimum coordinates of the map's bounding box.
    pub fn min_coords(&self) -> Coords3D {
        self.0.min_coords()
    }

    /// Returns `true` if the voxel at `coords` is free and within bounds.
    pub fn is_free(&self, coords: Coords3D) -> bool {
        self.0.is_free(coords)
    }

    /// Returns `true` if the voxel at `coords` is occupied and within bounds.
    pub fn is_occupied(&self, coords: Coords3D) -> bool {
        self.0.is_occupied(coords)
    }

    /// Returns all 26 neighbouring voxels (with their states) that are within bounds.
    pub fn get_neighbors(&self, coords: Coords3D) -> Vec<(Coords3D, VoxelState)> {
        self.0.get_neighbors(coords)
    }

    /// Returns the coordinates of all free neighbours reachable from `coords`.
    ///
    /// Diagonal moves are only allowed when every intermediate cardinal step
    /// is also free.
    pub fn get_free_neighbors(&self, coords: Coords3D) -> Vec<Coords3D> {
        self.0.get_free_neighbors(coords)
    }

    /// Returns the coordinates of all occupied neighbours of `coords`.
    pub fn get_occupied_neighbors(&self, coords: Coords3D) -> Vec<Coords3D> {
        self.0.get_occupied_neighbors(coords)
    }

    /// Sets multiple voxels at once.
    ///
    /// Returns the number of voxels that were successfully set (i.e. in bounds).
    pub fn set_voxels<I>(&mut self, coords_and_states: I) -> usize
    where
        I: IntoIterator<Item = (Coords3D, VoxelState)>,
    {
        self.0.set_voxels(coords_and_states)
    }

    /// Marks all voxels in the axis-aligned box `[min_coords, max_coords]` as
    /// occupied and returns the number of voxels set.
    pub fn create_box_obstacle(&mut self, min_coords: Coords3D, max_coords: Coords3D) -> usize {
        self.0.create_box_obstacle(min_coords, max_coords)
    }

    /// Marks all voxels within `radius` of `center` as occupied and returns
    /// the number of voxels set.
    pub fn create_sphere_obstacle(&mut self, center: Coords3D, radius: f32) -> usize {
        self.0.create_sphere_obstacle(center, radius)
    }

    /// Counts the number of occupied voxels inside the region
    /// `[min_coords, max_coords]`.
    pub fn count_occupied_in_region(&self, min_coords: Coords3D, max_coords: Coords3D) -> usize {
        self.0.count_occupied_in_region(min_coords, max_coords)
    }
}

impl Map3D for VoxelMap {
    fn is_out_of_bounds(&self, coords: Coords3D) -> bool {
        self.0.get_voxel(coords).is_none()
    }

    fn is_traversable(&self, coords: Coords3D) -> bool {
        self.0.is_free(coords)
    }

    fn is_traversable_from(&self, _from: Coords3D, to: Coords3D) -> bool {
        self.0.is_free(to)
    }

    fn neighbors(&self, coords: Coords3D) -> Vec<Coords3D> {
        self.0.get_free_neighbors(coords)
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

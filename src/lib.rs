#![doc(
    html_logo_url = "https://github.com/THeK3nger/movingai-rust/blob/37ad04b72a2e9e8fb7f794c7f1be176fee99b67e/assets/ma.png",
    html_favicon_url = "https://github.com/THeK3nger/movingai-rust/blob/37ad04b72a2e9e8fb7f794c7f1be176fee99b67e/assets/ma.png"
)]
#![deny(missing_docs)]

//!
//! The MovingAI Benchmark Parser
//!
//! # Overview
//!
//! Things.

/// Contains all the parser functions.
pub mod parser;

mod map2d;
mod map3d;

/// Contains data structure for 2D MovingAI maps.
pub use map2d::*;

/// Contains data structures for 3D MovingAI maps.
pub use map3d::*;

/// Contains data structures for efficient 3D map storage using octrees.
///
/// This module provides an efficient octree-based data structure for storing
/// and querying 3D voxel maps. The octree automatically optimizes storage by
/// collapsing uniform regions and subdividing only where necessary.
///
/// # Key Features
///
/// - Efficient storage of sparse 3D data
/// - Fast neighbor queries (all 26 neighboring voxels)
/// - Automatic space optimization through node collapsing
/// - Support for arbitrary coordinate systems
///
/// # Example
///
/// ```rust
/// use movingai::octree::{Octree3D, VoxelState};
///
/// // Create an 8x8x8 3D map
/// let mut map = Octree3D::new(8, (0, 0, 0), VoxelState::Free);
///
/// // Set some obstacles
/// map.set_voxel((3, 3, 3), VoxelState::Occupied);
///
/// // Query neighbors for pathfinding
/// let free_neighbors = map.get_free_neighbors((2, 2, 2));
/// println!("Found {} free neighbors", free_neighbors.len());
/// ```
pub mod octree;

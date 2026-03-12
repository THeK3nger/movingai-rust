use crate::octree::Coords3D;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

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

/// A node in the octree structure.
#[derive(Debug, Clone)]
enum OctreeNode {
    /// A leaf node that stores a uniform voxel state for the entire region.
    Leaf { state: VoxelState },
    /// An internal node that has 8 children (octants).
    Internal { children: Box<[OctreeNode; 8]> },
}

/// An efficient 3D map representation using an octree data structure.
///
/// The octree provides efficient storage for sparse 3D data by subdividing
/// space into octants and storing only the necessary detail levels.
#[derive(Debug, Clone)]
pub struct Octree3D {
    /// The root node of the octree.
    root: OctreeNode,
    /// The size of the octree (must be a power of 2).
    size: i32,
    /// The minimum coordinates of the bounding box.
    min_coords: Coords3D,
}

impl Octree3D {
    /// Creates a new octree with the specified size and default state.
    ///
    /// # Arguments
    /// * `size` - The size of the octree cube (must be a power of 2)
    /// * `min_coords` - The minimum coordinates of the bounding box
    /// * `default_state` - The default state for all voxels
    ///
    /// # Panics
    /// Panics if `size` is not a power of 2 or is less than 1.
    pub fn new(size: i32, min_coords: Coords3D, default_state: VoxelState) -> Self {
        assert!(
            size > 0 && (size & (size - 1)) == 0,
            "Size must be a positive power of 2"
        );

        Self {
            root: OctreeNode::Leaf {
                state: default_state,
            },
            size,
            min_coords,
        }
    }

    /// Sets the state of a voxel at the given coordinates.
    ///
    /// # Arguments
    /// * `coords` - The coordinates of the voxel to set
    /// * `state` - The new state for the voxel
    ///
    /// # Returns
    /// `true` if the coordinates are within bounds, `false` otherwise.
    pub fn set_voxel(&mut self, coords: Coords3D, state: VoxelState) -> bool {
        if !self.is_within_bounds(coords) {
            return false;
        }

        let local_coords = self.to_local_coords(coords);
        Self::set_voxel_recursive(&mut self.root, local_coords, state, 0, 0, 0, self.size);
        true
    }

    /// Gets the state of a voxel at the given coordinates.
    ///
    /// # Arguments
    /// * `coords` - The coordinates of the voxel to query
    ///
    /// # Returns
    /// The state of the voxel, or `None` if coordinates are out of bounds.
    pub fn get_voxel(&self, coords: Coords3D) -> Option<VoxelState> {
        if !self.is_within_bounds(coords) {
            return None;
        }

        let local_coords = self.to_local_coords(coords);
        Some(self.get_voxel_recursive(&self.root, local_coords, 0, 0, 0, self.size))
    }

    /// Gets the states of all 26 neighboring voxels around the given coordinates.
    ///
    /// # Arguments
    /// * `coords` - The center coordinates
    ///
    /// # Returns
    /// A vector of tuples containing (neighbor_coords, voxel_state) for each neighbor.
    /// Only neighbors within bounds are included.
    pub fn get_neighbors(&self, coords: Coords3D) -> Vec<(Coords3D, VoxelState)> {
        if !self.is_within_bounds(coords) {
            return Vec::new();
        }

        let mut neighbors = Vec::with_capacity(26);
        let (x, y, z) = coords;

        // Check all 26 neighbors (3x3x3 cube minus the center)
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue; // Skip the center voxel
                    }

                    let neighbor_coords = (x + dx, y + dy, z + dz);
                    if let Some(state) = self.get_voxel(neighbor_coords) {
                        neighbors.push((neighbor_coords, state));
                    }
                }
            }
        }

        neighbors
    }

    /// Gets only the free neighboring voxels around the given coordinates.
    ///
    /// # Arguments
    /// * `coords` - The center coordinates
    ///
    /// # Returns
    /// A vector of coordinates for free neighboring voxels.
    pub fn get_free_neighbors(&self, coords: Coords3D) -> Vec<Coords3D> {
        let (x, y, z) = coords;
        self.get_neighbors(coords)
            .into_iter()
            .filter_map(|(neighbor, state)| {
                if state != VoxelState::Free {
                    return None;
                }
                let (nx, ny, nz) = neighbor;
                let dx = nx - x;
                let dy = ny - y;
                let dz = nz - z;
                // Diagonal moves are only allowed if each individual cardinal
                // action is also possible (i.e. leads to a free voxel).
                let diagonal = (dx != 0) as u8 + (dy != 0) as u8 + (dz != 0) as u8 > 1;
                if diagonal {
                    if dx != 0 && !self.is_free((x + dx, y, z)) {
                        return None;
                    }
                    if dy != 0 && !self.is_free((x, y + dy, z)) {
                        return None;
                    }
                    if dz != 0 && !self.is_free((x, y, z + dz)) {
                        return None;
                    }
                }
                Some(neighbor)
            })
            .collect()
    }

    /// Gets only the occupied neighboring voxels around the given coordinates.
    ///
    /// # Arguments
    /// * `coords` - The center coordinates
    ///
    /// # Returns
    /// A vector of coordinates for occupied neighboring voxels.
    pub fn get_occupied_neighbors(&self, coords: Coords3D) -> Vec<Coords3D> {
        self.get_neighbors(coords)
            .into_iter()
            .filter_map(|(coords, state)| {
                if state == VoxelState::Occupied {
                    Some(coords)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Checks if the given coordinates are within the octree bounds.
    fn is_within_bounds(&self, coords: Coords3D) -> bool {
        let (x, y, z) = coords;
        let (min_x, min_y, min_z) = self.min_coords;
        let max_x = min_x + self.size;
        let max_y = min_y + self.size;
        let max_z = min_z + self.size;

        x >= min_x && x < max_x && y >= min_y && y < max_y && z >= min_z && z < max_z
    }

    /// Converts global coordinates to local octree coordinates.
    fn to_local_coords(&self, coords: Coords3D) -> Coords3D {
        let (x, y, z) = coords;
        let (min_x, min_y, min_z) = self.min_coords;
        (x - min_x, y - min_y, z - min_z)
    }

    /// Recursively sets a voxel in the octree.
    fn set_voxel_recursive(
        node: &mut OctreeNode,
        coords: Coords3D,
        state: VoxelState,
        node_x: i32,
        node_y: i32,
        node_z: i32,
        node_size: i32,
    ) {
        match node {
            OctreeNode::Leaf {
                state: current_state,
            } => {
                if *current_state != state {
                    if node_size == 1 {
                        *current_state = state;
                    } else {
                        // Split the leaf into 8 children
                        let children = Self::create_children(*current_state);
                        *node = OctreeNode::Internal { children };
                        Self::set_voxel_recursive(
                            node, coords, state, node_x, node_y, node_z, node_size,
                        );
                    }
                }
            }
            OctreeNode::Internal { children } => {
                let half_size = node_size / 2;
                let (x, y, z) = coords;

                let octant = Self::get_octant(x, y, z, node_x, node_y, node_z, half_size);
                let (child_x, child_y, child_z) =
                    Self::get_child_origin(octant, node_x, node_y, node_z, half_size);

                Self::set_voxel_recursive(
                    &mut children[octant],
                    coords,
                    state,
                    child_x,
                    child_y,
                    child_z,
                    half_size,
                );

                // Try to collapse the node if all children have the same state
                Self::try_collapse(node);
            }
        }
    }

    /// Recursively gets a voxel state from the octree.
    fn get_voxel_recursive(
        &self,
        node: &OctreeNode,
        coords: Coords3D,
        node_x: i32,
        node_y: i32,
        node_z: i32,
        node_size: i32,
    ) -> VoxelState {
        match node {
            OctreeNode::Leaf { state } => *state,
            OctreeNode::Internal { children } => {
                let half_size = node_size / 2;
                let (x, y, z) = coords;

                let octant = Self::get_octant(x, y, z, node_x, node_y, node_z, half_size);
                let (child_x, child_y, child_z) =
                    Self::get_child_origin(octant, node_x, node_y, node_z, half_size);

                self.get_voxel_recursive(
                    &children[octant],
                    coords,
                    child_x,
                    child_y,
                    child_z,
                    half_size,
                )
            }
        }
    }

    /// Creates 8 children nodes with the given state.
    fn create_children(state: VoxelState) -> Box<[OctreeNode; 8]> {
        Box::new([
            OctreeNode::Leaf { state },
            OctreeNode::Leaf { state },
            OctreeNode::Leaf { state },
            OctreeNode::Leaf { state },
            OctreeNode::Leaf { state },
            OctreeNode::Leaf { state },
            OctreeNode::Leaf { state },
            OctreeNode::Leaf { state },
        ])
    }

    /// Determines which octant a coordinate belongs to.
    fn get_octant(
        x: i32,
        y: i32,
        z: i32,
        node_x: i32,
        node_y: i32,
        node_z: i32,
        half_size: i32,
    ) -> usize {
        let mut octant = 0;
        if x >= node_x + half_size {
            octant |= 1;
        }
        if y >= node_y + half_size {
            octant |= 2;
        }
        if z >= node_z + half_size {
            octant |= 4;
        }
        octant
    }

    /// Gets the origin coordinates of a child node.
    fn get_child_origin(
        octant: usize,
        node_x: i32,
        node_y: i32,
        node_z: i32,
        half_size: i32,
    ) -> (i32, i32, i32) {
        let child_x = node_x + if octant & 1 != 0 { half_size } else { 0 };
        let child_y = node_y + if octant & 2 != 0 { half_size } else { 0 };
        let child_z = node_z + if octant & 4 != 0 { half_size } else { 0 };
        (child_x, child_y, child_z)
    }

    /// Tries to collapse an internal node if all children have the same state.
    fn try_collapse(node: &mut OctreeNode) {
        if let OctreeNode::Internal { children } = node {
            if let Some(uniform_state) = Self::get_uniform_state(children) {
                *node = OctreeNode::Leaf {
                    state: uniform_state,
                };
            }
        }
    }

    /// Checks if all children have the same state and returns it.
    fn get_uniform_state(children: &[OctreeNode; 8]) -> Option<VoxelState> {
        let first_state = match &children[0] {
            OctreeNode::Leaf { state } => *state,
            OctreeNode::Internal { .. } => return None,
        };

        for child in children.iter() {
            match child {
                OctreeNode::Leaf { state } if *state == first_state => continue,
                _ => return None,
            }
        }

        Some(first_state)
    }

    /// Returns the size of the octree.
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Returns the minimum coordinates of the bounding box.
    pub fn min_coords(&self) -> Coords3D {
        self.min_coords
    }

    /// Checks if a voxel is free (not occupied).
    ///
    /// # Arguments
    /// * `coords` - The coordinates to check
    ///
    /// # Returns
    /// `true` if the voxel is free and within bounds, `false` otherwise.
    pub fn is_free(&self, coords: Coords3D) -> bool {
        matches!(self.get_voxel(coords), Some(VoxelState::Free))
    }

    /// Checks if a voxel is occupied.
    ///
    /// # Arguments
    /// * `coords` - The coordinates to check
    ///
    /// # Returns
    /// `true` if the voxel is occupied and within bounds, `false` otherwise.
    pub fn is_occupied(&self, coords: Coords3D) -> bool {
        matches!(self.get_voxel(coords), Some(VoxelState::Occupied))
    }

    /// Sets multiple voxels at once.
    ///
    /// # Arguments
    /// * `coords_and_states` - An iterator of (coordinates, state) pairs
    ///
    /// # Returns
    /// The number of voxels successfully set.
    pub fn set_voxels<I>(&mut self, coords_and_states: I) -> usize
    where
        I: IntoIterator<Item = (Coords3D, VoxelState)>,
    {
        let mut count = 0;
        for (coords, state) in coords_and_states {
            if self.set_voxel(coords, state) {
                count += 1;
            }
        }
        count
    }

    /// Counts the number of occupied voxels in a given region.
    ///
    /// Uses recursive tree traversal to skip entire subtrees when a node
    /// is fully contained within the query region, avoiding per-voxel iteration.
    ///
    /// # Arguments
    /// * `min_coords` - The minimum coordinates of the region (inclusive)
    /// * `max_coords` - The maximum coordinates of the region (inclusive)
    ///
    /// # Returns
    /// The number of occupied voxels in the region.
    pub fn count_occupied_in_region(&self, min_coords: Coords3D, max_coords: Coords3D) -> usize {
        let local_min = self.to_local_coords(min_coords);
        let local_max = self.to_local_coords(max_coords);
        Self::count_occupied_recursive(&self.root, local_min, local_max, 0, 0, 0, self.size)
    }

    /// Recursively counts occupied voxels in a region by leveraging the tree structure.
    fn count_occupied_recursive(
        node: &OctreeNode,
        region_min: Coords3D,
        region_max: Coords3D,
        node_x: i32,
        node_y: i32,
        node_z: i32,
        node_size: i32,
    ) -> usize {
        let node_max_x = node_x + node_size - 1;
        let node_max_y = node_y + node_size - 1;
        let node_max_z = node_z + node_size - 1;

        // No overlap between node and region
        if node_x > region_max.0
            || node_max_x < region_min.0
            || node_y > region_max.1
            || node_max_y < region_min.1
            || node_z > region_max.2
            || node_max_z < region_min.2
        {
            return 0;
        }

        // Check if the node is fully contained within the region
        let fully_contained = node_x >= region_min.0
            && node_max_x <= region_max.0
            && node_y >= region_min.1
            && node_max_y <= region_max.1
            && node_z >= region_min.2
            && node_max_z <= region_max.2;

        match node {
            OctreeNode::Leaf { state } => {
                if *state != VoxelState::Occupied {
                    return 0;
                }
                if fully_contained {
                    (node_size as usize).pow(3)
                } else {
                    // Partial overlap with a leaf — count the intersection
                    let ox = (region_min.0.max(node_x), region_max.0.min(node_max_x));
                    let oy = (region_min.1.max(node_y), region_max.1.min(node_max_y));
                    let oz = (region_min.2.max(node_z), region_max.2.min(node_max_z));
                    ((ox.1 - ox.0 + 1) * (oy.1 - oy.0 + 1) * (oz.1 - oz.0 + 1)) as usize
                }
            }
            OctreeNode::Internal { children } => {
                let half_size = node_size / 2;
                let mut count = 0;
                for octant in 0..8 {
                    let (child_x, child_y, child_z) =
                        Self::get_child_origin(octant, node_x, node_y, node_z, half_size);
                    count += Self::count_occupied_recursive(
                        &children[octant],
                        region_min,
                        region_max,
                        child_x,
                        child_y,
                        child_z,
                        half_size,
                    );
                }
                count
            }
        }
    }

    /// Creates a box-shaped obstacle in the 3D space.
    ///
    /// Uses recursive tree traversal to set entire subtrees at once when a node
    /// is fully contained within the box, avoiding per-voxel tree traversals.
    ///
    /// # Arguments
    /// * `min_coords` - The minimum corner of the box (inclusive)
    /// * `max_coords` - The maximum corner of the box (inclusive)
    ///
    /// # Returns
    /// The number of voxels set as occupied.
    pub fn create_box_obstacle(&mut self, min_coords: Coords3D, max_coords: Coords3D) -> usize {
        let local_min = self.to_local_coords(min_coords);
        let local_max = self.to_local_coords(max_coords);
        Self::set_region_recursive(
            &mut self.root,
            local_min,
            local_max,
            VoxelState::Occupied,
            0,
            0,
            0,
            self.size,
        )
    }

    /// Recursively sets all voxels in a region to the given state.
    fn set_region_recursive(
        node: &mut OctreeNode,
        region_min: Coords3D,
        region_max: Coords3D,
        state: VoxelState,
        node_x: i32,
        node_y: i32,
        node_z: i32,
        node_size: i32,
    ) -> usize {
        let node_max_x = node_x + node_size - 1;
        let node_max_y = node_y + node_size - 1;
        let node_max_z = node_z + node_size - 1;

        // No overlap between node and region
        if node_x > region_max.0
            || node_max_x < region_min.0
            || node_y > region_max.1
            || node_max_y < region_min.1
            || node_z > region_max.2
            || node_max_z < region_min.2
        {
            return 0;
        }

        // Check if the node is fully contained within the region
        let fully_contained = node_x >= region_min.0
            && node_max_x <= region_max.0
            && node_y >= region_min.1
            && node_max_y <= region_max.1
            && node_z >= region_min.2
            && node_max_z <= region_max.2;

        if fully_contained {
            *node = OctreeNode::Leaf { state };
            return (node_size as usize).pow(3);
        }

        // Partial overlap — need to split if this is a leaf
        match node {
            OctreeNode::Leaf {
                state: current_state,
            } => {
                if *current_state == state {
                    // Already the target state; count the intersection
                    let ox = (region_min.0.max(node_x), region_max.0.min(node_max_x));
                    let oy = (region_min.1.max(node_y), region_max.1.min(node_max_y));
                    let oz = (region_min.2.max(node_z), region_max.2.min(node_max_z));
                    return ((ox.1 - ox.0 + 1) * (oy.1 - oy.0 + 1) * (oz.1 - oz.0 + 1)) as usize;
                }
                // Split this leaf into children, then recurse
                let children = Self::create_children(*current_state);
                *node = OctreeNode::Internal { children };
                Self::set_region_recursive(
                    node, region_min, region_max, state, node_x, node_y, node_z, node_size,
                )
            }
            OctreeNode::Internal { children } => {
                let half_size = node_size / 2;
                let mut count = 0;
                for octant in 0..8 {
                    let (child_x, child_y, child_z) =
                        Self::get_child_origin(octant, node_x, node_y, node_z, half_size);
                    count += Self::set_region_recursive(
                        &mut children[octant],
                        region_min,
                        region_max,
                        state,
                        child_x,
                        child_y,
                        child_z,
                        half_size,
                    );
                }
                // Try to collapse if all children now share the same state
                Self::try_collapse(node);
                count
            }
        }
    }

    /// Creates a sphere-shaped obstacle in the 3D space.
    ///
    /// # Arguments
    /// * `center` - The center of the sphere
    /// * `radius` - The radius of the sphere
    ///
    /// # Returns
    /// The number of voxels set as occupied.
    pub fn create_sphere_obstacle(&mut self, center: Coords3D, radius: f32) -> usize {
        let mut count = 0;
        let (cx, cy, cz) = center;
        let r = radius as i32 + 1; // Include boundary

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_creation() {
        let octree = Octree3D::new(8, (0, 0, 0), VoxelState::Free);
        assert_eq!(octree.size(), 8);
        assert_eq!(octree.min_coords(), (0, 0, 0));

        // Calculate max coords manually
        let (min_x, min_y, min_z) = octree.min_coords();
        let expected_max = (
            min_x + octree.size() - 1,
            min_y + octree.size() - 1,
            min_z + octree.size() - 1,
        );
        assert_eq!(expected_max, (7, 7, 7));
    }

    #[test]
    fn test_voxel_operations() {
        let mut octree = Octree3D::new(8, (0, 0, 0), VoxelState::Free);

        // Test setting and getting a voxel
        assert!(octree.set_voxel((2, 3, 4), VoxelState::Occupied));
        assert_eq!(octree.get_voxel((2, 3, 4)), Some(VoxelState::Occupied));
        assert_eq!(octree.get_voxel((0, 0, 0)), Some(VoxelState::Free));

        // Test out of bounds
        assert!(!octree.set_voxel((10, 10, 10), VoxelState::Occupied));
        assert_eq!(octree.get_voxel((10, 10, 10)), None);
    }

    #[test]
    fn test_neighbors() {
        let mut octree = Octree3D::new(8, (0, 0, 0), VoxelState::Free);

        // Set some occupied voxels
        octree.set_voxel((1, 1, 1), VoxelState::Occupied);
        octree.set_voxel((2, 1, 1), VoxelState::Occupied);
        octree.set_voxel((1, 2, 1), VoxelState::Occupied);

        // Test neighbors of (1, 1, 1)
        let neighbors = octree.get_neighbors((1, 1, 1));
        assert_eq!(neighbors.len(), 26); // All neighbors should be within bounds

        // Test free neighbors: diagonals through (2,1,1) or (1,2,1) are blocked.
        let free_neighbors = octree.get_free_neighbors((1, 1, 1));
        assert_eq!(free_neighbors.len(), 11);

        // Test occupied neighbors
        let occupied_neighbors = octree.get_occupied_neighbors((1, 1, 1));
        assert_eq!(occupied_neighbors.len(), 2); // (2,1,1) and (1,2,1)
    }

    #[test]
    fn test_neighbors_out_of_bounds_center() {
        let octree = Octree3D::new(8, (0, 0, 0), VoxelState::Free);

        assert!(octree.get_neighbors((-1, 0, 0)).is_empty());
        assert!(octree.get_free_neighbors((0, -1, 0)).is_empty());
        assert!(octree.get_occupied_neighbors((0, 0, 9)).is_empty());
    }

    #[test]
    fn test_negative_coordinates() {
        let mut octree = Octree3D::new(8, (-4, -4, -4), VoxelState::Free);

        assert!(octree.set_voxel((-2, -2, -2), VoxelState::Occupied));
        assert_eq!(octree.get_voxel((-2, -2, -2)), Some(VoxelState::Occupied));

        let neighbors = octree.get_neighbors((-2, -2, -2));
        assert_eq!(neighbors.len(), 26);
    }

    #[test]
    fn test_utility_methods() {
        let mut octree = Octree3D::new(8, (0, 0, 0), VoxelState::Free);

        // Test is_free and is_occupied
        assert!(octree.is_free((1, 1, 1)));
        assert!(!octree.is_occupied((1, 1, 1)));

        octree.set_voxel((1, 1, 1), VoxelState::Occupied);
        assert!(!octree.is_free((1, 1, 1)));
        assert!(octree.is_occupied((1, 1, 1)));

        // Test set_voxels
        let coords_and_states = vec![
            ((2, 2, 2), VoxelState::Occupied),
            ((3, 3, 3), VoxelState::Occupied),
            ((10, 10, 10), VoxelState::Occupied), // Out of bounds
        ];

        let count = octree.set_voxels(coords_and_states);
        assert_eq!(count, 2); // Only 2 should be set (third is out of bounds)

        // Test count_occupied_in_region
        let occupied_count = octree.count_occupied_in_region((0, 0, 0), (4, 4, 4));
        assert_eq!(occupied_count, 3); // (1,1,1), (2,2,2), (3,3,3)
    }

    #[test]
    fn test_box_obstacle() {
        let mut octree = Octree3D::new(8, (0, 0, 0), VoxelState::Free);

        let count = octree.create_box_obstacle((2, 2, 2), (4, 4, 4));
        assert_eq!(count, 27); // 3x3x3 = 27 voxels

        // Check that the box is actually created
        assert!(octree.is_occupied((2, 2, 2)));
        assert!(octree.is_occupied((3, 3, 3)));
        assert!(octree.is_occupied((4, 4, 4)));
        assert!(octree.is_free((1, 1, 1))); // Outside the box
        assert!(octree.is_free((5, 5, 5))); // Outside the box
    }

    #[test]
    fn test_sphere_obstacle() {
        let mut octree = Octree3D::new(16, (0, 0, 0), VoxelState::Free);

        let center = (8, 8, 8);
        let radius = 2.0;
        let count = octree.create_sphere_obstacle(center, radius);

        assert!(count > 0);

        // Check that the center is occupied
        assert!(octree.is_occupied(center));

        // Check that voxels within radius are occupied
        assert!(octree.is_occupied((7, 8, 8))); // Distance = 1
        assert!(octree.is_occupied((8, 7, 8))); // Distance = 1

        // Check that voxels outside radius are free
        assert!(octree.is_free((5, 8, 8))); // Distance = 3 > radius
    }
}

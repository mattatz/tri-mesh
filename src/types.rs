//! Common types used throughout the crate

use crate::math::Vec3;

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AxisAlignedBoundingBox {
    min: Vec3,
    max: Vec3,
}

impl AxisAlignedBoundingBox {
    /// Creates a new axis-aligned bounding box from minimum and maximum points
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Creates a new axis-aligned bounding box from a set of positions
    pub fn new_with_positions(positions: &[nalgebra::Vector3<f32>]) -> Self {
        if positions.is_empty() {
            return Self {
                min: Vec3::zeros(),
                max: Vec3::zeros(),
            };
        }

        let mut min = Vec3::new(
            positions[0].x as f64,
            positions[0].y as f64,
            positions[0].z as f64,
        );
        let mut max = min;

        for pos in positions.iter().skip(1) {
            min.x = min.x.min(pos.x as f64);
            min.y = min.y.min(pos.y as f64);
            min.z = min.z.min(pos.z as f64);
            max.x = max.x.max(pos.x as f64);
            max.y = max.y.max(pos.y as f64);
            max.z = max.z.max(pos.z as f64);
        }

        Self { min, max }
    }

    /// Returns the minimum point of the bounding box
    pub fn min(&self) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(self.min.x as f32, self.min.y as f32, self.min.z as f32)
    }

    /// Returns the maximum point of the bounding box
    pub fn max(&self) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(self.max.x as f32, self.max.y as f32, self.max.z as f32)
    }

    /// Returns the center of the bounding box
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// Returns the size of the bounding box
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }
}

/// Represents mesh positions
#[derive(Debug, Clone, PartialEq)]
pub enum Positions {
    /// 32-bit positions
    F32(Vec<[f32; 3]>),
    /// 64-bit positions
    F64(Vec<[f64; 3]>),
}

impl Positions {
    /// Returns the number of positions
    pub fn len(&self) -> usize {
        match self {
            Positions::F32(v) => v.len(),
            Positions::F64(v) => v.len(),
        }
    }

    /// Returns whether the positions are empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets a position at the given index as f64 array
    pub fn get(&self, index: usize) -> Option<[f64; 3]> {
        match self {
            Positions::F32(v) => v
                .get(index)
                .map(|p| [p[0] as f64, p[1] as f64, p[2] as f64]),
            Positions::F64(v) => v.get(index).copied(),
        }
    }
}

/// Represents mesh indices
#[derive(Debug, Clone, PartialEq)]
pub enum Indices {
    /// 8-bit indices
    U8(Vec<u8>),
    /// 16-bit indices
    U16(Vec<u16>),
    /// 32-bit indices
    U32(Vec<u32>),
}

impl Indices {
    /// Returns the number of indices
    pub fn len(&self) -> usize {
        match self {
            Indices::U8(v) => v.len(),
            Indices::U16(v) => v.len(),
            Indices::U32(v) => v.len(),
        }
    }

    /// Returns whether the indices are empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets an index at the given position as usize
    pub fn get(&self, index: usize) -> Option<usize> {
        match self {
            Indices::U8(v) => v.get(index).map(|&i| i as usize),
            Indices::U16(v) => v.get(index).map(|&i| i as usize),
            Indices::U32(v) => v.get(index).map(|&i| i as usize),
        }
    }
}

/// Simple triangle mesh structure for testing
#[derive(Debug, Clone, PartialEq)]
pub struct MeshSource {
    /// The positions of the mesh
    pub positions: Positions,
    /// The indices of the mesh
    pub indices: Option<Indices>,
    /// The normals of the mesh
    pub normals: Option<Positions>,
}

impl Default for MeshSource {
    fn default() -> Self {
        Self {
            positions: Positions::F64(vec![]),
            indices: None,
            normals: None,
        }
    }
}

impl MeshSource {
    /// Creates a unit sphere mesh for testing
    pub fn sphere(_subdivisions: u32) -> Self {
        // Simple icosphere generation
        let t = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let vertices = vec![
            [-1.0, t, 0.0],
            [1.0, t, 0.0],
            [-1.0, -t, 0.0],
            [1.0, -t, 0.0],
            [0.0, -1.0, t],
            [0.0, 1.0, t],
            [0.0, -1.0, -t],
            [0.0, 1.0, -t],
            [t, 0.0, -1.0],
            [t, 0.0, 1.0],
            [-t, 0.0, -1.0],
            [-t, 0.0, 1.0],
        ];

        // Normalize vertices to unit sphere
        let positions: Vec<[f64; 3]> = vertices
            .into_iter()
            .map(|v| {
                let norm = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                [v[0] / norm, v[1] / norm, v[2] / norm]
            })
            .collect();

        let indices = vec![
            0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11, 1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7,
            6, 7, 1, 8, 3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9, 4, 9, 5, 2, 4, 11, 6, 2, 10,
            8, 6, 7, 9, 8, 1,
        ];

        Self {
            positions: Positions::F64(positions),
            indices: Some(Indices::U32(indices)),
            normals: None,
        }
    }

    /// Creates a cylinder mesh for testing
    pub fn cylinder(segments: u32) -> Self {
        let mut positions = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices
        for i in 0..segments {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (segments as f64);
            let x = angle.cos();
            let z = angle.sin();

            // Bottom circle
            positions.push([x, -1.0, z]);
            // Top circle
            positions.push([x, 1.0, z]);
        }

        // Generate indices for the sides
        for i in 0..segments {
            let i1 = i * 2;
            let i2 = i * 2 + 1;
            let i3 = ((i + 1) % segments) * 2;
            let i4 = ((i + 1) % segments) * 2 + 1;

            // First triangle
            indices.push(i1);
            indices.push(i2);
            indices.push(i3);

            // Second triangle
            indices.push(i3);
            indices.push(i2);
            indices.push(i4);
        }

        // Add center vertices for caps
        let bottom_center = positions.len() as u32;
        positions.push([0.0, -1.0, 0.0]);
        let top_center = positions.len() as u32;
        positions.push([0.0, 1.0, 0.0]);

        // Generate indices for caps
        for i in 0..segments {
            let i1 = i * 2;
            let i3 = ((i + 1) % segments) * 2;

            // Bottom cap
            indices.push(bottom_center);
            indices.push(i3);
            indices.push(i1);

            // Top cap
            let i2 = i * 2 + 1;
            let i4 = ((i + 1) % segments) * 2 + 1;
            indices.push(top_center);
            indices.push(i2);
            indices.push(i4);
        }

        Self {
            positions: Positions::F64(positions),
            indices: Some(Indices::U32(indices)),
            normals: None,
        }
    }

    /// Creates a cube mesh for testing
    pub fn cube() -> Self {
        MeshSource {
            indices: Some(Indices::U8(vec![
                0, 1, 2, 0, 2, 3, 4, 7, 6, 4, 6, 5, 0, 4, 5, 0, 5, 1, 1, 5, 6, 1, 6, 2, 2, 6, 7, 2,
                7, 3, 4, 0, 3, 4, 3, 7,
            ])),
            positions: Positions::F64(vec![
                [1.0, -1.0, -1.0],
                [1.0, -1.0, 1.0],
                [-1.0, -1.0, 1.0],
                [-1.0, -1.0, -1.0],
                [1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
                [-1.0, 1.0, 1.0],
                [-1.0, 1.0, -1.0],
            ]),
            ..Default::default()
        }
    }
}

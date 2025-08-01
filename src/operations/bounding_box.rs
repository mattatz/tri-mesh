//! See [Mesh](crate::mesh::Mesh).

use crate::mesh::*;
pub use crate::types::AxisAlignedBoundingBox;

/// # Bounding box
impl Mesh {
    /// Returns the smallest axis aligned box which contains the entire mesh, ie. the axis aligned bounding box.
    pub fn axis_aligned_bounding_box(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new_with_positions(
            &self
                .vertex_iter()
                .map(|v| {
                    let pos = self.position(v);
                    nalgebra::Vector3::<f32>::new(pos.x as f32, pos.y as f32, pos.z as f32)
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod tests {}

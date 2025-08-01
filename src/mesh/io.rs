//! See [Mesh](crate::mesh::Mesh).

use crate::mesh::*;
use crate::types::{Indices, MeshSource, Positions};

impl Mesh {
    ///
    /// Constructs a new [Mesh] from a [MeshSource] which can either be manually constructed or from test generation functions.
    ///
    /// # Examples
    /// ```
    /// use tri_mesh::*;
    /// use tri_mesh::types::MeshSource;
    /// let mesh = Mesh::new(&MeshSource::sphere(4));
    /// ```
    ///
    /// ```
    /// use tri_mesh::*;
    /// use tri_mesh::types::{MeshSource, Positions, Indices};
    /// let mesh = Mesh::new(&MeshSource {
    ///     positions: Positions::F64(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]),
    ///     indices: None,
    ///     normals: None,
    /// });
    /// ```
    ///
    pub fn new(input: &MeshSource) -> Self {
        let no_vertices = input.positions.len();
        let no_faces = match &input.indices {
            Some(indices) => indices.len() / 3,
            None => no_vertices / 3,
        };

        let mesh = Mesh {
            connectivity_info: ConnectivityInfo::new(no_vertices, no_faces),
        };

        // Create vertices
        for i in 0..no_vertices {
            let pos = input.positions.get(i).unwrap();
            mesh.connectivity_info
                .new_vertex(vec3(pos[0], pos[1], pos[2]));
        }

        let mut twins = HashMap::<(VertexID, VertexID), HalfEdgeID>::new();
        fn sort(a: VertexID, b: VertexID) -> (VertexID, VertexID) {
            if a < b {
                (a, b)
            } else {
                (b, a)
            }
        }

        // Create faces and twin connectivity
        for face in 0..no_faces {
            let (v0, v1, v2) = match &input.indices {
                Some(indices) => (
                    indices.get(face * 3).unwrap() as u32,
                    indices.get(face * 3 + 1).unwrap() as u32,
                    indices.get(face * 3 + 2).unwrap() as u32,
                ),
                None => (
                    (face * 3) as u32,
                    (face * 3 + 1) as u32,
                    (face * 3 + 2) as u32,
                ),
            };

            let face = mesh.connectivity_info.create_face(
                unsafe { VertexID::new(v0) },
                unsafe { VertexID::new(v1) },
                unsafe { VertexID::new(v2) },
            );

            // mark twin halfedges
            let mut walker = mesh.walker_from_face(face);
            for _ in 0..3 {
                let vertex_id = walker.vertex_id().unwrap();
                walker.as_next();
                let key = sort(vertex_id, walker.vertex_id().unwrap());
                if let Some(twin) = twins.get(&key) {
                    mesh.connectivity_info
                        .set_halfedge_twin(walker.halfedge_id().unwrap(), *twin);
                } else {
                    twins.insert(key, walker.halfedge_id().unwrap());
                }
            }
        }
        for halfedge in mesh.connectivity_info.halfedge_iterator() {
            if mesh
                .connectivity_info
                .halfedge(halfedge)
                .unwrap()
                .twin
                .is_none()
            {
                let vertex = mesh
                    .walker_from_halfedge(halfedge)
                    .as_previous()
                    .vertex_id()
                    .unwrap();
                mesh.connectivity_info.set_halfedge_twin(
                    halfedge,
                    mesh.connectivity_info
                        .new_halfedge(Some(vertex), None, None),
                );
            }
        }

        mesh
    }

    ///
    /// Exports the [Mesh] into a [MeshSource] that contain the raw buffer data.
    ///
    pub fn export(&self) -> MeshSource {
        let vertices: Vec<VertexID> = self.vertex_iter().collect();
        let mut indices = Vec::with_capacity(self.no_faces() * 3);
        for face_id in self.face_iter() {
            for halfedge_id in self.face_halfedge_iter(face_id) {
                let vertex_id = self.walker_from_halfedge(halfedge_id).vertex_id().unwrap();
                let index = vertices.iter().position(|v| v == &vertex_id).unwrap();
                indices.push(index as u32);
            }
        }

        let positions: Vec<[f64; 3]> = self
            .vertex_iter()
            .map(|vertex_id| {
                let pos = self.vertex_position(vertex_id);
                [pos.x, pos.y, pos.z]
            })
            .collect();

        let normals: Vec<[f64; 3]> = self
            .vertex_iter()
            .map(|vertex_id| {
                let normal = self.vertex_normal(vertex_id);
                [normal.x, normal.y, normal.z]
            })
            .collect();

        MeshSource {
            indices: Some(Indices::U32(indices)),
            positions: Positions::F64(positions),
            normals: Some(Positions::F64(normals)),
        }
    }
}

impl From<MeshSource> for Mesh {
    fn from(mesh: MeshSource) -> Self {
        Self::new(&mesh)
    }
}

impl From<&MeshSource> for Mesh {
    fn from(mesh: &MeshSource) -> Self {
        Self::new(mesh)
    }
}

impl From<Mesh> for MeshSource {
    fn from(mesh: Mesh) -> Self {
        mesh.export()
    }
}

impl From<&Mesh> for MeshSource {
    fn from(mesh: &Mesh) -> Self {
        mesh.export()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_export() {
        let mesh: Mesh = MeshSource::cylinder(16).into();
        let m: MeshSource = (&mesh).into();

        assert_eq!(m.indices.as_ref().unwrap().len() / 3, mesh.no_faces());
        assert_eq!(m.positions.len(), mesh.no_vertices());

        for face in 0..mesh.no_faces() {
            if let Some(indices) = &m.indices {
                let i0 = indices.get(face * 3).unwrap();
                let i1 = indices.get(face * 3 + 1).unwrap();
                let i2 = indices.get(face * 3 + 2).unwrap();

                let id0 = unsafe { VertexID::new(i0 as u32) };
                let id1 = unsafe { VertexID::new(i1 as u32) };
                let id2 = unsafe { VertexID::new(i2 as u32) };

                let p0 = m.positions.get(i0).unwrap();
                let p1 = m.positions.get(i1).unwrap();
                let p2 = m.positions.get(i2).unwrap();

                let mesh_p0 = mesh.vertex_position(id0);
                let mesh_p1 = mesh.vertex_position(id1);
                let mesh_p2 = mesh.vertex_position(id2);

                assert!((vec3(p0[0], p0[1], p0[2]) - mesh_p0).magnitude() < 0.001);
                assert!((vec3(p1[0], p1[1], p1[2]) - mesh_p1).magnitude() < 0.001);
                assert!((vec3(p2[0], p2[1], p2[2]) - mesh_p2).magnitude() < 0.001);
            }
        }
    }

    #[test]
    fn test_new_from_positions() {
        let mesh: Mesh = MeshSource {
            positions: Positions::F64(vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 1.0, 1.0],
                [1.0, 0.0, 1.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 1.0],
                [0.0, 1.0, 1.0],
            ]),
            indices: None,
            normals: None,
        }
        .into();

        assert_eq!(9, mesh.no_vertices());
        assert_eq!(3, mesh.no_faces());
        mesh.is_valid().unwrap();
    }
}

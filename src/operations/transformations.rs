//! See [Mesh](crate::mesh::Mesh).

use crate::mesh::*;

/// # Transformations
impl Mesh {
    /// Moves the vertex to the specified position.
    pub fn move_vertex_to(&mut self, vertex_id: VertexID, value: Vec3) {
        self.set_vertex_position(vertex_id, value);
    }

    /// Moves the vertex by the specified vector, i.e. the new position is `mesh.vertex_position(vertex_id) + value`.
    pub fn move_vertex_by(&mut self, vertex_id: VertexID, value: Vec3) {
        let p = value + self.vertex_position(vertex_id);
        self.move_vertex_to(vertex_id, p);
    }

    /// Scales the entire mesh by multiplying `scale` to each vertex position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tri_mesh::*;
    /// #
    /// # fn main() {
    /// #   use tri_mesh::*;
    ///     let mut mesh: Mesh = MeshSource::sphere(4).into();
    /// #   let first_face_id = mesh.face_iter().next().unwrap();
    /// #   let face_area_before = mesh.face_area(first_face_id);
    ///     mesh.scale(2.0);
    /// #   let face_area_after = mesh.face_area(first_face_id);
    /// #   assert_eq!(4.0 * face_area_before, face_area_after);
    /// #   mesh.is_valid().unwrap();
    /// # }
    /// ```
    ///
    pub fn scale(&mut self, scale: f64) {
        for vertex_id in self.vertex_iter() {
            let p = self.vertex_position(vertex_id);
            self.move_vertex_to(vertex_id, p * scale);
        }
    }

    /// Scales the entire mesh by multiplying `scale_x` to the x component of each vertex position, `scale_y` to the y component and `scale_z` to the z component.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tri_mesh::*;
    /// #
    /// # fn main() {
    ///     let mut mesh: Mesh = MeshSource::sphere(4).into();
    /// #   let mut iter = mesh.vertex_iter();
    /// #   iter.next();
    /// #   let vertex0_pos = mesh.vertex_position(iter.next().unwrap());
    /// #   let vertex1_pos = mesh.vertex_position(iter.next().unwrap());
    ///     mesh.non_uniform_scale(2.0, 1.0, 1.0);
    /// #   let mut iter = mesh.vertex_iter();
    /// #   iter.next();
    /// #   let vertex0_pos_new = mesh.vertex_position(iter.next().unwrap());
    /// #   let vertex1_pos_new = mesh.vertex_position(iter.next().unwrap());
    /// #   assert_eq!(vertex0_pos.x * 2.0, vertex0_pos_new.x);
    /// #   assert_eq!(vertex0_pos.y, vertex0_pos_new.y);
    /// #   assert_eq!(vertex0_pos.z, vertex0_pos_new.z);
    /// #   assert_eq!(vertex1_pos.x * 2.0, vertex1_pos_new.x);
    /// #   assert_eq!(vertex1_pos.y, vertex1_pos_new.y);
    /// #   assert_eq!(vertex1_pos.z, vertex1_pos_new.z);
    /// #   mesh.is_valid().unwrap();
    /// # }
    /// ```
    ///
    pub fn non_uniform_scale(&mut self, scale_x: f64, scale_y: f64, scale_z: f64) {
        for vertex_id in self.vertex_iter() {
            let p = self.vertex_position(vertex_id);
            self.move_vertex_to(vertex_id, vec3(p.x * scale_x, p.y * scale_y, p.z * scale_z));
        }
    }

    /// Translates the entire mesh by applying the `translation` to each vertex position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tri_mesh::*;
    /// #
    /// # fn main() {
    ///     let mut mesh: Mesh = MeshSource::sphere(4).into();
    /// #   let first_vertex_id = mesh.vertex_iter().next().unwrap();
    /// #   let vertex_position_before = mesh.vertex_position(first_vertex_id);
    ///     mesh.translate(vec3(2.5, -1.0, 0.0));
    /// #   let vertex_position_after = mesh.vertex_position(first_vertex_id);
    /// #   assert_eq!(vertex_position_before + vec3(2.5, -1.0, 0.0), vertex_position_after);
    /// #   mesh.is_valid().unwrap();
    /// # }
    /// ```
    ///
    pub fn translate(&mut self, translation: Vec3) {
        for vertex_id in self.vertex_iter() {
            self.move_vertex_by(vertex_id, translation);
        }
    }

    ///
    /// Transforms the entire mesh by applying the `transformation` to each vertex position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tri_mesh::*;
    /// #
    /// # fn main() {
    ///     let mut mesh: Mesh = MeshSource::sphere(4).into();
    /// #   let first_vertex_id = mesh.vertex_iter().next().unwrap();
    /// #   let vertex_position_before = mesh.vertex_position(first_vertex_id);
    ///     mesh.apply_transformation(Mat4::new_translation(&vec3(2.5, -1.0, 0.0)));
    /// #   let vertex_position_after = mesh.vertex_position(first_vertex_id);
    /// #   assert_eq!(vertex_position_before + vec3(2.5, -1.0, 0.0), vertex_position_after);
    /// #   mesh.is_valid().unwrap();
    /// # }
    /// ```
    ///
    pub fn apply_transformation(&mut self, transformation: Mat4) {
        for vertex_id in self.vertex_iter() {
            let p = self.vertex_position(vertex_id);
            let p4 = Vec4::new(p.x, p.y, p.z, 1.0);
            let result = transformation * p4;
            let p_new = Vec3::new(result.x, result.y, result.z);
            self.move_vertex_to(vertex_id, p_new);
        }
    }
}

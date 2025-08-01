//!
//! Linear algebra types for vector calculations. Using nalgebra library.
//!

pub use na::{Matrix3, Matrix4, Vector3, Vector4};
use nalgebra as na;

/// Vector with three elements.
pub type Vec3 = Vector3<f64>;
/// Vector with four elements.
pub type Vec4 = Vector4<f64>;

/// 3x3 matrix.
pub type Mat3 = Matrix3<f64>;
/// 4x4 matrix.
pub type Mat4 = Matrix4<f64>;

/// Degrees
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Deg<T>(pub T);

/// Radians
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rad<T>(pub T);

/// Degrees
pub type Degrees = Deg<f64>;
/// Radians
pub type Radians = Rad<f64>;

/// Constructs a [Vec3]
pub const fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vector3::new(x, y, z)
}

/// Constructs a [Vec4]
pub const fn vec4(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
    Vector4::new(x, y, z, w)
}

/// Constructs a [Degrees]
pub const fn degrees(v: f64) -> Degrees {
    Deg(v)
}
/// Constructs a [Radians]
pub const fn radians(v: f64) -> Radians {
    Rad(v)
}

/// Conversion implementations
impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        Deg(rad.0.to_degrees())
    }
}

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        Rad(deg.0.to_radians())
    }
}

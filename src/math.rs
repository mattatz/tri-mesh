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

/// Trait to emulate cgmath's prelude functionality
pub trait InnerSpace: Sized {
    type Scalar;
    fn dot(&self, other: &Self) -> Self::Scalar;
    fn magnitude(&self) -> Self::Scalar;
    fn magnitude2(&self) -> Self::Scalar;
    fn normalize(&self) -> Self;
}

impl InnerSpace for Vector3<f64> {
    type Scalar = f64;

    fn dot(&self, other: &Self) -> Self::Scalar {
        na::Vector3::dot(self, other)
    }

    fn magnitude(&self) -> Self::Scalar {
        self.norm()
    }

    fn magnitude2(&self) -> Self::Scalar {
        self.norm_squared()
    }

    fn normalize(&self) -> Self {
        self.normalize()
    }
}

impl InnerSpace for Vector4<f64> {
    type Scalar = f64;

    fn dot(&self, other: &Self) -> Self::Scalar {
        na::Vector4::dot(self, other)
    }

    fn magnitude(&self) -> Self::Scalar {
        self.norm()
    }

    fn magnitude2(&self) -> Self::Scalar {
        self.norm_squared()
    }

    fn normalize(&self) -> Self {
        self.normalize()
    }
}

/// Trait for vector space operations
pub trait VectorSpace: Sized {
    type Scalar;
}

impl VectorSpace for Vector3<f64> {
    type Scalar = f64;
}

impl VectorSpace for Vector4<f64> {
    type Scalar = f64;
}

/// Trait for element-wise operations
pub trait ElementWise {
    fn add_element_wise(&self, scalar: f64) -> Self;
    fn sub_element_wise(&self, scalar: f64) -> Self;
    fn mul_element_wise(&self, scalar: f64) -> Self;
    fn div_element_wise(&self, scalar: f64) -> Self;
}

impl ElementWise for Vector3<f64> {
    fn add_element_wise(&self, scalar: f64) -> Self {
        self.add_scalar(scalar)
    }

    fn sub_element_wise(&self, scalar: f64) -> Self {
        self.add_scalar(-scalar)
    }

    fn mul_element_wise(&self, scalar: f64) -> Self {
        self * scalar
    }

    fn div_element_wise(&self, scalar: f64) -> Self {
        self / scalar
    }
}

impl ElementWise for Vector4<f64> {
    fn add_element_wise(&self, scalar: f64) -> Self {
        self.add_scalar(scalar)
    }

    fn sub_element_wise(&self, scalar: f64) -> Self {
        self.add_scalar(-scalar)
    }

    fn mul_element_wise(&self, scalar: f64) -> Self {
        self * scalar
    }

    fn div_element_wise(&self, scalar: f64) -> Self {
        self / scalar
    }
}

/// Matrix trait for square matrix operations
pub trait SquareMatrix {
    fn from_scale(scale: f64) -> Self;
    fn from_nonuniform_scale(x: f64, y: f64, z: f64) -> Self;
}

impl SquareMatrix for Matrix4<f64> {
    fn from_scale(scale: f64) -> Self {
        Matrix4::new_scaling(scale)
    }

    fn from_nonuniform_scale(x: f64, y: f64, z: f64) -> Self {
        Matrix4::new_nonuniform_scaling(&Vector3::new(x, y, z))
    }
}

/// Matrix transformation traits
pub trait Transform {
    fn from_translation(v: Vec3) -> Self;
}

impl Transform for Matrix4<f64> {
    fn from_translation(v: Vec3) -> Self {
        Matrix4::new_translation(&v)
    }
}

/// Rotation traits
pub trait Rotation3 {
    fn from_axis_angle(axis: Vec3, angle: Radians) -> Self;
}

impl Rotation3 for Matrix4<f64> {
    fn from_axis_angle(axis: Vec3, angle: Radians) -> Self {
        let rotation = na::Rotation3::from_axis_angle(&na::Unit::new_normalize(axis), angle.0);
        rotation.to_homogeneous()
    }
}

impl Rotation3 for Matrix3<f64> {
    fn from_axis_angle(axis: Vec3, angle: Radians) -> Self {
        na::Rotation3::from_axis_angle(&na::Unit::new_normalize(axis), angle.0).into_inner()
    }
}

/// Re-export commonly used traits
pub mod prelude {
    pub use super::{ElementWise, InnerSpace, Rotation3, SquareMatrix, Transform, VectorSpace};
}

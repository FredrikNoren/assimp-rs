//! The `math` module contains definitions of primitive math types.
//!
//! Not really anything useful here. Conversion traits are implemented on each type to convert
//! into/from the much more useful `cgmath` types.
//! e.g. `Matrix3x3` converts to/from `cgmath::Matrix3<f32>`.

pub use self::color3::Color3D;
pub use self::color4::Color4D;
pub use self::matrix3::Matrix3x3;
pub use self::matrix4::Matrix4x4;
pub use self::quaternion::Quaternion;
pub use self::vector2::Vector2D;
pub use self::vector3::Vector3D;

pub mod color3;
pub mod color4;
pub mod matrix3;
pub mod matrix4;
pub mod quaternion;
pub mod vector2;
pub mod vector3;

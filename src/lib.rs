extern crate assimp_sys as sys;
extern crate cgmath;
extern crate libc;

pub use color3::*;
pub use color4::*;
pub use matrix3::*;
pub use matrix4::*;
pub use quaternion::*;
pub use vector2::*;
pub use vector3::*;

mod color3;
mod color4;
mod matrix3;
mod matrix4;
mod quaternion;
mod vector2;
mod vector3;

pub mod ffi {
    pub use sys::*;
}

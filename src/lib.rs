//! # assimp - Open Asset Import Library
//!
//! Bindings for the [Assimp](http://assimp.sourceforge.net) library.

extern crate assimp_sys as ffi;
extern crate cgmath;
extern crate libc;

pub use import::Importer;
pub use log::LogStream;
pub use math::{Color3D, Color4D, Matrix3x3, Matrix4x4, Quaternion, Vector2D, Vector3D};
pub use scene::{Animation, NodeAnim, VectorKey, QuatKey, Camera, Face, Light, Material, Mesh, Node, Scene, Texture};

#[macro_use]
mod internal_macros;

pub mod export;
pub mod import;
pub mod log;
pub mod math;
pub mod scene;

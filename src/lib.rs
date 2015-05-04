//! # assimp - Open Asset Import Library
//!
//! Bindings for the [Assimp](http://assimp.sourceforge.net) library.

extern crate assimp_sys as ffi;
extern crate cgmath;
extern crate libc;

pub use import::Importer;
pub use log::LogStream;
pub use scene::{Scene, SceneConst, SceneMut};

pub mod export;
pub mod import;
mod log;
pub mod math;
pub mod scene;

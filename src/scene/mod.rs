//! The `scene` module contains definitions of imported scene data.

pub use self::animation::*;
pub use self::camera::*;
pub use self::face::*;
pub use self::light::*;
pub use self::material::*;
pub use self::mesh::*;
pub use self::node::*;
pub use self::scene::Scene;
pub use self::texture::*;

mod animation;
mod camera;
mod face;
mod light;
mod material;
mod mesh;
mod node;
mod scene;
mod texture;

//! The `scene` module contains definitions of imported scene data.

pub use self::animation::{Animation, AnimationIter};
pub use self::camera::{Camera, CameraIter};
pub use self::face::{Face, FaceIter};
pub use self::light::{Light, LightIter};
pub use self::material::{Material, MaterialIter};
pub use self::mesh::{Mesh, MeshIter};
pub use self::node::{Node, NodeIter};
pub use self::scene::Scene;
pub use self::texture::{Texture, TextureIter};

mod animation;
mod camera;
mod face;
mod light;
mod material;
mod mesh;
mod node;
mod scene;
mod texture;

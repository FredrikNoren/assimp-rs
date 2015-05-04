use std::ptr;
use std::slice::from_raw_parts;

use ffi::*;
use libc::c_uint;

pub use self::animation::Animation;
pub use self::camera::Camera;
pub use self::face::Face;
pub use self::light::Light;
pub use self::material::Material;
pub use self::mesh::Mesh;
pub use self::texture::Texture;

use self::animation::AnimationInternal;
use self::face::FaceInternal;
use self::light::LightInternal;
use self::camera::CameraInternal;
use self::material::MaterialInternal;
use self::mesh::MeshInternal;
use self::texture::TextureInternal;

mod animation;
mod camera;
mod face;
mod light;
mod material;
mod mesh;
mod texture;

pub struct SceneConst(*const AiScene);
pub struct SceneMut(*mut AiScene);

pub trait Scene {
    fn inner(&self) -> *const AiScene;

    fn num_meshes(&self) -> c_uint {
        unsafe { (*self.inner()).num_meshes }
    }

    fn meshes(&self) -> Vec<Mesh> {
        unsafe {
            let ref raw = &*self.inner();
            let len = raw.num_meshes as usize;
            from_raw_parts(raw.meshes, len).iter().map(|x| Mesh::new(*x)).collect()
        }
    }

    fn num_materials(&self) -> c_uint {
        unsafe { (*self.inner()).num_materials }
    }

    fn materials(&self) -> Vec<Material> {
        unsafe {
            let ref raw = &*self.inner();
            let len = raw.num_materials as usize;
            from_raw_parts(raw.materials, len).iter().map(|x| Material::new(*x)).collect()
        }
    }

    fn num_animations(&self) -> c_uint {
        unsafe { (*self.inner()).num_animations }
    }

    fn animations(&self) -> Vec<Animation> {
        unsafe {
            let ref raw = &*self.inner();
            let len = raw.num_animations as usize;
            from_raw_parts(raw.animations, len).iter().map(|x| Animation::new(*x)).collect()
        }
    }

    fn num_textures(&self) -> c_uint {
        unsafe { (*self.inner()).num_textures }
    }

    fn textures(&self) -> Vec<Texture> {
        unsafe {
            let ref raw = &*self.inner();
            let len = raw.num_textures as usize;
            from_raw_parts(raw.textures, len).iter().map(|x| Texture::new(*x)).collect()
        }
    }

    fn num_lights(&self) -> c_uint {
        unsafe { (*self.inner()).num_lights }
    }

    fn lights(&self) -> Vec<Light> {
        unsafe {
            let ref raw = &*self.inner();
            let len = raw.num_lights as usize;
            from_raw_parts(raw.lights, len).iter().map(|x| Light::new(*x)).collect()
        }
    }

    fn num_cameras(&self) -> c_uint {
        unsafe { (*self.inner()).num_cameras }
    }

    fn cameras(&self) -> Vec<Camera> {
        unsafe {
            let ref raw = &*self.inner();
            let len = raw.num_cameras as usize;
            from_raw_parts(raw.cameras, len).iter().map(|x| Camera::new(*x)).collect()
        }
    }
}

impl Scene for SceneConst {
    fn inner(&self) -> *const AiScene { self.0 }
}

impl Scene for SceneMut {
    fn inner(&self) -> *const AiScene { self.0 }
}

impl From<SceneConst> for SceneMut {
    fn from(scene: SceneConst) -> SceneMut {
        let mut new_scene = ptr::null_mut();
        unsafe { aiCopyScene(scene.0, &mut new_scene) };
        SceneMut(new_scene)
    }
}

impl Drop for SceneConst {
    fn drop(&mut self) {
        unsafe { aiReleaseImport(self.0); }
    }
}

impl Drop for SceneMut {
    fn drop(&mut self) {
        unsafe { aiFreeScene(self.0); }
    }
}

#[doc(hidden)]
pub trait SceneInternal: Scene {
    fn new(raw_scene: *const AiScene) -> SceneConst {
        SceneConst(raw_scene)
    }
    fn get_raw_ptr(&self) -> *const AiScene {
        self.inner()
    }
}

impl SceneInternal for SceneConst {}

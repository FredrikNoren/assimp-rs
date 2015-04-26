use libc::{c_uint, c_void};

use anim::*;
use camera::*;
use light::*;
use material::*;
use mesh::*;
use metadata::*;
use texture::*;
use types::*;

#[repr(C)]
pub struct AiNode {
    pub name: AiString,
    pub transformation: AiMatrix4x4,
    pub parent: *mut AiNode,
    pub num_children: c_uint,
    pub children: *mut *mut AiNode,
    pub num_meshes: c_uint,
    pub meshes: *mut c_uint,
    pub metadata: *mut AiMetadata
}

bitflags! {
    #[repr(C)]
    flags AiSceneFlags : c_uint {
        const AI_SCENE_FLAGS_INCOMPLETE = 0x1,
        const AI_SCENE_FLAGS_VALIDATED = 0x2,
        const AI_SCENE_FLAGS_VALIDATION_WARNING = 0x4,
        const AI_SCENE_FLAGS_NON_VERBOSE_FORMAT = 0x8,
        const AI_SCENE_FLAGS_TERRAIN = 0x10
    }
}

#[repr(C)]
pub struct AiScene {
    pub flags: AiSceneFlags,
    pub root_node: *mut AiNode,
    pub num_meshes: c_uint,
    pub meshes: *mut *mut AiMesh,
    pub num_materials: c_uint,
    pub materials: *mut *mut AiMaterial,
    pub num_animations: c_uint,
    pub animations: *mut *mut AiAnimation,
    pub num_textures: c_uint,
    pub textures: *mut *mut AiTexture,
    pub num_lights: c_uint,
    pub lights: *mut *mut AiLight,
    pub num_cameras: c_uint,
    pub cameras: *mut *mut AiCamera,
    private: *const c_void
}

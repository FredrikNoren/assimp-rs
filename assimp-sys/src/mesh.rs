use libc::{c_float, c_uint};

use types::*;

pub const AI_MAX_FACE_INDICES: usize = 0x7fff;
pub const AI_MAX_BONE_WEIGHTS: usize = 0x7fffffff;
pub const AI_MAX_VERTICES: usize = 0x7fffffff;
pub const AI_MAX_FACES: usize = 0x7fffffff;
pub const AI_MAX_NUMBER_OF_COLOR_SETS: usize = 0x8;
pub const AI_MAX_NUMBER_OF_TEXTURECOORDS: usize = 0x8;

#[repr(C)]
pub struct AiFace {
    pub num_indices: c_uint,
    pub indices: *mut c_uint
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AiVertexWeight {
    pub vertex_id: c_uint,
    pub weight: c_float
}

#[repr(C)]
pub struct AiBone {
    pub name: AiString,
    pub num_weights: c_uint,
    pub weights: *mut AiVertexWeight,
    pub offset_matrix: AiMatrix4x4
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiPrimitiveType {
    Point    = 0x1,
    Line     = 0x2,
    Triangle = 0x4,
    Polygon  = 0x8
}

#[repr(C)]
pub struct AiAnimMesh {
    pub vertices: *mut AiVector3D,
    pub normals: *mut AiVector3D,
    pub tangents: *mut AiVector3D,
    pub bitangents: *mut AiVector3D,
    pub colors: [*mut AiColor4D; AI_MAX_NUMBER_OF_COLOR_SETS],
    pub texture_coords: [*mut AiVector3D; AI_MAX_NUMBER_OF_TEXTURECOORDS],
    pub num_vertices: c_uint
}

#[repr(C)]
pub struct AiMesh {
    pub primitive_types: c_uint,
    pub num_vertices: c_uint,
    pub num_faces: c_uint,
    pub vertices: *mut AiVector3D,
    pub normals: *mut AiVector3D,
    pub tangents: *mut AiVector3D,
    pub bitangents: *mut AiVector3D,
    pub colors: [*mut AiColor4D; AI_MAX_NUMBER_OF_COLOR_SETS],
    pub texture_coords: [*mut AiVector3D; AI_MAX_NUMBER_OF_TEXTURECOORDS],
    pub num_uv_components: [c_uint; AI_MAX_NUMBER_OF_TEXTURECOORDS],
    pub faces: *mut AiFace,
    pub num_bones: c_uint,
    pub bones: *mut *mut AiBone,
    pub material_index: c_uint,
    pub name: AiString,
    pub num_anim_meshes: c_uint,
    pub anim_meshes: *mut *mut AiAnimMesh
}

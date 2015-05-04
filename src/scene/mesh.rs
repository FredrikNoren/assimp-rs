use std::slice::from_raw_parts;

use ffi::AiMesh;
use libc::c_uint;

use math::vector3::*;
use super::face::*;

// TODO mutable mesh type
pub struct Mesh(*const AiMesh);

impl Mesh {
    #[inline]
    fn inner(&self) -> &AiMesh {
        unsafe { &*self.0 }
    }

    // TODO return as PrimitiveType enum
    pub fn primitive_types(&self) -> u32 {
        self.inner().primitive_types
    }

    pub fn num_vertices(&self) -> c_uint {
        self.inner().num_vertices
    }

    pub fn num_faces(&self) -> c_uint {
        self.inner().num_faces
    }

    pub fn vertices(&self) -> Vec<Vector3D> {
        let raw = self.inner();
        let len = raw.num_vertices as usize;
        unsafe {
            from_raw_parts(raw.vertices, len).iter().map(|x| Vector3D::from_raw(x)).collect()
        }
    }

    pub fn normals(&self) -> Vec<Vector3D> {
        let raw = self.inner();
        let len = raw.num_vertices as usize;
        unsafe {
            from_raw_parts(raw.normals, len).iter().map(|x| Vector3D::from_raw(x)).collect()
        }
    }

    pub fn tangents(&self) -> Vec<Vector3D> {
        let raw = self.inner();
        let len = raw.num_vertices as usize;
        unsafe {
            from_raw_parts(raw.tangents, len).iter().map(|x| Vector3D::from_raw(x)).collect()
        }
    }

    pub fn bitangents(&self) -> Vec<Vector3D> {
        let raw = self.inner();
        let len = raw.num_vertices as usize;
        unsafe {
            from_raw_parts(raw.bitangents, len).iter().map(|x| Vector3D::from_raw(x)).collect()
        }
    }

    pub fn faces(&self) -> Vec<Face> {
        let raw = self.inner();
        let len = raw.num_faces as usize;
        unsafe {
            from_raw_parts(raw.faces, len).iter().map(|x| Face::new(x)).collect()
        }
    }
}

#[doc(hidden)]
pub trait MeshInternal {
    fn new(raw_mesh: *const AiMesh) -> Mesh {
        Mesh(raw_mesh)
    }
}

impl MeshInternal for Mesh {}

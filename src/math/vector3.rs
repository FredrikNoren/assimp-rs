use std::ops::Deref;

use cgmath::{Point3, Vector3};
use ffi::AiVector3D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3D(pub AiVector3D);

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D(AiVector3D {
            x: x,
            y: y,
            z: z
        })
    }
}

impl Into<[f32; 3]> for Vector3D {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<Point3<f32>> for Vector3D {
    fn from(p: Point3<f32>) -> Vector3D {
        Vector3D::new(p[0], p[1], p[2])
    }
}

impl Into<Point3<f32>> for Vector3D {
    fn into(self) -> Point3<f32> {
        Point3::new(self.x, self.y, self.z)
    }
}

impl From<Vector3<f32>> for Vector3D {
    fn from(p: Vector3<f32>) -> Vector3D {
        Vector3D::new(p[0], p[1], p[2])
    }
}

impl Into<Vector3<f32>> for Vector3D {
    fn into(self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl Deref for Vector3D {
    type Target = AiVector3D;

    fn deref<'a>(&'a self) -> &'a AiVector3D {
        &self.0
    }
}

pub trait Vector3DInternal {
    fn from_raw(raw_vector: *const AiVector3D) -> Vector3D {
        unsafe { Vector3D(*raw_vector) }
    }
}

impl Vector3DInternal for Vector3D {}

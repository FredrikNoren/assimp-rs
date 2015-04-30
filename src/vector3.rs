use std::ops::Deref;

use cgmath::{Point3, Vector3};
use ffi::AiVector3D;
use libc::c_float;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3D(AiVector3D);

impl Vector3D {
    pub fn new(x: c_float, y: c_float, z: c_float) -> Vector3D {
        Vector3D(AiVector3D {
            x: x,
            y: y,
            z: z
        })
    }
}

impl From<Point3<c_float>> for Vector3D {
    fn from(p: Point3<c_float>) -> Vector3D {
        Vector3D::new(p[0], p[1], p[2])
    }
}

impl Into<Point3<c_float>> for Vector3D {
    fn into(self) -> Point3<c_float> {
        Point3::new(self.0.x, self.0.y, self.0.z)
    }
}

impl From<Vector3<c_float>> for Vector3D {
    fn from(p: Vector3<c_float>) -> Vector3D {
        Vector3D::new(p[0], p[1], p[2])
    }
}

impl Into<Vector3<c_float>> for Vector3D {
    fn into(self) -> Vector3<c_float> {
        Vector3::new(self.0.x, self.0.y, self.0.z)
    }
}

impl Deref for Vector3D {
    type Target = AiVector3D;

    fn deref<'a>(&'a self) -> &'a AiVector3D {
        &self.0
    }
}

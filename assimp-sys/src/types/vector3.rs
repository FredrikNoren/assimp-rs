use cgmath::{Point3, Vector3};
use libc::c_float;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiVector3D {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float
}

impl AiVector3D {
    pub fn to_cgmath_vector(&self) -> Vector3<c_float> {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn from_cgmath_vector(vector: &Vector3<c_float>) -> AiVector3D {
        AiVector3D {
            x: vector.x,
            y: vector.y,
            z: vector.z
        }
    }

    pub fn to_cgmath_point(&self) -> Point3<c_float> {
        Point3::new(self.x, self.y, self.z)
    }

    pub fn from_cgmath_point(point: &Point3<c_float>) -> AiVector3D {
        AiVector3D {
            x: point.x,
            y: point.y,
            z: point.z
        }
    }
}

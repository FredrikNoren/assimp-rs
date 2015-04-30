use std::ops::Deref;

use cgmath::Vector3;
use ffi::AiColor3D;
use libc::c_float;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color3D(AiColor3D);

impl Color3D {
    pub fn new(r: c_float, g: c_float, b: c_float) -> Color3D {
        Color3D(AiColor3D {
            r: r,
            g: g,
            b: b
        })
    }
}

impl From<Vector3<c_float>> for Color3D {
    fn from(p: Vector3<c_float>) -> Color3D {
        Color3D::new(p[0], p[1], p[2])
    }
}

impl Into<Vector3<c_float>> for Color3D {
    fn into(self) -> Vector3<c_float> {
        Vector3::new(self.0.r, self.0.g, self.0.b)
    }
}

impl Deref for Color3D {
    type Target = AiColor3D;

    fn deref<'a>(&'a self) -> &'a AiColor3D {
        &self.0
    }
}

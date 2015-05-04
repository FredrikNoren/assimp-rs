use std::ops::Deref;

use cgmath::Vector3;
use ffi::AiColor3D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color3D(pub AiColor3D);

impl Color3D {
    pub fn new(r: f32, g: f32, b: f32) -> Color3D {
        Color3D(AiColor3D {
            r: r,
            g: g,
            b: b
        })
    }
}

impl From<Vector3<f32>> for Color3D {
    fn from(p: Vector3<f32>) -> Color3D {
        Color3D::new(p[0], p[1], p[2])
    }
}

impl Into<Vector3<f32>> for Color3D {
    fn into(self) -> Vector3<f32> {
        Vector3::new(self.r, self.g, self.b)
    }
}

impl Deref for Color3D {
    type Target = AiColor3D;

    fn deref<'a>(&'a self) -> &'a AiColor3D {
        &self.0
    }
}

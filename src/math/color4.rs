use std::ops::Deref;

use cgmath::Vector4;
use ffi::AiColor4D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color4D(pub AiColor4D);

impl Color4D {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color4D {
        Color4D(AiColor4D {
            r: r,
            g: g,
            b: b,
            a: a
        })
    }
}

impl From<Vector4<f32>> for Color4D {
    fn from(p: Vector4<f32>) -> Color4D {
        Color4D::new(p[0], p[1], p[2], p[3])
    }
}

impl Into<Vector4<f32>> for Color4D {
    fn into(self) -> Vector4<f32> {
        Vector4::new(self.r, self.g, self.b, self.a)
    }
}

impl Deref for Color4D {
    type Target = AiColor4D;

    fn deref<'a>(&'a self) -> &'a AiColor4D {
        &self.0
    }
}

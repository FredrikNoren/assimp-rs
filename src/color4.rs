use cgmath::Vector4;
use ffi::AiColor4D;
use libc::c_float;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color4D(AiColor4D);

impl Color4D {
    pub fn new(r: c_float, g: c_float, b: c_float, a: c_float) -> Color4D {
        Color4D(AiColor4D {
            r: r,
            g: g,
            b: b,
            a: a
        })
    }
}

impl From<Vector4<c_float>> for Color4D {
    fn from(p: Vector4<c_float>) -> Color4D {
        Color4D::new(p[0], p[1], p[2], p[3])
    }
}

impl Into<Vector4<c_float>> for Color4D {
    fn into(self) -> Vector4<c_float> {
        Vector4::new(self.0.r, self.0.g, self.0.b, self.0.a)
    }
}

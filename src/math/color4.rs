use cgmath::Vector4;
use ffi::AiColor4D;

define_type! {
    /// Color4D docs
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Color4D(AiColor4D)
}

impl Color4D {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color4D {
        Color4D(AiColor4D { r: r, g: g, b: b, a: a })
    }
}

impl From<[f32; 4]> for Color4D {
    fn from(v: [f32; 4]) -> Color4D {
        Color4D::new(v[0], v[1], v[2], v[3])
    }
}

impl Into<[f32; 4]> for Color4D {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
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

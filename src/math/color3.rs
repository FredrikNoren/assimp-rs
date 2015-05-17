use cgmath::Vector3;
use ffi::AiColor3D;

define_type! {
    /// Color3D docs
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Color3D(AiColor3D)
}

impl Color3D {
    pub fn new(r: f32, g: f32, b: f32) -> Color3D {
        Color3D(AiColor3D { r: r, g: g, b: b })
    }
}

impl From<[f32; 3]> for Color3D {
    fn from(v: [f32; 3]) -> Color3D {
        Color3D::new(v[0], v[1], v[2])
    }
}

impl Into<[f32; 3]> for Color3D {
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
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

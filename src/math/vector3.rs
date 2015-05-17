use cgmath::{Point3, Vector3};
use ffi::AiVector3D;

define_type_and_iterator! {
    /// Vector3D docs
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Vector3D(AiVector3D)
    /// Vector3DIter docs
    struct Vector3DIter
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D(AiVector3D { x: x, y: y, z: z })
    }
}

impl From<[f32; 3]> for Vector3D {
    fn from(v: [f32; 3]) -> Vector3D {
        Vector3D::new(v[0], v[1], v[2])
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

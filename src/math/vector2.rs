use cgmath::{Point2, Vector2};
use ffi::AiVector2D;

define_type! {
    /// Vector2D docs
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Vector2D(AiVector2D)
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D(AiVector2D { x: x, y: y })
    }
}

impl From<[f32; 2]> for Vector2D {
    fn from(v: [f32; 2]) -> Vector2D {
        Vector2D::new(v[0], v[1])
    }
}

impl Into<[f32; 2]> for Vector2D {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl From<Point2<f32>> for Vector2D {
    fn from(p: Point2<f32>) -> Vector2D {
        Vector2D::new(p[0], p[1])
    }
}

impl Into<Point2<f32>> for Vector2D {
    fn into(self) -> Point2<f32> {
        Point2::new(self.x, self.y)
    }
}

impl From<Vector2<f32>> for Vector2D {
    fn from(p: Vector2<f32>) -> Vector2D {
        Vector2D::new(p[0], p[1])
    }
}

impl Into<Vector2<f32>> for Vector2D {
    fn into(self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }
}

use std::ops::Deref;

use cgmath::{Point2, Vector2};
use ffi::AiVector2D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2D(pub AiVector2D);

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D(AiVector2D {
            x: x,
            y: y
        })
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

impl Deref for Vector2D {
    type Target = AiVector2D;

    fn deref<'a>(&'a self) -> &'a AiVector2D {
        &self.0
    }
}

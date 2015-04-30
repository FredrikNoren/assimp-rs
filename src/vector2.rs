use std::ops::Deref;

use cgmath::{Point2, Vector2};
use ffi::AiVector2D;
use libc::c_float;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2D(AiVector2D);

impl Vector2D {
    pub fn new(x: c_float, y: c_float) -> Vector2D {
        Vector2D(AiVector2D {
            x: x,
            y: y
        })
    }
}

impl From<Point2<c_float>> for Vector2D {
    fn from(p: Point2<c_float>) -> Vector2D {
        Vector2D::new(p[0], p[1])
    }
}

impl Into<Point2<c_float>> for Vector2D {
    fn into(self) -> Point2<c_float> {
        Point2::new(self.0.x, self.0.y)
    }
}

impl From<Vector2<c_float>> for Vector2D {
    fn from(p: Vector2<c_float>) -> Vector2D {
        Vector2D::new(p[0], p[1])
    }
}

impl Into<Vector2<c_float>> for Vector2D {
    fn into(self) -> Vector2<c_float> {
        Vector2::new(self.0.x, self.0.y)
    }
}

impl Deref for Vector2D {
    type Target = AiVector2D;

    fn deref<'a>(&'a self) -> &'a AiVector2D {
        &self.0
    }
}

use cgmath::{Point2, Vector2};
use libc::c_float;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiVector2D {
    pub x: c_float,
    pub y: c_float
}

impl AiVector2D {
    pub fn to_cgmath_vector(&self) -> Vector2<c_float> {
        Vector2::new(self.x, self.y)
    }

    pub fn from_cgmath_vector(vector: &Vector2<c_float>) -> AiVector2D {
        AiVector2D {
            x: vector.x,
            y: vector.y
        }
    }

    pub fn to_cgmath_point(&self) -> Point2<c_float> {
        Point2::new(self.x, self.y)
    }

    pub fn from_cgmath_point(point: &Point2<c_float>) -> AiVector2D {
        AiVector2D {
            x: point.x,
            y: point.y
        }
    }
}

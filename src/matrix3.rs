use std::ops::Deref;

use cgmath::Matrix3;
use ffi::AiMatrix3x3;
use libc::c_float;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3x3(AiMatrix3x3);

impl Matrix3x3 {
    pub fn new(c0r0: c_float, c0r1: c_float, c0r2: c_float,
               c1r0: c_float, c1r1: c_float, c1r2: c_float,
               c2r0: c_float, c2r1: c_float, c2r2: c_float) -> Matrix3x3 {
        Matrix3x3(AiMatrix3x3 {
            a1: c0r0, a2: c0r1, a3: c0r2,
            b1: c1r0, b2: c1r1, b3: c1r2,
            c1: c2r0, c2: c2r1, c3: c2r2,
        })
    }
}

impl From<Matrix3<c_float>> for Matrix3x3 {
    fn from(mat: Matrix3<c_float>) -> Matrix3x3 {
        Matrix3x3::new(mat[0][0], mat[1][0], mat[2][0],
                       mat[0][1], mat[1][1], mat[2][1],
                       mat[0][2], mat[1][2], mat[2][2])
    }
}

impl Into<Matrix3<c_float>> for Matrix3x3 {
    fn into(self) -> Matrix3<c_float> {
        Matrix3::new(self.0.a1, self.0.b1, self.0.c1,
                     self.0.a2, self.0.b2, self.0.c2,
                     self.0.a3, self.0.b3, self.0.c3)
    }
}

impl Deref for Matrix3x3 {
    type Target = AiMatrix3x3;

    fn deref<'a>(&'a self) -> &'a AiMatrix3x3 {
        &self.0
    }
}

use std::ops::Deref;

use cgmath::Matrix4;
use ffi::AiMatrix4x4;
use libc::c_float;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4x4(AiMatrix4x4);

impl Matrix4x4 {
    pub fn new(c0r0: c_float, c0r1: c_float, c0r2: c_float, c0r3: c_float,
               c1r0: c_float, c1r1: c_float, c1r2: c_float, c1r3: c_float,
               c2r0: c_float, c2r1: c_float, c2r2: c_float, c2r3: c_float,
               c3r0: c_float, c3r1: c_float, c3r2: c_float, c3r3: c_float) -> Matrix4x4 {
        Matrix4x4(AiMatrix4x4 {
            a1: c0r0, a2: c0r1, a3: c0r2, a4: c0r3,
            b1: c1r0, b2: c1r1, b3: c1r2, b4: c1r3,
            c1: c2r0, c2: c2r1, c3: c2r2, c4: c2r3,
            d1: c3r0, d2: c3r1, d3: c3r2, d4: c3r3,
        })
    }
}

impl From<Matrix4<c_float>> for Matrix4x4 {
    fn from(mat: Matrix4<c_float>) -> Matrix4x4 {
        Matrix4x4::new(mat[0][0], mat[1][0], mat[2][0], mat[3][0],
                       mat[0][1], mat[1][1], mat[2][1], mat[3][1],
                       mat[0][2], mat[1][2], mat[2][2], mat[3][2],
                       mat[0][3], mat[1][3], mat[2][3], mat[3][3])
    }
}

impl Into<Matrix4<c_float>> for Matrix4x4 {
    fn into(self) -> Matrix4<c_float> {
        Matrix4::new(self.0.a1, self.0.b1, self.0.c1, self.0.d1,
                     self.0.a2, self.0.b2, self.0.c2, self.0.d2,
                     self.0.a3, self.0.b3, self.0.c3, self.0.d3,
                     self.0.a4, self.0.b4, self.0.c4, self.0.d4)
    }
}

impl Deref for Matrix4x4 {
    type Target = AiMatrix4x4;

    fn deref<'a>(&'a self) -> &'a AiMatrix4x4 {
        &self.0
    }
}

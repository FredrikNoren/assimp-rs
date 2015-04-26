use cgmath::Matrix4;
use libc::c_float;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiMatrix4x4 {
    pub a1: c_float,
    pub a2: c_float,
    pub a3: c_float,
    pub a4: c_float,
    pub b1: c_float,
    pub b2: c_float,
    pub b3: c_float,
    pub b4: c_float,
    pub c1: c_float,
    pub c2: c_float,
    pub c3: c_float,
    pub c4: c_float,
    pub d1: c_float,
    pub d2: c_float,
    pub d3: c_float,
    pub d4: c_float
}

impl AiMatrix4x4 {
    pub fn identity() -> AiMatrix4x4 {
        AiMatrix4x4::from_cgmath_matrix(&Matrix4::identity())
    }

    pub fn to_cgmath_matrix(&self) -> Matrix4<c_float> {
        Matrix4::new(self.a1, self.b1, self.c1, self.d1,
                     self.a2, self.b2, self.c2, self.d2,
                     self.a3, self.b3, self.c3, self.d3,
                     self.a4, self.b4, self.c4, self.d4)
    }

    pub fn from_cgmath_matrix(mat: &Matrix4<c_float>) -> AiMatrix4x4 {
        AiMatrix4x4 {
            a1: mat[0][0], a2: mat[1][0], a3: mat[2][0], a4: mat[3][0],
            b1: mat[0][1], b2: mat[1][1], b3: mat[2][1], b4: mat[3][1],
            c1: mat[0][2], c2: mat[1][2], c3: mat[2][2], c4: mat[3][2],
            d1: mat[0][3], d2: mat[1][3], d3: mat[2][3], d4: mat[3][3]
        }
    }
}

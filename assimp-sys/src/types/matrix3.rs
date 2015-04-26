use cgmath::Matrix3;
use libc::c_float;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiMatrix3x3 {
    pub a1: c_float,
    pub a2: c_float,
    pub a3: c_float,
    pub b1: c_float,
    pub b2: c_float,
    pub b3: c_float,
    pub c1: c_float,
    pub c2: c_float,
    pub c3: c_float
}

impl AiMatrix3x3 {
    pub fn identity() -> AiMatrix3x3 {
        AiMatrix3x3::from_cgmath_matrix(&Matrix3::identity())
    }

    pub fn to_cgmath_matrix(&self) -> Matrix3<c_float> {
        Matrix3::new(self.a1, self.b1, self.c1,
                     self.a2, self.b2, self.c2,
                     self.a3, self.b3, self.c3)
    }

    pub fn from_cgmath_matrix(mat: &Matrix3<c_float>) -> AiMatrix3x3 {
        AiMatrix3x3 {
            a1: mat[0][0], a2: mat[1][0], a3: mat[2][0],
            b1: mat[0][1], b2: mat[1][1], b3: mat[2][1],
            c1: mat[0][2], c2: mat[1][2], c3: mat[2][2]
        }
    }
}

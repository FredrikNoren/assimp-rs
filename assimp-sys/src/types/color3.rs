use cgmath::Vector3;
use libc::c_float;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiColor3D {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float
}

impl AiColor3D {
    pub fn to_cgmath_vector(&self) -> Vector3<c_float> {
        Vector3::new(self.r, self.g, self.b)
    }

    pub fn from_cgmath_vector(vector: &Vector3<c_float>) -> AiColor3D {
        AiColor3D {
            r: vector.x,
            g: vector.y,
            b: vector.z
        }
    }
}

use cgmath::Quaternion;
use libc::c_float;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiQuaternion {
    pub w: c_float,
    pub x: c_float,
    pub y: c_float,
    pub z: c_float
}

impl AiQuaternion {
    pub fn to_cgmath_quaternion(&self) -> Quaternion<c_float> {
        Quaternion::new(self.w, self.x, self.y, self.z)
    }

    pub fn from_cgmath_quaternion(quat: &Quaternion<c_float>) -> AiQuaternion {
        AiQuaternion {
            w: quat[0],
            x: quat[1],
            y: quat[2],
            z: quat[3]
        }
    }
}

use cgmath::Quaternion as CgQuaternion;
use ffi::AiQuaternion;
use libc::c_float;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion(AiQuaternion);

impl Quaternion {
    pub fn new(w: c_float, x: c_float, y: c_float ,z: c_float) -> Quaternion {
        Quaternion(AiQuaternion {
            w: w,
            x: x,
            y: y,
            z: z
        })
    }
}

impl From<CgQuaternion<c_float>> for Quaternion {
    fn from(q: CgQuaternion<c_float>) -> Quaternion {
        Quaternion::new(q[0], q[1], q[2], q[3])
    }
}

impl Into<CgQuaternion<c_float>> for Quaternion {
    fn into(self) -> CgQuaternion<c_float> {
        CgQuaternion::new(self.0.w, self.0.x, self.0.y, self.0.z)
    }
}

use std::ops::Deref;

use cgmath::Quaternion as CgQuaternion;
use ffi::AiQuaternion;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion(pub AiQuaternion);

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32 ,z: f32) -> Quaternion {
        Quaternion(AiQuaternion {
            w: w,
            x: x,
            y: y,
            z: z
        })
    }
}

impl From<CgQuaternion<f32>> for Quaternion {
    fn from(q: CgQuaternion<f32>) -> Quaternion {
        Quaternion::new(q[0], q[1], q[2], q[3])
    }
}

impl Into<CgQuaternion<f32>> for Quaternion {
    fn into(self) -> CgQuaternion<f32> {
        CgQuaternion::new(self.0.w, self.0.x, self.0.y, self.0.z)
    }
}

impl Deref for Quaternion {
    type Target = AiQuaternion;

    fn deref<'a>(&'a self) -> &'a AiQuaternion {
        &self.0
    }
}

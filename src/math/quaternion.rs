use cgmath::Quaternion as CgQuaternion;
use ffi::AiQuaternion;

define_type! {
    /// Quaternion docs
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Quaternion(AiQuaternion)
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32 ,z: f32) -> Quaternion {
        Quaternion(AiQuaternion { w: w, x: x, y: y, z: z })
    }
}

impl From<CgQuaternion<f32>> for Quaternion {
    fn from(q: CgQuaternion<f32>) -> Quaternion {
        Quaternion::new(q[0], q[1], q[2], q[3])
    }
}

impl Into<CgQuaternion<f32>> for Quaternion {
    fn into(self) -> CgQuaternion<f32> {
        CgQuaternion::new(self.w, self.x, self.y, self.z)
    }
}

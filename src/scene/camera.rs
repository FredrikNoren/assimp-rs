use std::ops::Deref;

use ffi::AiCamera;

// TODO mutable camera type
pub struct Camera(*const AiCamera);

#[doc(hidden)]
pub trait CameraInternal {
    fn new(raw_camera: *const AiCamera) -> Camera {
        Camera(raw_camera)
    }
}

impl CameraInternal for Camera {}

// TODO remove deref when types are implemented
impl Deref for Camera {
    type Target = AiCamera;

    fn deref<'a>(&'a self) -> &'a AiCamera {
        unsafe { &*self.0 }
    }
}

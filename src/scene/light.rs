use std::ops::Deref;

use ffi::AiLight;

// TODO mutable light type
pub struct Light(*const AiLight);

#[doc(hidden)]
pub trait LightInternal {
    fn new(raw_light: *const AiLight) -> Light {
        Light(raw_light)
    }
}

impl LightInternal for Light {}

// TODO remove deref when types are implemented
impl Deref for Light {
    type Target = AiLight;

    fn deref<'a>(&'a self) -> &'a AiLight {
        unsafe { &*self.0 }
    }
}

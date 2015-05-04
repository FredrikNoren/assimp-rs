use std::ops::Deref;

use ffi::AiAnimation;

// TODO mutable animation type
pub struct Animation(*const AiAnimation);

#[doc(hidden)]
pub trait AnimationInternal {
    fn new(raw_animation: *const AiAnimation) -> Animation {
        Animation(raw_animation)
    }
}

impl AnimationInternal for Animation {}

// TODO remove deref when types are implemented
impl Deref for Animation {
    type Target = AiAnimation;

    fn deref<'a>(&'a self) -> &'a AiAnimation {
        unsafe { &*self.0 }
    }
}

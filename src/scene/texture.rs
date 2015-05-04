use std::ops::Deref;

use ffi::AiTexture;

// TODO mutable texture type
pub struct Texture(*const AiTexture);

#[doc(hidden)]
pub trait TextureInternal {
    fn new(raw_texture: *const AiTexture) -> Texture {
        Texture(raw_texture)
    }
}

impl TextureInternal for Texture {}

// TODO remove deref when types are implemented
impl Deref for Texture {
    type Target = AiTexture;

    fn deref<'a>(&'a self) -> &'a AiTexture {
        unsafe { &*self.0 }
    }
}

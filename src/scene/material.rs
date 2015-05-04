use std::ops::Deref;

use ffi::AiMaterial;

// TODO mutable material type
pub struct Material(*const AiMaterial);

#[doc(hidden)]
pub trait MaterialInternal {
    fn new(raw_material: *const AiMaterial) -> Material {
        Material(raw_material)
    }
}

impl MaterialInternal for Material {}

// TODO remove deref when types are implemented
impl Deref for Material {
    type Target = AiMaterial;

    fn deref<'a>(&'a self) -> &'a AiMaterial {
        unsafe { &*self.0 }
    }
}

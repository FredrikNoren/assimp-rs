use std::mem;
use std::ops::{Deref, Index};

use ffi::AiFace;
use libc::c_uint;

// TODO mutable face type
pub struct Face(*const AiFace);

#[doc(hidden)]
pub trait FaceInternal {
    fn new(raw_face: *const AiFace) -> Face {
        Face(raw_face)
    }
}

impl FaceInternal for Face {}

// TODO remove deref when types are implemented
impl Deref for Face {
    type Target = AiFace;

    fn deref<'a>(&'a self) -> &'a AiFace {
        unsafe { &*self.0 }
    }
}

impl Index<isize> for Face {
    type Output = c_uint;
    fn index(&self, index: isize) -> &c_uint {
        unsafe {
            let inner = &*self.0;
            assert!(index < inner.num_indices as isize);
            mem::transmute(inner.indices.offset(index))
        }
    }
}

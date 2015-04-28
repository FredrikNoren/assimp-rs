use libc::c_float;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiVector2D {
    pub x: c_float,
    pub y: c_float
}

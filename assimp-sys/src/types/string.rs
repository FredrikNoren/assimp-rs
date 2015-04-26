use libc::{c_uchar, size_t};
use std::ffi::CString;
use std::fmt::{Debug, Formatter, Result};
use std::str;

pub const MAXLEN : usize = 1024;

#[repr(C)]
#[derive(Copy, Eq)]
pub struct AiString {
    pub length: size_t,
    pub data: [c_uchar; MAXLEN]
}

impl AiString {
    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.data[0..self.length as usize]).unwrap()
    }

    pub fn from_str(s: &str) -> AiString {
        assert!(s.len() < MAXLEN);

        let cstr = CString::new(s).unwrap();
        let bytes = cstr.to_bytes();

        let mut aistr = AiString {
            length: s.len() as size_t,
            data: [0; MAXLEN]
        };
        for i in 0..s.len() {
            aistr.data[i] = bytes[i];
        }
        aistr
    }
}

impl Clone for AiString {
    fn clone(&self) -> AiString { *self }
}

impl Debug for AiString {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl PartialEq for AiString {
    fn eq(&self, other: &AiString) -> bool {
        self.as_str() == other.as_str()
    }
}


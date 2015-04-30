use std::ffi::CString;

use ffi::*;
use libc::{c_float, c_int};

use config::*;
use matrix4::*;

pub enum Property<'a> {
    Bool(PropertyNameBool, bool),
    Integer(PropertyNameInteger, c_int),
    Float(PropertyNameFloat, c_float),
    String(PropertyNameString, &'a str),
    Matrix(PropertyNameMatrix, Matrix4x4)
}

pub struct Importer {
    property_store: *mut AiPropertyStore,
    flags: AiPostProcessSteps
}

impl Importer {
    /// Create a new Importer
    pub fn new() -> Importer {
        Importer {
            property_store: unsafe { aiCreatePropertyStore() },
            flags: AiPostProcessSteps::empty()
        }
    }

    pub fn set_import_property(&mut self, property: Property) {
        match property {
            Property::Bool(PropertyNameBool(name), b) => {
                let cstr = CString::new(name).unwrap().as_ptr();
                unsafe { aiSetImportPropertyInteger(self.property_store, cstr, b as c_int) }
            }
            Property::Integer(PropertyNameInteger(name), i) => {
                let cstr = CString::new(name).unwrap().as_ptr();
                unsafe { aiSetImportPropertyInteger(self.property_store, cstr, i) }
            },
            Property::Float(PropertyNameFloat(name), f) => {
                let cstr = CString::new(name).unwrap().as_ptr();
                unsafe { aiSetImportPropertyFloat(self.property_store, cstr, f) }
            },
            Property::String(PropertyNameString(name), s) => {
                let cstr = CString::new(name).unwrap().as_ptr();
                let value: AiString = From::from(s);
                unsafe { aiSetImportPropertyString(self.property_store, cstr, &value) }
            },
            Property::Matrix(PropertyNameMatrix(name), m) => {
                let cstr = CString::new(name).unwrap().as_ptr();
                unsafe { aiSetImportPropertyMatrix(self.property_store, cstr, &*m) }
            }
        }
    }

    /// Get a list of all supported file extensions
    pub fn get_extension_list() -> Vec<String> {
        let mut ext_list: AiString = Default::default();
        unsafe { aiGetExtensionList(&mut ext_list); }

        let extensions = ext_list.as_ref().split(';');
        extensions.map(|x| x.to_owned()).collect()
    }
}

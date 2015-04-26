use libc::{c_char, c_void, size_t};

use cfileio::*;
use postprocess::*;
use scene::*;
use types::*;

#[repr(C)]
pub struct AiExportFormatDesc {
    pub id: *const c_char,
    pub description: *const c_char,
    pub file_extension: *const c_char
}

#[repr(C)]
pub struct AiExportDataBlob {
    pub size: size_t,
    pub data: *mut c_void,
    pub name: AiString,
    pub next: *mut AiExportDataBlob
}

#[link(name = "assimp")]
extern {
    pub fn aiGetExportFormatCount() -> size_t;
    pub fn aiGetExportFormatDescription(index: size_t) -> *const AiExportFormatDesc;
    pub fn aiCopyScene(input: *const AiScene, output: *mut *mut AiScene);
    pub fn aiFreeScene(input: *const AiScene);
    pub fn aiExportScene(scene: *const AiScene, format_id: *const c_char, filename: *const c_char,
                         preprocessing: AiPostProcessSteps) -> AiReturn;
    pub fn aiExportSceneEx(scene: *const AiScene, format_id: *const c_char, filename: *const c_char,
                           io: *mut AiFileIO, preprocessing: AiPostProcessSteps) -> AiReturn;
    pub fn aiExportSceneToBlob(scene: *const AiScene, format_id: *const c_char,
                               preprocessing: AiPostProcessSteps) -> *const AiExportDataBlob;
    pub fn aiReleaseExportBlob(data: *const AiExportDataBlob);
}

#[macro_use]
extern crate assimp_sys;
use assimp_sys::*;

#[test]
fn test_config_strings() {
    unsafe {
        let stream = aiGetPredefinedLogStream(AiDefaultLogStream::StdOut, ::std::ptr::null());
        aiAttachLogStream(&stream);

        let store = aiCreatePropertyStore();
        aiSetImportPropertyInteger(store, config_str!(AI_CONFIG_GLOB_MEASURE_TIME), 1);
        aiReleasePropertyStore(store);
    }
}

extern crate assimp;
extern crate cgmath;
use assimp::*;
use assimp::config::*;

#[test]
fn test_get_extension_list() {
    let extensions = assimp::Importer::get_extension_list();
    assert!(extensions.len() > 0);
}

#[test]
fn set_import_property_success() {
    // Test one of each property type for type-checking
    let mut importer = Importer::new();
    importer.set_import_property(Property::Bool(GLOB_MEASURE_TIME, true));
    importer.set_import_property(Property::Float(PP_CT_MAX_SMOOTHING_ANGLE, 5.0));
    importer.set_import_property(Property::Integer(GLOB_MULTITHREADING, 2));
    importer.set_import_property(Property::String(PP_RRM_EXCLUDE_LIST, "test"));
    let matrix: Matrix4x4 = From::from(cgmath::Matrix4::identity());
    importer.set_import_property(Property::Matrix(PP_PTV_ROOT_TRANSFORMATION, matrix));
}

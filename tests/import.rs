extern crate assimp;
extern crate assimp_sys;
extern crate cgmath;

use assimp::*;
use assimp::import::structs::*;

#[test]
fn test_get_extension_list() {
    let extensions = assimp::import::get_extension_list();
    println!("{:?}", extensions);
    assert!(extensions.len() > 0);
}

#[test]
fn test_import_from_file_success() {
    let importer = Importer::new();
    let scene = importer.read_file("examples/box.obj");
    assert!(scene.is_ok());
}

#[test]
fn test_import_from_file_failure() {
    let importer = Importer::new();
    let scene = importer.read_file("examples/non_existent_file.obj");
    assert!(scene.is_err());
}

#[test]
fn test_convert_to_mut() {
    let importer = Importer::new();
    let scene = importer.read_file("examples/box.obj").unwrap();
    let scene_mut = SceneMut::from(scene);
    assert_eq!(scene_mut.num_meshes(), 1);
}

#[test]
fn test_apply_postprocessing_success() {
    let mut importer = Importer::new();
    let scene = importer.read_file("examples/box.obj").unwrap();
    importer.triangulate(true);
    let _new_scene = importer.apply_postprocessing(scene).unwrap();
}

#[test]
fn test_apply_postprocessing_failure() {

    // Dodgy way of testing apply_postprocessing failure to work around how Assimp works.
    //
    // Due to aiApplyPostProcessing only accepting new flags but not new properties, it is necessary
    // to apply the property prior to the initial import then disable the flag, then re-enable the
    // flag which will cause the reimport to use the initial types value.
    //
    // Luckily apply_postprocessing is very unlikely to fail in normal usage, so this workaround
    // is only really necessary to deliberately cause failure for testing purposes.

    use assimp::import::structs::PrimitiveType::*;

    let mut importer = Importer::new();
    let all = vec![Point, Line, Triangle, Polygon];
    importer.sort_by_primitive_type(SortByPrimitiveTypeArgs { enable: true, types: all.clone() });
    importer.sort_by_primitive_type(SortByPrimitiveTypeArgs { enable: false, types: all.clone() });
    let scene = importer.read_file("examples/box.obj").unwrap();
    importer.sort_by_primitive_type(SortByPrimitiveTypeArgs { enable: true, types: all.clone() });
    importer.validate_data_structure(true);
    let new_scene = importer.apply_postprocessing(scene);
    assert!(new_scene.is_err());
}

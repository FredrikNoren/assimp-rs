extern crate assimp;

use assimp::Importer;

#[test]
fn test_get_extension_list() {
    let extensions = Importer::get_extension_list();
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
fn test_apply_postprocessing_success() {
    let mut importer = Importer::new();
    let scene = importer.read_file("examples/box.obj").unwrap();
    importer.triangulate(true);
    let _new_scene = importer.apply_postprocessing(scene).unwrap();
}

#[test]
#[should_panic]
fn test_sort_by_primitive_type_panic() {
    use assimp::import::structs::PrimitiveType::*;
    let mut importer = Importer::new();
    let all = vec![Point, Line, Triangle, Polygon];
    importer.sort_by_primitive_type(|x| { x.enable = true; x.remove = all.clone() });
}

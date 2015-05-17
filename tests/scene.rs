extern crate assimp;

use assimp::Importer;

#[test]
fn test_scene_properties() {
    let importer = Importer::new();
    let scene = importer.read_file("examples/spider.obj").unwrap();
    assert_eq!(scene.num_meshes() as usize, scene.mesh_iter().len());
    assert_eq!(scene.num_materials() as usize, scene.material_iter().len());
    assert_eq!(scene.num_animations() as usize, scene.animation_iter().len());
    assert_eq!(scene.num_textures() as usize, scene.texture_iter().len());
    assert_eq!(scene.num_lights() as usize, scene.light_iter().len());
    assert_eq!(scene.num_cameras() as usize, scene.camera_iter().len());
}

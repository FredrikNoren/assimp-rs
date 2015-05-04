extern crate assimp;
extern crate cgmath;

use assimp::{Importer, Scene};

#[test]
fn test_scene_properties() {
    let importer = Importer::new();
    let scene = importer.read_file("examples/spider.obj").unwrap();
    assert_eq!(scene.num_meshes() as usize, scene.meshes().len());
    assert_eq!(scene.num_materials() as usize, scene.materials().len());
    assert_eq!(scene.num_animations() as usize, scene.animations().len());
    assert_eq!(scene.num_textures() as usize, scene.textures().len());
    assert_eq!(scene.num_lights() as usize, scene.lights().len());
    assert_eq!(scene.num_cameras() as usize, scene.cameras().len());
}

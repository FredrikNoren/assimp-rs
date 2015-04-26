extern crate assimp_sys;
use assimp_sys::*;
use std::mem::size_of;

#[test]
#[cfg(target_arch = "x86_64")]
fn check_struct_sizes_x64() {
    assert_eq!(size_of::<AiAnimation>(), 1080);
    assert_eq!(size_of::<AiAnimMesh>(), 168);
    assert_eq!(size_of::<AiBone>(), 1112);
    assert_eq!(size_of::<AiCamera>(), 1088);
    assert_eq!(size_of::<AiColor3D>(), 12);
    assert_eq!(size_of::<AiColor4D>(), 16);
    assert_eq!(size_of::<AiExportDataBlob>(), 1056);
    assert_eq!(size_of::<AiExportFormatDesc>(), 24);
    assert_eq!(size_of::<AiFace>(), 16);
    assert_eq!(size_of::<AiFile>(), 56);
    assert_eq!(size_of::<AiFileIO>(), 24);
    assert_eq!(size_of::<AiImporterDesc>(), 64);
    assert_eq!(size_of::<AiLight>(), 1120);
    assert_eq!(size_of::<AiLogStream>(), 16);
    assert_eq!(size_of::<AiMaterial>(), 16);
    assert_eq!(size_of::<AiMaterialProperty>(), 1056);
    assert_eq!(size_of::<AiMatrix3x3>(), 36);
    assert_eq!(size_of::<AiMatrix4x4>(), 64);
    assert_eq!(size_of::<AiMemoryInfo>(), 32);
    assert_eq!(size_of::<AiMesh>(), 1288);
    assert_eq!(size_of::<AiMeshAnim>(), 1048);
    assert_eq!(size_of::<AiMeshKey>(), 16);
    assert_eq!(size_of::<AiMetadata>(), 24);
    assert_eq!(size_of::<AiMetadataEntry>(), 16);
    assert_eq!(size_of::<AiNode>(), 1144);
    assert_eq!(size_of::<AiNodeAnim>(), 1088);
    assert_eq!(size_of::<AiPlane>(), 16);
    assert_eq!(size_of::<AiPropertyStore>(), 1);
    assert_eq!(size_of::<AiQuaternion>(), 16);
    assert_eq!(size_of::<AiQuatKey>(), 24);
    assert_eq!(size_of::<AiRay>(), 24);
    assert_eq!(size_of::<AiScene>(), 120);
    assert_eq!(size_of::<AiString>(), 1032);
    assert_eq!(size_of::<AiTexel>(), 4);
    assert_eq!(size_of::<AiTexture>(), 24);
    assert_eq!(size_of::<AiUVTransform>(), 20);
    assert_eq!(size_of::<AiVector2D>(), 8);
    assert_eq!(size_of::<AiVector3D>(), 12);
    assert_eq!(size_of::<AiVectorKey>(), 24);
    assert_eq!(size_of::<AiVertexWeight>(), 8);
}

#[test]
#[cfg(target_arch = "x86")]
fn check_struct_sizes_x86() {
    assert_eq!(size_of::<AiAnimation>(), 1064);
    assert_eq!(size_of::<AiAnimMesh>(), 84);
    assert_eq!(size_of::<AiBone>(), 1100);
    assert_eq!(size_of::<AiCamera>(), 1088);
    assert_eq!(size_of::<AiColor3D>(), 12);
    assert_eq!(size_of::<AiColor4D>(), 16);
    assert_eq!(size_of::<AiExportDataBlob>(), 1040);
    assert_eq!(size_of::<AiExportFormatDesc>(), 12);
    assert_eq!(size_of::<AiFace>(), 8);
    assert_eq!(size_of::<AiFile>(), 28);
    assert_eq!(size_of::<AiFileIO>(), 12);
    assert_eq!(size_of::<AiImporterDesc>(), 40);
    assert_eq!(size_of::<AiLight>(), 1112);
    assert_eq!(size_of::<AiLogStream>(), 8);
    assert_eq!(size_of::<AiMaterial>(), 12);
    assert_eq!(size_of::<AiMaterialProperty>(), 1048);
    assert_eq!(size_of::<AiMatrix3x3>(), 36);
    assert_eq!(size_of::<AiMatrix4x4>(), 64);
    assert_eq!(size_of::<AiMemoryInfo>(), 32);
    assert_eq!(size_of::<AiMesh>(), 1176);
    assert_eq!(size_of::<AiMeshAnim>(), 1036);
    assert_eq!(size_of::<AiMeshKey>(), 16);
    assert_eq!(size_of::<AiMetadata>(), 12);
    assert_eq!(size_of::<AiMetadataEntry>(), 8);
    assert_eq!(size_of::<AiNode>(), 1116);
    assert_eq!(size_of::<AiNodeAnim>(), 1060);
    assert_eq!(size_of::<AiPlane>(), 16);
    assert_eq!(size_of::<AiPropertyStore>(), 1);
    assert_eq!(size_of::<AiQuaternion>(), 16);
    assert_eq!(size_of::<AiQuatKey>(), 24);
    assert_eq!(size_of::<AiRay>(), 24);
    assert_eq!(size_of::<AiScene>(), 60);
    assert_eq!(size_of::<AiString>(), 1028);
    assert_eq!(size_of::<AiTexel>(), 4);
    assert_eq!(size_of::<AiTexture>(), 16);
    assert_eq!(size_of::<AiUVTransform>(), 20);
    assert_eq!(size_of::<AiVector2D>(), 8);
    assert_eq!(size_of::<AiVector3D>(), 12);
    assert_eq!(size_of::<AiVectorKey>(), 24);
    assert_eq!(size_of::<AiVertexWeight>(), 8);
}

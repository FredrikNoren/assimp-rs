//! Argument structs for `Importer` post-processing configuration.

use cgmath::Matrix4;
use ffi::config::*;

use math::Matrix4x4;

/// Enumerates components of the Scene and Mesh data structures that can be excluded from the import
/// using the `remove_component` step.
///
/// See `Importer::remove_component` for more details.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentType {
    Normals,
    TangentsAndBitangents,
    Colors,
    TexCoords,
    BoneWeights,
    Animations,
    Textures,
    Lights,
    Cameras,
    Meshes,
    Materials
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UVTransformFlag {
    Scaling,
    Rotation,
    Translation,
    All
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveType {
    Point,
    Line,
    Triangle,
    Polygon
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Import step argument structs
////////////////////////////////////////////////////////////////////////////////////////////////////

// Macro to simplify defining and structs and implementing Default trait
macro_rules! struct_with_defaults {
    (struct $i:ident {
        $($n:ident: $t:ty = $v:expr),*
    }) => (
        pub struct $i {
            pub enable: bool,
            $(pub $n: $t),*
        }

        impl Default for $i {
            fn default() -> $i {
                $i {
                    enable: false,
                    $($n: $v),*
                }
            }
        }
    )
}

struct_with_defaults! {
    struct CalcTangentSpaceArgs {
        max_smoothing_angle: f32 = 45.0,
        texture_channel: i32 = 0
    }
}

struct_with_defaults! {
    struct RemoveComponentArgs {
        components: Vec<ComponentType> = Vec::new()
    }
}

struct_with_defaults! {
    struct GenerateNormalsArgs {
        smooth: bool = false,
        max_smoothing_angle: f32 = 175.0
    }
}

struct_with_defaults! {
    struct SplitLargeMeshesArgs {
        vertex_limit: i32 = AI_SLM_DEFAULT_MAX_VERTICES,
        triangle_limit: i32 = AI_SLM_DEFAULT_MAX_TRIANGLES
    }
}

struct_with_defaults! {
    struct PreTransformVerticesArgs {
        keep_hierarchy: bool = false,
        normalize: bool = false,
        add_root_transformation: bool = false,
        root_transformation: Matrix4x4 = Matrix4x4::from(Matrix4::<f32>::identity())
    }
}

struct_with_defaults! {
    struct LimitBoneWeightsArgs {
        max_weights: i32 = AI_LMW_MAX_WEIGHTS
    }
}

struct_with_defaults! {
    struct ImproveCacheLocalityArgs {
        cache_size: i32 = PP_ICL_DEFAULT_PTCACHE_SIZE
    }
}

struct_with_defaults! {
    struct RemoveRedundantMaterialsArgs {
        exclude_list: String = String::new()
    }
}

struct_with_defaults! {
    struct SortByPrimitiveTypeArgs {
        types: Vec<PrimitiveType> = Vec::new()
    }
}

struct_with_defaults! {
    struct FindDegeneratesArgs {
        remove: bool = false
    }
}

struct_with_defaults! {
    struct FindInvalidDataArgs {
        accuracy: f32 = 0.0
    }
}

struct_with_defaults! {
    struct TransformUVCoordsArgs {
        flags: Vec<UVTransformFlag> = vec![UVTransformFlag::All]
    }
}

struct_with_defaults! {
    struct OptimizeGraphArgs {
        exclude_list: String = String::new()
    }
}

struct_with_defaults! {
    struct SplitByBoneCountArgs {
        max_bones: i32 = AI_SBBC_DEFAULT_MAX_BONES
    }
}

struct_with_defaults! {
    struct DeboneArgs {
        threshold: f32 = AI_DEBONE_THRESHOLD,
        all_or_none: bool = false
    }
}

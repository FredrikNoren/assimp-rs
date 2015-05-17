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


// Macro to simplify defining and structs and implementing Default trait
// NOTE: pub keyword in field definition is to workaround rust issue #24189
macro_rules! struct_with_defaults {
    ($(#[$struct_attr:meta])* struct $i:ident {
        $($(#[$field_attr:meta])* pub $n:ident: $t:ty = $v:expr),*
    }) => (
        $(#[$struct_attr])*
        pub struct $i {
            /// Whether to enable the step. Default: false
            pub enable: bool,
            $($(#[$field_attr])* pub $n: $t),*
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
    /// Arguments for `calc_tangent_space` post-process step.
    struct CalcTangentSpace {
        /// Maximum angle between two vertex tangents used for smoothing. Default: 45.0
        pub max_smoothing_angle: f32 = 45.0,
        /// Source UV channel for tangent space computation. Default: 0
        pub texture_channel: i32 = 0
    }
}

struct_with_defaults! {
    /// Arguments for `remove_component` post-process step.
    struct RemoveComponent {
        /// Specify which components to remove. Default: none
        pub components: Vec<ComponentType> = Vec::new()
    }
}

struct_with_defaults! {
    /// Arguments for `generate_normals` post-process step.
    struct GenerateNormals {
        /// Whether the generated normals are smoothed or not. Default: false
        pub smooth: bool = false,
        /// Maximum angle between two vertex normals used for smoothing. Default: 175.0
        /// Only applies if `smooth` is `true`.
        pub max_smoothing_angle: f32 = 175.0
    }
}

struct_with_defaults! {
    /// Arguments for `split_large_meshes` post-process step.
    struct SplitLargeMeshes {
        /// Maximum number of vertices per mesh. Default: 1000000
        pub vertex_limit: i32 = AI_SLM_DEFAULT_MAX_VERTICES,
        /// Maximum number of triangles per mesh. Default: 1000000
        pub triangle_limit: i32 = AI_SLM_DEFAULT_MAX_TRIANGLES
    }
}

struct_with_defaults! {
    /// Arguments for `pre_transform_vertices` post-process step.
    struct PreTransformVertices {
        /// Whether to keep the existing scene hierarchy. Default: false
        pub keep_hierarchy: bool = false,
        /// Whether to normalize all vertices into the [-1, 1] range. Default: false
        pub normalize: bool = false,
        /// Whether to pre-transform all vertices using the matrix specified in the
        /// `root_transformation` field. Default: false
        pub add_root_transformation: bool = false,
        /// Transformation matrix to use.
        pub root_transformation: Matrix4x4 = Matrix4x4::from(Matrix4::<f32>::identity())
    }
}

struct_with_defaults! {
    /// Arguments for `limit_bone_weights` post-process step.
    struct LimitBoneWeights {
        /// Maximum number of bones that affect a single vertex. Default: 4
        pub max_weights: i32 = AI_LMW_MAX_WEIGHTS
    }
}

struct_with_defaults! {
    /// Arguments for `improve_cache_locality` post-process step.
    struct ImproveCacheLocality {
        /// Set the size of the post-transform vertex cache. Default: 12
        pub cache_size: i32 = PP_ICL_DEFAULT_PTCACHE_SIZE
    }
}

struct_with_defaults! {
    /// Arguments for `remove_redundant_materials` post-process step.
    struct RemoveRedundantMaterials {
        /// Space-delimited list of materials to keep. Identifiers containing whitespace must be
        /// enclosed in single quotes. e.g. `material1 'material 2' material3`.
        pub exclude_list: String = String::new()
    }
}

struct_with_defaults! {
    /// Arguments for `sort_by_primitive_type` post-process step.
    struct SortByPrimitiveType {
        /// List of primitive types to remove. Default: none
        pub remove: Vec<PrimitiveType> = Vec::new()
    }
}

struct_with_defaults! {
    /// Arguments for `find_degenerates` post-process step.
    struct FindDegenerates {
        /// Whether to remove any found degenerates. Default: true
        pub remove: bool = false
    }
}

struct_with_defaults! {
    /// Arguments for `find_invalid_data` post-process step.
    struct FindInvalidData {
        /// Specify the accuracy for considering animation values as invalid. Default: 0
        pub accuracy: f32 = 0.0
    }
}

struct_with_defaults! {
    /// Arguments for `transform_uv_coords` post-process step.
    struct TransformUVCoords {
        /// Specify which UV transforms to evaluate. Default: all
        pub flags: Vec<UVTransformFlag> = vec![UVTransformFlag::All]
    }
}

struct_with_defaults! {
    /// Arguments for `optimize_graph` post-process step.
    struct OptimizeGraph {
        /// Space-delimited list of nodes to keep. Identifiers containing whitespace must be
        /// enclosed in single quotes. e.g. `node1 'node 2' node3`.
        pub exclude_list: String = String::new()
    }
}

struct_with_defaults! {
    /// Arguments for `split_by_bone_count` post-process step.
    struct SplitByBoneCount {
        /// Maximum number of bones per mesh. Default: 60
        pub max_bones: i32 = AI_SBBC_DEFAULT_MAX_BONES
    }
}

struct_with_defaults! {
    /// Arguments for `debone` post-process step.
    struct Debone {
        /// Threshold for considering bone necessary. Default: 1.0
        pub threshold: f32 = AI_DEBONE_THRESHOLD,
        /// Whether to require all bones to meet the threshold before removing any. Default: false
        pub all_or_none: bool = false
    }
}

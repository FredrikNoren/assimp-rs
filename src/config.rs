use ffi::config;

// Typed config strings for added safety
pub struct PropertyNameBool(pub &'static str);
pub struct PropertyNameFloat(pub &'static str);
pub struct PropertyNameInteger(pub &'static str);
pub struct PropertyNameString(pub &'static str);
pub struct PropertyNameMatrix(pub &'static str);

macro_rules! define_typesafe_properties {
    ($($i:ident: $ty:path),+) => (
        $(pub const $i:$ty = $ty(config::$i);)+
    )
}

define_typesafe_properties! {
    // Library settings
    GLOB_MEASURE_TIME: PropertyNameBool,
    IMPORT_NO_SKELETON_MESHES: PropertyNameBool,
    GLOB_MULTITHREADING: PropertyNameInteger,

    // Post-processing settings
    PP_SBBC_MAX_BONES: PropertyNameInteger,
    PP_CT_MAX_SMOOTHING_ANGLE: PropertyNameFloat,
    PP_CT_TEXTURE_CHANNEL_INDEX: PropertyNameInteger,
    PP_GSN_MAX_SMOOTHING_ANGLE: PropertyNameFloat,
    IMPORT_MDL_COLORMAP: PropertyNameString,
    PP_RRM_EXCLUDE_LIST: PropertyNameString,
    PP_PTV_KEEP_HIERARCHY: PropertyNameBool,
    PP_PTV_NORMALIZE: PropertyNameBool,
    PP_PTV_ADD_ROOT_TRANSFORMATION: PropertyNameBool,
    PP_PTV_ROOT_TRANSFORMATION: PropertyNameMatrix,
    PP_FD_REMOVE: PropertyNameBool,
    PP_OG_EXCLUDE_LIST: PropertyNameString,
    PP_SLM_TRIANGLE_LIMIT: PropertyNameInteger,
    PP_SLM_VERTEX_LIMIT: PropertyNameInteger,
    PP_LBW_MAX_WEIGHTS: PropertyNameInteger,
    PP_DB_THRESHOLD: PropertyNameFloat,
    PP_DB_ALL_OR_NONE: PropertyNameBool,
    PP_ICL_PTCACHE_SIZE: PropertyNameInteger,
    PP_RVC_FLAGS: PropertyNameInteger, // TODO restrict to AiComponent flags
    PP_SBP_REMOVE: PropertyNameInteger, // TODO restrict to AiPrimitiveType flags
    PP_FID_ANIM_ACCURACY: PropertyNameFloat,
    PP_TUV_EVALUATE: PropertyNameInteger, // TODO restrict to AiUVTransformFlags flags
    FAVOUR_SPEED: PropertyNameBool,

    // Importer settings
    IMPORT_FBX_READ_ALL_GEOMETRY_LAYERS: PropertyNameBool,
    IMPORT_FBX_READ_ALL_MATERIALS: PropertyNameBool,
    IMPORT_FBX_READ_MATERIALS: PropertyNameBool,
    IMPORT_FBX_READ_CAMERAS: PropertyNameBool,
    IMPORT_FBX_READ_LIGHTS: PropertyNameBool,
    IMPORT_FBX_READ_ANIMATIONS: PropertyNameBool,
    IMPORT_FBX_STRICT_MODE: PropertyNameBool,
    IMPORT_FBX_PRESERVE_PIVOTS: PropertyNameBool,
    IMPORT_FBX_OPTIMIZE_EMPTY_ANIMATION_CURVES: PropertyNameBool,
    IMPORT_GLOBAL_KEYFRAME: PropertyNameInteger,
    IMPORT_MD3_KEYFRAME: PropertyNameInteger,
    IMPORT_MD2_KEYFRAME: PropertyNameInteger,
    IMPORT_MDL_KEYFRAME: PropertyNameInteger,
    IMPORT_MDC_KEYFRAME: PropertyNameInteger,
    IMPORT_SMD_KEYFRAME: PropertyNameInteger,
    IMPORT_UNREAL_KEYFRAME: PropertyNameInteger,
    IMPORT_AC_SEPARATE_BFCULL: PropertyNameBool,
    IMPORT_AC_EVAL_SUBDIVISION: PropertyNameBool,
    UNREAL_HANDLE_FLAGS: PropertyNameBool,
    IMPORT_TER_MAKE_UVS: PropertyNameBool,
    IMPORT_ASE_RECONSTRUCT_NORMALS: PropertyNameBool,
    IMPORT_MD3_HANDLE_MULTIPART: PropertyNameBool,
    IMPORT_MD3_SKIN_NAME: PropertyNameString,
    IMPORT_MD3_SHADER_SRC: PropertyNameString,
    IMPORT_LWO_ONE_LAYER_ONLY: PropertyNameInteger,
    IMPORT_MD5_NO_ANIM_AUTOLOAD: PropertyNameBool,
    IMPORT_LWS_ANIM_START: PropertyNameInteger,
    IMPORT_LWS_ANIM_END: PropertyNameInteger,
    IMPORT_IRR_ANIM_FPS: PropertyNameInteger,
    IMPORT_OGRE_MATERIAL_FILE: PropertyNameString,
    IMPORT_OGRE_TEXTURETYPE_FROM_FILENAME: PropertyNameBool,
    IMPORT_IFC_SKIP_SPACE_REPRESENTATIONS: PropertyNameBool,
    IMPORT_IFC_SKIP_CURVE_REPRESENTATIONS: PropertyNameBool,
    IMPORT_IFC_CUSTOM_TRIANGULATION: PropertyNameBool,
    IMPORT_COLLADA_IGNORE_UP_DIRECTION: PropertyNameBool
}

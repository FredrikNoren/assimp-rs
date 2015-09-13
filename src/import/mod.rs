//! The `import` module contains functionality for importing scenes.
//!
//! TODO write better documentation, at the moment it's mostly copied from Assimp and some of it
//! is incorrect/irrelevant.
//!
//! # Examples
//! ```
//! use assimp::import::Importer;
//!
//! fn main() {
//!     let importer = Importer::new();
//!     let scene = importer.read_file("examples/box.obj");
//! }
//! ```

use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::str;

use ffi::*;
use ffi::config::*;

use math::matrix4::*;
use scene::*;

pub mod structs;
use self::structs::*;

/// The `Importer` type.
///
/// See [module-level documentation](index.html) for examples.
pub struct Importer {
    property_store: *mut AiPropertyStore,
    flags: AiPostProcessSteps
}

impl Importer {
    /// Create a new Importer.
    pub fn new() -> Importer {
        Importer {
            property_store: unsafe { aiCreatePropertyStore() },
            flags: AiPostProcessSteps::empty()
        }
    }

    /// Load a scene from the specified file.
    ///
    /// If the call succeeds, return value is `Ok`, containing the loaded `Scene` structure.
    /// If the call fails, return value is `Err`, containing the error string returned from
    /// the Assimp library.
    pub fn read_file<'a>(&self, file: &str) -> Result<Scene<'a>, &str> {
        let cstr = CString::new(file).unwrap().as_ptr();
        let raw_scene = unsafe {
            aiImportFileExWithProperties(cstr, self.flags, ptr::null_mut(), self.property_store)
        };
        if !raw_scene.is_null() {
            Ok(Scene::from_raw(raw_scene))
        } else {
            let error_str = unsafe { aiGetErrorString() };
            if error_str.is_null() {
                Err("Unknown error")
            } else {
                unsafe {
                    let cstr = CStr::from_ptr(error_str);
                    match str::from_utf8(cstr.to_bytes()) {
                        Ok(s) => Err(s),
                        Err(_) => Err("Unknown error")
                    }
                }
            }
        }
    }

    /// Load a scene from a string.
    ///
    /// If the call succeeds, return value is `Ok`, containing the loaded `Scene` structure.
    /// If the call fails, return value is `Err`, containing the error string returned from
    /// the Assimp library.
    pub fn read_string<'a>(&self, data: &str) -> Result<Scene<'a>, &str> {
        let cstr = CString::new(data).unwrap().as_ptr();
        let raw_scene = unsafe {
            aiImportFromMemoryWithProperties(cstr, data.len() as u32, self.flags, ptr::null_mut(), self.property_store)
        };
        if !raw_scene.is_null() {
            Ok(Scene::from_raw(raw_scene))
        } else {
            let error_str = unsafe { aiGetErrorString() };
            if error_str.is_null() {
                Err("Unknown error")
            } else {
                unsafe {
                    let cstr = CStr::from_ptr(error_str);
                    match str::from_utf8(cstr.to_bytes()) {
                        Ok(s) => Err(s),
                        Err(_) => Err("Unknown error")
                    }
                }
            }
        }
    }

    /// Apply post-processing to an already-imported scene.
    ///
    /// This performs all enabled post-processing steps on an already imported scene. The main
    /// use case for this is to inspect the scene returned by `read_file` before choosing which
    /// additional post-process steps to apply.
    ///
    /// Due to how the Assimp C API works, this isn't as useful as it should be. Currently it isn't
    /// possible to configure properties of post-processing steps after the initial import.
    ///
    /// # Return value
    /// The new scene, with new post-processing steps applied. Note that it is possible for this
    /// method to fail, in which case the return value is `Err`.
    pub fn apply_postprocessing<'a>(&'a self, scene: Scene<'a>) -> Result<Scene, &str> {
        let raw_scene = unsafe { aiApplyPostProcessing(scene.to_raw(), self.flags) };
        if !raw_scene.is_null() {
            // Return original scene, Assimp applies post-processing in-place so returning
            // a new scene object would cause the scene to get double-dropped.
            Ok(scene)
        } else {
            // Assimp frees the scene on failure, dropping would cause the memory to be
            // freed twice so use mem::forget to prevent that happening.
            mem::forget(scene);
            Err("apply_postprocessing failed, see output log for errors.")
        }
    }

    /// Enables time measurements.
    ///
    /// If enabled, measures the time needed for each part of the loading process (i.e. IO time,
    /// importing, postprocessing, ..) and dumps these timings to the output log.
    pub fn measure_time(&mut self, enable: bool) {
        self.set_bool_property(GLOB_MEASURE_TIME, enable);
    }

    /// A hint to Assimp to favour speed against import quality.
    ///
    /// Enabling this option may result in faster loading, but it needn't. It represents just a hint
    /// to loaders and post-processing steps to use faster code paths, if possible.
    pub fn favour_speed(&mut self, enable: bool) {
        self.set_bool_property(FAVOUR_SPEED, enable);
    }

    /// Helper method to set or clear the appropriate import flag
    fn set_import_flag(&mut self, flag: AiPostProcessSteps, value: bool) {
        if value {
            self.flags.insert(flag)
        } else {
            self.flags.remove(flag)
        }
    }

    /// Helper method to set a boolean import property.
    fn set_bool_property(&mut self, name: &str, value: bool) {
        self.set_int_property(name, value as i32)
    }

    /// Helper method to set an integer import property.
    fn set_int_property(&mut self, name: &str, value: i32) {
        let cstr = CString::new(name).unwrap().as_ptr();
        unsafe { aiSetImportPropertyInteger(self.property_store, cstr, value); }
    }

    /// Helper method to set a floating point import property.
    fn set_float_property(&mut self, name: &str, value: f32) {
        let cstr = CString::new(name).unwrap().as_ptr();
        unsafe { aiSetImportPropertyFloat(self.property_store, cstr, value); }
    }

    /// Helper method to set a 4x4 matrix import property.
    fn set_matrix_property(&mut self, name: &str, value: Matrix4x4) {
        let cstr = CString::new(name).unwrap().as_ptr();
        unsafe { aiSetImportPropertyMatrix(self.property_store, cstr, &*value); }
    }

    /// Helper method to set a string import property.
    fn set_string_property(&mut self, name: &str, value: &str) {
        let cstr = CString::new(name).unwrap().as_ptr();
        let aistr: AiString = From::from(value);
        unsafe { aiSetImportPropertyString(self.property_store, cstr, &aistr) }
    }

    /// Calculates the tangents and bitangents for the imported meshes.
    ///
    /// Does nothing if a mesh does not have normals. You might want this post processing step to be
    /// executed if you plan to use tangent space calculations such as normal mapping applied to the
    /// meshes. The `max_smoothing_angle` property allows you to specify a maximum smoothing angle
    /// for the algorithm. However, usually you'll want to leave it at the default value.
    pub fn calc_tangent_space<F: Fn(&mut CalcTangentSpace)>(&mut self, closure: F) {
        let mut args = CalcTangentSpace::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_CALC_TANGENT_SPACE, args.enable);
        if args.enable {
            self.set_float_property(PP_CT_MAX_SMOOTHING_ANGLE, args.max_smoothing_angle);
            self.set_int_property(PP_CT_TEXTURE_CHANNEL_INDEX, args.texture_channel);
        }
    }

    /// Identifies and joins identical vertex data sets within all imported meshes.
    ///
    /// After this step is run, each mesh contains unique vertices, so a vertex may be used by
    /// multiple faces. You usually want to use this post processing step. If your application deals
    /// with indexed geometry, this step is compulsory or you'll just waste rendering time.
    /// If this flag is not specified</b>, no vertices are referenced by more than one face and
    /// no index buffer is required for rendering.
    pub fn join_identical_vertices(&mut self, enable: bool) {
        self.set_import_flag(AIPROCESS_JOIN_IDENTICAL_VERTICES, enable);
    }

    /// Converts all the imported data to a left-handed coordinate space.
    ///
    /// By default the data is returned in a right-handed coordinate space (which OpenGL prefers).
    /// In this space, +X points to the right, +Z points towards the viewer, and +Y points upwards.
    /// In the DirectX coordinate space +X points to the right, +Y points upwards, and +Z points
    /// away from the viewer.
    ///
    /// You'll probably want to consider this flag if you use Direct3D for rendering.
    pub fn make_left_handed(&mut self, enable: bool) {
        self.set_import_flag(AIPROCESS_MAKE_LEFT_HANDED, enable);
    }

    /// Triangulates all faces of all meshes.
    ///
    /// By default the imported mesh data might contain faces with more than 3 indices. For
    /// rendering you'll usually want all faces to be triangles. This post processing step splits up
    /// faces with more than 3 indices into triangles. Line and point primitives are *not* modified!
    /// If you want 'triangles only' with no other kinds of primitives, try the following solution:
    ///
    /// * Enable both `triangulate` and `sort_by_primitive_type`
    /// * Ignore all point and line meshes when you process assimp's output
    pub fn triangulate(&mut self, enable: bool) {
        self.set_import_flag(AIPROCESS_TRIANGULATE, enable);
    }

    /// Removes some parts of the data structure (animations, materials, light sources, cameras,
    /// textures, vertex components).
    ///
    /// The components to be removed are specified in the `flags` property. This is quite useful
    /// if you don't need all parts of the output structure. Vertex colors are rarely used today for
    /// example... Calling this step to remove unneeded data from the pipeline as early as possible
    /// results in increased performance and a more optimized output data structure.
    ///
    /// This step is also useful if you want to force Assimp to recompute normals or tangents.
    /// The corresponding steps don't recompute them if they're already there (loaded from the
    /// source asset). By using this step you can make sure they are NOT there.
    ///
    /// This flag is a poor one, mainly because its purpose is usually misunderstood. Consider the
    /// following case: a 3D model has been exported from a CAD app, and it has per-face vertex
    /// colors. Vertex positions can't be shared, thus the `join_identical_vertices` step fails to
    /// optimize the data because of these nasty little vertex colors. Most apps don't even process
    /// them, so it's all for nothing. By using this step, unneeded components are excluded as early
    /// as possible, thus opening more room for internal optimizations.
    pub fn remove_component<F: Fn(&mut RemoveComponent)>(&mut self, closure: F) {
        use self::structs::ComponentType::*;

        let mut args = RemoveComponent::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_REMOVE_COMPONENT, args.enable);
        if args.enable {
            let flags = args.components.iter().fold(0, |x, &c|
                x | match c {
                    Normals => AICOMPONENT_NORMALS,
                    TangentsAndBitangents => AICOMPONENT_TANGENTS_AND_BITANGENTS,
                    Colors => AICOMPONENT_COLORS,
                    TexCoords => AICOMPONENT_TEXCOORDS,
                    BoneWeights => AICOMPONENT_BONE_WEIGHTS,
                    Animations => AICOMPONENT_ANIMATIONS,
                    Textures => AICOMPONENT_TEXTURES,
                    Lights => AICOMPONENT_LIGHTS,
                    Cameras => AICOMPONENT_CAMERAS,
                    Meshes => AICOMPONENT_MESHES,
                    Materials => AICOMPONENT_MATERIALS
                }.bits()
            );
            self.set_int_property(PP_RVC_FLAGS, flags as i32);
        }
    }

    /// Generates normals for imported meshes.
    ///
    /// This is ignored if normals are already there at the time this flag is evaluated. Model
    /// importers try to load them from the source file, so they're usually already there.
    ///
    /// The `smooth` property specifies how normals are calculated. When set to false, normals are
    /// calculated per face, and shared between all points of a single face, so a single point can
    /// have multiple normals, which forces the library to duplicate vertices in some cases.
    ///
    /// When set to true, normals are calculated per vertex. The `max_smoothing_angle` property
    /// allows you to specify an angle maximum for the normal smoothing algorithm. Normals exceeding
    /// this limit are not smoothed, resulting in a hard seam between two faces. Using a decent
    /// angle here (e.g. 80 degrees) results in very good visual appearance.
    pub fn generate_normals<F: Fn(&mut GenerateNormals)>(&mut self, closure: F) {
        let mut args = GenerateNormals::default();
        closure(&mut args);

        if args.enable {
            if args.smooth {
                self.flags.insert(AIPROCESS_GEN_SMOOTH_NORMALS);
                self.set_float_property(PP_GSN_MAX_SMOOTHING_ANGLE, args.max_smoothing_angle);
            } else {
                self.flags.insert(AIPROCESS_GEN_NORMALS);
            }
        } else {
            self.flags.remove(AIPROCESS_GEN_NORMALS | AIPROCESS_GEN_SMOOTH_NORMALS);
        }
    }

    /// Splits large meshes into smaller sub-meshes.
    ///
    /// This is quite useful for real-time rendering, where the number of triangles which can be
    /// maximally processed in a single draw-call is limited by the video driver/hardware. The
    /// maximum vertex buffer is usually limited too. Both requirements can be met with this step:
    /// you may specify both a triangle and vertex limit for a single mesh.
    ///
    /// The split limits can (and should!) be set through the `vertex_limit` and `triangle_limit`
    /// properties.
    ///
    /// Note that splitting is generally a time-consuming task, but only if there's something to
    /// split. The use of this step is recommended for most users.
    pub fn split_large_meshes<F: Fn(&mut SplitLargeMeshes)>(&mut self, closure: F) {
        let mut args = SplitLargeMeshes::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_SPLIT_LARGE_MESHES, args.enable);
        if args.enable {
            self.set_int_property(PP_SLM_TRIANGLE_LIMIT, args.triangle_limit);
            self.set_int_property(PP_SLM_VERTEX_LIMIT, args.vertex_limit);
        }
    }

    /// Removes the node graph and pre-transforms all vertices with the local transformation
    /// matrices of their nodes.
    ///
    /// The output scene still contains nodes, however there is only a root node with children, each
    /// one referencing only one mesh, and each mesh referencing one material. For rendering, you
    /// can simply render all meshes in order - you don't need to pay attention to local
    /// transformations and the node hierarchy. Animations are removed during this step.
    ///
    /// This step is intended for applications without a scenegraph. The step CAN cause some
    /// problems: if e.g. a mesh of the asset contains normals and another, using the same material
    /// index, does not, they will be brought together, but the first meshes's part of the normal
    /// list is zeroed. However, these artifacts are rare.
    pub fn pre_transform_vertices<F: Fn(&mut PreTransformVertices)>(&mut self, closure: F) {
        let mut args = PreTransformVertices::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_PRE_TRANSFORM_VERTICES, args.enable);
        if args.enable {
            self.set_bool_property(PP_PTV_KEEP_HIERARCHY, args.keep_hierarchy);
            self.set_bool_property(PP_PTV_NORMALIZE, args.normalize);
            self.set_bool_property(PP_PTV_ADD_ROOT_TRANSFORMATION, args.add_root_transformation);
            self.set_matrix_property(PP_PTV_ROOT_TRANSFORMATION, args.root_transformation);
        }
    }

    /// Limits the number of bones simultaneously affecting a single vertex to a maximum value.
    ///
    /// If any vertex is affected by more than the maximum number of bones, the least important
    /// vertex weights are removed and the remaining vertex weights are renormalized so that the
    /// weights still sum up to 1. The default bone weight limit is 4, but you can use the
    /// `max_weights` property to supply your own limit to the post processing step.
    ///
    /// If you intend to perform the skinning in hardware, this post processing step might be of
    /// interest to you.
    pub fn limit_bone_weights<F: Fn(&mut LimitBoneWeights)>(&mut self, closure: F) {
        let mut args = LimitBoneWeights::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_LIMIT_BONE_WEIGHTS, args.enable);
        if args.enable {
            self.set_int_property(PP_LBW_MAX_WEIGHTS, args.max_weights);
        }
    }

    /// Validates the imported scene data structure.
    ///
    /// This makes sure that all indices are valid, all animations and bones are linked correctly,
    /// all material references are correct .. etc.
    ///
    /// It is recommended that you capture Assimp's log output if you use this flag, so you can
    /// easily find out what's wrong if a file fails the validation. The validator is quite strict
    /// and will find *all* inconsistencies in the data structure... It is recommended that plugin
    /// developers use it to debug their loaders. There are two types of validation failures:
    ///
    /// * Error: Error: There's something wrong with the imported data. Further postprocessing is
    ///   not possible and the data is not usable at all. The import fails.
    ///   #Importer::GetErrorString() or #aiGetErrorString() carry the error message around.
    /// * Warning: There are some minor issues (e.g. 1000000 animation keyframes with the same
    ///   time), but further postprocessing and use of the data structure is still safe. Warning
    ///   details are written to the log file, <tt>#AI_SCENE_FLAGS_VALIDATION_WARNING</tt> is set
    ///   in #aiScene::mFlags</li>
    ///
    /// This post-processing step is not time-consuming. Its use is not compulsory, but recommended.
    pub fn validate_data_structure(&mut self, enable: bool) {
        self.set_import_flag(AIPROCESS_VALIDATE_DATA_STRUCTURE, enable);
    }

    /// Reorders triangles for better vertex cache locality.
    ///
    /// The step tries to improve the ACMR (average post-transform vertex cache miss ratio) for all
    /// meshes. The implementation runs in O(n) and is roughly based on the 'tipsify' algorithm (see
    /// [this paper](http://www.cs.princeton.edu/gfx/pubs/Sander_2007_%3ETR/tipsy.pdf)).
    ///
    /// If you intend to render huge models in hardware, this step might be of interest to you.
    /// The `cache_size` property can be used to fine-tune the cache optimization.
    pub fn improve_cache_locality<F: Fn(&mut ImproveCacheLocality)>(&mut self, closure: F) {
        let mut args = ImproveCacheLocality::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_IMPROVE_CACHE_LOCALITY, args.enable);
        if args.enable {
            self.set_int_property(PP_ICL_PTCACHE_SIZE, args.cache_size);
        }
    }

    /// Searches for redundant/unreferenced materials and removes them.
    ///
    /// This is especially useful in combination with the `pre_transform_vertices` and
    /// `optimize_meshes` steps. Both join small meshes with equal characteristics, but they can't
    /// do  their work if two meshes have different materials. Because several material settings are
    /// lost during Assimp's import filters, (and because many exporters don't check for redundant
    /// materials), huge models often have materials which are are defined several times with
    /// exactly the same settings.
    ///
    /// Several material settings not contributing to the final appearance of a surface are ignored
    /// in all comparisons (e.g. the material name). So, if you're passing additional information
    /// through the content pipeline (probably using *magic* material names), don't specify this
    /// flag. Alternatively take a look at the  exclude_list` property.
    pub fn remove_redudant_materials<F: Fn(&mut RemoveRedundantMaterials)>(&mut self, closure: F) {
        let mut args = RemoveRedundantMaterials::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_REMOVE_REDUNDANT_MATERIALS, args.enable);
        if args.enable {
            self.set_string_property(PP_RRM_EXCLUDE_LIST, &args.exclude_list);
        }
    }

    /// This step tries to determine which meshes have normal vectors that are facing inwards and
    /// inverts them.
    ///
    /// The algorithm is simple but effective:
    /// the bounding box of all vertices + their normals is compared against the volume of the
    /// bounding box of all vertices without their normals. This works well for most objects,
    /// problems might occur with planar surfaces. However, the step tries to filter such cases.
    /// The step inverts all in-facing normals. Generally it is recommended to enable this step,
    /// although the result is not always correct.
    pub fn fix_infacing_normals(&mut self, enable: bool) {
        self.set_import_flag(AIPROCESS_FIX_INFACING_NORMALS, enable);
    }

    /// This step splits meshes with more than one primitive type in homogeneous sub-meshes.
    ///
    /// The step is executed after the triangulation step. After the step returns, just one bit is
    /// set in aiMesh::mPrimitiveTypes. This is especially useful for real-time rendering where
    /// point and line primitives are often ignored or rendered separately.
    ///
    /// You can use the `types` property to specify which primitive types you need. This can be
    /// used to easily exclude lines and points, which are rarely used, from the import.
    ///
    /// # Panics
    /// Specifying all possible primitive types for removal is illegal and causes a panic.
    pub fn sort_by_primitive_type<F: Fn(&mut SortByPrimitiveType)>(&mut self, closure: F) {
        use self::structs::PrimitiveType::*;

        let mut args = SortByPrimitiveType::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_SORT_BY_PTYPE, args.enable);
        if args.enable {
            let flags = args.remove.iter().fold(0, |x, &t|
                x | match t {
                    Point => AIPRIMITIVETYPE_POINT,
                    Line => AIPRIMITIVETYPE_LINE,
                    Triangle => AIPRIMITIVETYPE_TRIANGLE,
                    Polygon => AIPRIMITIVETYPE_POLYGON
                }.bits()
            );

            // Removing all primitives is a bad thing and causes Assimp to segfault when
            // used in combination with `validate_data_structure` and `apply_postprocessing`.
            if flags == (AIPRIMITIVETYPE_POINT |
                         AIPRIMITIVETYPE_LINE |
                         AIPRIMITIVETYPE_TRIANGLE |
                         AIPRIMITIVETYPE_POLYGON)
                         .bits() {
                panic!("Trying to remove all possible primitive types is illegal.");
            }

            self.set_int_property(PP_SBP_REMOVE, flags as i32);
        }
    }

    /// This step searches all meshes for degenerate primitives and converts them to proper lines
    /// or points.
    ///
    /// A face is 'degenerate' if one or more of its points are identical. To have the degenerate
    /// stuff not only detected and collapsed but removed, try one of the following procedures:
    ///
    /// 1. If you support lines and points for rendering but don't want the degenerates:
    ///    * Enable the `find_degenerates` step.
    ///    * Set the `remove` property to true. This will cause the step to remove degenerate
    ///      triangles from the import as soon as they're detected. They won't pass any further
    ///      pipeline steps.
    /// 2. If you don't support lines and points at all:
    ///    * Enable the `find_degenerates` step.
    ///    * Enable the `sort_by_primitive_type` step. This moves line and point primitives to
    ///      separate meshes.
    ///    * Set the `components` property to aiPrimitiveType_POINTS | aiPrimitiveType_LINES
    ///      to cause `sort_by_primitive_type` to reject point and line meshes from the scene.
    ///
    /// Degenerate polygons are not necessarily evil and that's why they're not removed by default.
    /// There are several file formats which don't support lines or points, and some exporters
    /// bypass the format specification and write them as degenerate triangles instead.
    pub fn find_degenerates<F: Fn(&mut FindDegenerates)>(&mut self, closure: F) {
        let mut args = FindDegenerates::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_FIND_DEGENERATES, args.enable);
        if args.enable {
            self.set_bool_property(PP_FD_REMOVE, args.remove);
        }
    }

    /// This step searches all meshes for invalid data, such as zeroed normal vectors or invalid UV
    /// coords and removes/fixes them. This is intended to get rid of some common exporter errors.
    ///
    /// This is especially useful for normals. If they are invalid, and the step recognizes this,
    /// they will be removed and can later be recomputed, i.e. by the `gen_normals` step.
    ///
    /// The step will also remove meshes that are infinitely small and reduce animation tracks
    /// consisting of hundreds if redundant keys to a single key.
    /// The `accuracy` property decides the accuracy of the check for duplicate animation tracks.
    pub fn find_invalid_data<F: Fn(&mut FindInvalidData)>(&mut self, closure: F) {
        let mut args = FindInvalidData::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_FIND_INVALID_DATA, args.enable);
        if args.enable {
            self.set_float_property(PP_FID_ANIM_ACCURACY, args.accuracy);
        }
    }

    /// This step converts non-UV mappings (such as spherical or cylindrical mapping) to proper
    /// texture coordinate channels.
    ///
    /// Most applications will support UV mapping only, so you will probably want to specify this
    /// step in every case. Note that Assimp is not always able to match the original mapping
    /// implementation of the 3D app which produced a model perfectly. It's always better to let the
    /// modelling app compute the UV channels - 3ds max, Maya, Blender, LightWave, and Modo do this
    /// for example.
    ///
    /// If this step is not requested, you'll need to process the `AI_MATKEY_MAPPING` material
    /// property in order to display all assets properly.
    pub fn gen_uv_coords(&mut self, enable: bool) {
        self.set_import_flag(AIPROCESS_GEN_UV_COORDS, enable);
    }

    /// This step applies per-texture UV transformations and bakes them into stand-alone vtexture
    /// coordinate channels.
    ///
    /// UV transformations are specified per-texture - see the `AI_MATKEY_UVTRANSFORM` material key
    /// for more information. This step processes all textures with transformed input UV coordinates
    /// and generates a new (pre-transformed) UV channel which replaces the old channel. Most
    /// applications won't support UV transformations, so you will probably want to specify
    /// this step.
    ///
    /// UV transformations are usually implemented in real-time apps by transforming texture
    /// coordinates at vertex shader stage with a 3x3 (homogenous) transformation matrix.
    pub fn transform_uv_coords<F: Fn(&mut TransformUVCoords)>(&mut self, closure: F) {
        use self::structs::UVTransformFlag::*;

        let mut args = TransformUVCoords::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_TRANSFORM_UV_COORDS, args.enable);
        if args.enable {
            let flags = args.flags.iter().fold(0, |x, &f|
                x | match f {
                    Scaling => AI_UVTRAFO_SCALING,
                    Rotation => AI_UVTRAFO_ROTATION,
                    Translation => AI_UVTRAFO_TRANSLATION,
                    All => AI_UVTRAFO_ALL
                }.bits()
            );
            self.set_int_property(PP_TUV_EVALUATE, flags as i32);
        }
    }

    /// This step searches for duplicate meshes and replaces them with references to the first mesh.
    ///
    /// This step takes a while, so don't use it if speed is a concern. Its main purpose is to
    /// workaround the fact that many export file formats don't support instanced meshes, so
    /// exporters need to duplicate meshes. This step removes the duplicates again. Please note that
    /// Assimp does not currently support per-node material assignment to meshes, which means that
    /// identical meshes with different materials are currently *not* joined, although this is
    /// planned for future versions.
    pub fn find_instances(&mut self, enable: bool) {
        self.set_import_flag(AIPROCESS_FIND_INSTANCES, enable);
    }

    /// A postprocessing step to reduce the number of meshes.
    ///
    /// This will, in fact, reduce the number of draw calls.
    ///
    /// This is a very effective optimization and is recommended to be used together with
    /// `optimize_graph`, if possible. The flag is fully compatible with both `split_large_meshes`
    /// and `sort_by_primitive_type`.
    pub fn optimize_meshes(&mut self, enable: bool) {
        self.set_import_flag(AIPROCESS_OPTIMIZE_MESHES, enable);
    }

    /// A postprocessing step to optimize the scene hierarchy.
    ///
    /// Nodes without animations, bones, lights or cameras assigned are collapsed and joined.
    ///
    /// Node names can be lost during this step. If you use special 'tag nodes' to pass additional
    /// information through your content pipeline, use the `exclude_list` property to specify a
    /// list of node names you want to be kept. Nodes matching one of the names in this list won't
    /// be touched or modified.
    ///
    /// Use this flag with caution. Most simple files will be collapsed to a single node, so
    /// complex hierarchies are usually completely lost. This is not useful for editor environments,
    /// but probably a very effective optimization if you just want to get the model data, convert
    /// it to your own format, and render it as fast as possible.
    ///
    /// This flag is designed to be used with `optimize_meshes` for best results.
    ///
    /// 'Crappy' scenes with thousands of extremely small meshes packed in deeply nested nodes exist
    /// for almost all file formats. `optimize_meshes` in combination with `optimize_graph`
    /// usually fixes them all and makes them renderable.
    pub fn optimize_graph<F: Fn(&mut OptimizeGraph)>(&mut self, closure: F) {
        let mut args = OptimizeGraph::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_OPTIMIZE_GRAPH, args.enable);
        if args.enable {
            self.set_string_property(PP_OG_EXCLUDE_LIST, &args.exclude_list);
        }
    }

    /// This step flips all UV coordinates along the y-axis and adjusts material settings and
    /// bitangents accordingly.
    ///
    /// *Output UV coordinate system:*
    ///
    /// ```text
    /// 0y|0y ---------- 1x|0y
    /// |                 |
    /// |                 |
    /// |                 |
    /// 0x|1y ---------- 1x|1y
    /// ```
    ///
    /// You'll probably want to consider this flag if you use Direct3D for rendering.
    pub fn flip_uvs(&mut self, enable: bool) {
        self.set_import_flag(AIPROCESS_FLIP_UVS, enable);
    }

    /// This step adjusts the output face winding order to be CW.
    ///
    /// The default face winding order is counter clockwise (CCW).
    ///
    /// *Output face order:*
    ///
    /// ```text
    ///       x2
    ///
    ///                         x0
    ///  x1
    /// ```
    pub fn flip_winding_order(&mut self, enable: bool) {
        self.set_import_flag(AIPROCESS_FLIP_WINDING_ORDER, enable);
    }

    /// This step splits meshes with many bones into sub-meshes so that each submesh has fewer or
    /// as many bones as a given limit.
    pub fn split_by_bone_count<F: Fn(&mut SplitByBoneCount)>(&mut self, closure: F) {
        let mut args = SplitByBoneCount::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_SPLIT_BY_BONE_COUNT, args.enable);
        if args.enable {
            self.set_int_property(PP_SBBC_MAX_BONES, args.max_bones);
        }
    }

    /// This step removes bones losslessly or according to some threshold.
    ///
    /// In some cases (i.e. formats that require it) exporters are forced to assign dummy bone
    /// weights to otherwise static meshes assigned to animated meshes. Full, weight-based skinning
    /// is expensive while animating nodes is extremely cheap, so this step is offered to clean up
    /// the data in that regard.
    ///
    /// Use the `threshold` property to control this.
    /// Use the `all_or_none` property if you want bones removed if and only if all bones within the
    /// scene qualify for removal.
    pub fn debone<F: Fn(&mut Debone)>(&mut self, closure: F) {
        let mut args = Debone::default();
        closure(&mut args);

        self.set_import_flag(AIPROCESS_DEBONE, args.enable);
        if args.enable {
            self.set_float_property(PP_DB_THRESHOLD, args.threshold);
            self.set_bool_property(PP_DB_ALL_OR_NONE, args.all_or_none);
        }
    }

    /// Global setting to disable generation of skeleton dummy meshes
    ///
    /// Skeleton dummy meshes are generated as a visualization aid in cases which the input data
    /// contains no geometry, but only animation data.
    pub fn import_no_skeleton_meshes(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_NO_SKELETON_MESHES, enable);
    }

    /// Sets the colormap to be used to decode embedded textures in MDL (Quake or 3DGS) files.
    ///
    /// This must be a valid path to a file. The file is 768 (256*3) bytes large and contains RGB
    /// triplets for each of the 256 palette entries. If the file is not found, a default palette
    /// (from Quake 1) is used.
    ///
    /// Default: colormap.lmp
    pub fn import_mdl_colormap(&mut self, path: &str) {
        self.set_string_property(IMPORT_MDL_COLORMAP, path);
    }

    /// Set whether the FBX importer will merge all geometry layers present in the source file or
    /// take only the first.
    ///
    /// Default: true.
    pub fn fbx_read_all_geometry_layers(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_FBX_READ_ALL_GEOMETRY_LAYERS, enable);
    }

    /// Set whether the FBX importer will read all materials present in the source file or take only
    /// the referenced materials. This has no effect if `fbx_read_materials` is false.
    ///
    /// Default: false.
    pub fn fbx_read_all_materials(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_FBX_READ_ALL_MATERIALS, enable);
    }

    /// Set whether the FBX importer will read materials.
    ///
    /// Default: true.
    pub fn fbx_read_materials(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_FBX_READ_MATERIALS, enable);
    }

    /// Set whether the FBX importer will read cameras.
    ///
    /// Default: true.
    pub fn fbx_read_cameras(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_FBX_READ_CAMERAS, enable);
    }

    /// Set whether the FBX importer will read light sources.
    ///
    /// Default: true.
    pub fn fbx_read_lights(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_FBX_READ_LIGHTS, enable);
    }

    /// Set whether the FBX importer will read animations.
    ///
    /// Default: true.
    pub fn fbx_read_animations(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_FBX_READ_ANIMATIONS, enable);
    }

    /// Set whether the FBX importer will act in strict mode in which only FBX 2013 is supported and
    /// any other sub formats are rejected. FBX 2013 is the primary target for the importer, so this
    /// format is best supported and well-tested.
    ///
    /// Default: false.
    pub fn fbx_strict_mode(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_FBX_STRICT_MODE, enable);
    }

    /// Set whether the FBX importer will preserve pivot points for transformations (as extra
    /// nodes). If set to false, pivots and offsets will be evaluated whenever possible.
    ///
    /// Default: true.
    pub fn fbx_preserve_pivots(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_FBX_PRESERVE_PIVOTS, enable);
    }

    /// Specifies whether the FBX importer will drop empty animation curves or animation curves
    /// which match the bind pose transformation over their entire defined range.
    ///
    /// Default: true.
    pub fn fbx_optimize_empty_animation_curves(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_FBX_OPTIMIZE_EMPTY_ANIMATION_CURVES, enable);
    }

    /// Set the vertex animation keyframe to be imported
    ///
    /// Assimp does not support vertex keyframes (only bone animation is supported). The library
    /// reads only one frame of models with vertex animations. This option applies to all importers,
    /// unless overridden for a specific loader.
    ///
    /// Default: first frame.
    pub fn global_keyframe(&mut self, value: i32) {
        self.set_int_property(IMPORT_GLOBAL_KEYFRAME, value);
    }

    /// Override [`global_keyframe`](#method.global_keyframe) property for the MD3 importer.
    pub fn md3_keyframe(&mut self, value: i32) {
        self.set_int_property(IMPORT_MD3_KEYFRAME, value);
    }

    /// Override [`global_keyframe`](#method.global_keyframe) property for the MD2 importer.
    pub fn md2_keyframe(&mut self, value: i32) {
        self.set_int_property(IMPORT_MD2_KEYFRAME, value);
    }

    /// Override [`global_keyframe`](#method.global_keyframe) property for the MDL importer.
    pub fn mdl_keyframe(&mut self, value: i32) {
        self.set_int_property(IMPORT_MDL_KEYFRAME, value);
    }

    /// Override [`global_keyframe`](#method.global_keyframe) property for the MDC importer.
    pub fn mdc_keyframe(&mut self, value: i32) {
        self.set_int_property(IMPORT_MDC_KEYFRAME, value);
    }

    /// Override [`global_keyframe`](#method.global_keyframe) property for the SMD importer.
    pub fn smd_keyframe(&mut self, value: i32) {
        self.set_int_property(IMPORT_SMD_KEYFRAME, value);
    }

    /// Override [`global_keyframe`](#method.global_keyframe) property for the Unreal importer.
    pub fn unreal_keyframe(&mut self, value: i32) {
        self.set_int_property(IMPORT_UNREAL_KEYFRAME, value);
    }

    /// Configures the AC importer to collect all surfaces which have the "Backface cull" flag set
    /// in separate meshes.
    ///
    /// Default: true.
    pub fn ac_separate_bf_cull(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_AC_SEPARATE_BFCULL, enable);
    }

    /// Configures whether the AC importer evaluates subdivision surfaces (indicated by the presence
    /// of the 'subdiv' attribute in the file). By default, Assimp performs the subdivision using
    /// the standard Catmull-Clark algorithm.
    ///
    /// Default: true.
    pub fn ac_eval_subdivision(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_AC_EVAL_SUBDIVISION, enable);
    }

    /// Configures the Unreal importer to separate faces with different surface flags (e.g.
    /// two-sided vs. single-sided).
    ///
    /// Default: true.
    pub fn unreal_handle_flags(&mut self, enable: bool) {
        self.set_bool_property(UNREAL_HANDLE_FLAGS, enable);
    }

    /// Configures the terragen importer to compute UVs for terrains, if not given.
    /// Furthermore a default texture is assigned.
    ///
    /// Default: false.
    pub fn ter_make_uvs(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_TER_MAKE_UVS, enable);
    }

    /// Configures the ASE importer to always reconstruct normal vectors based on the smoothing
    /// groups loaded from the file.
    ///
    /// Default: true.
    pub fn ase_reconstruct_normals(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_ASE_RECONSTRUCT_NORMALS, enable);
    }

    /// Configures the MD3 importer to detect and process multi-part Quake player models.
    ///
    /// These models usually consist of 3 files, lower.md3, upper.md3 and head.md3. If this property
    /// is set to true, Assimp will try to load and combine all three files if one of them is
    /// loaded.
    ///
    /// Default: true.
    pub fn md3_handle_multipart(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_MD3_HANDLE_MULTIPART, enable);
    }

    /// Tells the MD3 importer which skin files to load.
    ///
    /// When loading MD3 files, Assimp checks whether a file `<md3_file_name>_<skin_name>.skin` is
    /// existing. These files are used by Quake III to be able to assign different skins (e.g. red
    /// and blue team) to models. 'default', 'red', 'blue' are typical skin names.
    ///
    /// Default: "default".
    pub fn md3_skin_name(&mut self, name: &str) {
        self.set_string_property(IMPORT_MD3_SKIN_NAME, name);
    }

    /// Specify the Quake 3 shader file to be used for a particular MD3 file. This can also be a
    /// search path.
    ///
    /// By default Assimp's behaviour is as follows: If a MD3 file
    /// `<any_path>/models/<any_q3_subdir>/<model_name>/<file_name>.md3` is loaded, the library
    /// tries to locate the corresponding shader file in `<any_path>/scripts/<model_name>.shader`.
    /// This property overrides this behaviour. It can either specify a full path to the shader to
    /// be loaded or alternatively the path (relative or absolute) to the directory where the
    /// shaders for all MD3s to be loaded reside. Assimp attempts to open
    /// `<dir>/<model_name>.shader` first, `<dir>/<file_name>.shader` is the fallback file.
    /// Note that `<dir>` should have a terminal (back)slash.
    pub fn md3_shader_src(&mut self, path: &str) {
        self.set_string_property(IMPORT_MD3_SHADER_SRC, path);
    }

    /// Configures the LWO importer to load just one layer from the model.
    ///
    /// LWO files consist of layers and in some cases it could be useful to load only one of them.
    /// This property is a string which specifies the name of the layer. If the property is not set
    /// the whole LWO model is loaded. Loading fails if the requested layer is not available.
    /// The layer name may not be empty.
    ///
    /// Default: all layers are loaded.
    pub fn lwo_one_layer_only_str(&mut self, name: &str) {
        self.set_string_property(IMPORT_LWO_ONE_LAYER_ONLY, name);
    }

    /// Configures the LWO importer to load just one layer from the model.
    ///
    /// LWO files consist of layers and in some cases it could be useful to load only one of them.
    /// This property is an integer which specifies the index of the layer. If the property is not
    /// set the whole LWO model is loaded. Loading fails if the requested layer is not available.
    /// The layer index is zero-based.
    ///
    /// Default: all layers are loaded.
    pub fn lwo_one_layer_only_int(&mut self, index: i32) {
        self.set_int_property(IMPORT_LWO_ONE_LAYER_ONLY, index);
    }

    /// Configures the MD5 loader to not load the MD5ANIM file for a MD5MESH file automatically.
    ///
    /// The default strategy is to look for a file with the same name but the MD5ANIM extension in
    /// the same directory. If it is found, it is loaded and combined with the MD5MESH file. This
    /// configuration option can be used to disable this behaviour.
    ///
    /// Default: false.
    pub fn md5_no_anim_autoload(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_MD5_NO_ANIM_AUTOLOAD, enable);
    }

    /// Defines the begin of the time range for which the LWS loader evaluates animations and
    /// computes aiNodeAnims.
    ///
    /// Assimp provides full conversion of LightWave's envelope system, including pre and post
    /// conditions. The loader computes linearly subsampled animation channels with the frame rate
    /// given in the LWS file. This property defines the start time. Note: animation channels are
    /// only generated if a node has at least one envelope with more than one key assigned. This
    /// property is given in frames, '0' is the first frame. By default, if this property is not
    /// set, the importer takes the animation start from the input LWS file ('FirstFrame' line)
    ///
    /// Default: taken from file.
    pub fn lws_anim_start(&mut self, frame: i32) {
        self.set_int_property(IMPORT_LWS_ANIM_START, frame);
    }

    /// Defines the end of the time range for which the LWS loader evaluates animations and
    /// computs aiNodeAnims. See [`lws_anim_start`](#method.lws_anim_start) for more info.
    ///
    /// Default: taken from file.
    pub fn lws_anim_end(&mut self, frame: i32) {
        self.set_int_property(IMPORT_LWS_ANIM_END, frame);
    }

    /// Defines the output frame rate of the IRR loader.
    ///
    /// IRR animations are difficult to convert for Assimp and there will always be a loss of
    /// quality. This setting defines how many keys per second are returned by the converter.
    ///
    /// Default: 100.
    pub fn irr_anim_fps(&mut self, fps: i32) {
        self.set_int_property(IMPORT_IRR_ANIM_FPS, fps);
    }

    /// Ogre Importer will try to find referenced materials from this file.
    ///
    /// Ogre meshes reference with material names, this does not tell Assimp the file where it is
    /// located in. Assimp will try to find the source file in the following order:
    ///
    /// 1. `<material-name>.material`
    /// 2. `<mesh-filename-base>.material`
    /// 3. The material name defined by this config property.
    ///
    /// Default value: Scene.material.
    pub fn ogre_material_file(&mut self, file: &str) {
        self.set_string_property(IMPORT_OGRE_MATERIAL_FILE, file);
    }

    /// Ogre Importer detect the texture usage from its filename.
    ///
    /// Ogre material texture units do not define texture type, the textures usage depends on the
    /// used shader or Ogres fixed pipeline. If this config property is true Assimp will try to
    /// detect the type from the textures filename postfix:
    ///
    /// * _n, _nrm, _nrml, _normal, _normals and _normalmap for normal map
    /// * _s, _spec, _specular and _specularmap for specular map
    /// * _l, _light, _lightmap, _occ and _occlusion for light map
    /// * _disp and _displacement for displacement map
    ///
    /// The matching is case insensitive. Post fix is taken between last "_" and last ".". Default
    /// behavior is to detect type from lower cased texture unit name by matching against:
    /// normalmap, specularmap, lightmap and displacementmap. For both cases if no match is found
    /// aiTextureType_DIFFUSE is used.
    ///
    /// Default: false.
    pub fn ogre_texture_type_from_filename(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_OGRE_TEXTURETYPE_FROM_FILENAME, enable);
    }

    /// Specifies whether the IFC loader skips over IfcSpace elements.
    ///
    /// IfcSpace elements (and their geometric representations) are used to represent, well, free
    /// space in a building storey.
    ///
    /// Default: true.
    pub fn ifc_skip_space_representations(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_IFC_SKIP_SPACE_REPRESENTATIONS, enable);
    }

    /// Specifies whether the IFC loader skips over shape representations of type 'Curve2D'.
    ///
    /// A lot of files contain both a faceted mesh representation and a outline with a presentation
    /// type of 'Curve2D'. Currently Assimp doesn't convert those, so turning this option off just
    // clutters the log with errors.
    ///
    /// Default: true.
    pub fn ifc_skip_curve_representations(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_IFC_SKIP_CURVE_REPRESENTATIONS, enable);
    }

    /// Specifies whether the IFC loader will use its own, custom triangulation algorithm to
    /// triangulate wall and floor meshes.
    ///
    /// If this property is set to false, walls will be either triangulated by `triangulate`
    /// [`triangulate`](#method.triangulate) or will be passed through as huge polygons with
    /// faked holes (i.e. holes that are connected with the outer boundary using a dummy edge).
    /// It is highly recommended to set this property to true if you want triangulated data because
    /// `triangulate` is known to have problems with the kind of polygons that the IFC loader spits
    /// out for complicated meshes.
    ///
    /// Default: true.
    pub fn ifc_custom_triangulation(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_IFC_CUSTOM_TRIANGULATION, enable);
    }

    /// Tells the Collada importer to ignore the up direction specified in the file.
    ///
    /// Default: false.
    pub fn collada_ignore_up_direction(&mut self, enable: bool) {
        self.set_bool_property(IMPORT_COLLADA_IGNORE_UP_DIRECTION, enable);
    }

    /// Get a list of all file extensions supported by Assimp.
    ///
    /// If a file extension is contained in the list this does, of course, not mean that Assimp is
    /// able to load all files with this extension.
    ///
    /// # Return value
    /// `Vec<String>` containing the supported file extensions in lower-case with no leading
    /// wildcard or period characters, e.g. "3ds", "obj", "fbx".
    pub fn get_extension_list() -> Vec<String> {
        let mut ext_list = AiString::default();
        unsafe { aiGetExtensionList(&mut ext_list) };

        let extensions = ext_list.as_ref().split(';');
        extensions.map(|x| x.trim_left_matches("*.").to_owned()).collect()
    }
}

impl Drop for Importer {
    fn drop(&mut self) {
        unsafe { aiReleasePropertyStore(self.property_store) }
    }
}

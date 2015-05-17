use ffi::AiLight;

define_type_and_iterator_indirect! {
    /// Light type (not yet implemented)
    struct Light(&AiLight)
    /// Light iterator type.
    struct LightIter
}

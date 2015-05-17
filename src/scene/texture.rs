use ffi::AiTexture;

define_type_and_iterator_indirect! {
    /// Texture type.
    struct Texture(&AiTexture)
    /// Texture iterator type.
    struct TextureIter
}

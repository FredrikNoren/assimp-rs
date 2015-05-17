use ffi::AiAnimation;

define_type_and_iterator_indirect! {
    /// Animation type (not yet implemented)
    struct Animation(&AiAnimation)
    /// Animation iterator type.
    struct AnimationIter
}

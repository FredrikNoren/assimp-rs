use ffi::AiCamera;

define_type_and_iterator_indirect! {
    /// Camera type (not yet implemented)
    struct Camera(&AiCamera)
    /// Camera iterator type.
    struct CameraIter
}

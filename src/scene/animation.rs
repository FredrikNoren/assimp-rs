use ffi::AiAnimation;
use ffi::AiNodeAnim;
use ffi::AiVectorKey;
use ffi::AiQuatKey;

define_type_and_iterator_indirect! {
    /// Animation type (not yet implemented)
    struct Animation(&AiAnimation)
    /// Animation iterator type.
    struct AnimationIter
}

define_type_and_iterator_indirect! {
    /// NodeAnim type (not yet implemented)
    struct NodeAnim(&AiNodeAnim)
    /// NodeAnim iterator type.
    struct NodeAnimIter
}

define_type_and_iterator_indirect! {
    /// VectorKey type (not yet implemented)
    struct VectorKey(&AiVectorKey)
    /// VectorKey iterator type.
    struct VectorKeyIter
}

define_type_and_iterator_indirect! {
    /// QuatKey type (not yet implemented)
    struct QuatKey(&AiQuatKey)
    /// QuatKey iterator type.
    struct QuatKeyIter
}

impl<'a> NodeAnim<'a> {
    pub fn get_position_key(&self, id: usize) -> Option<VectorKey> {
        if id < self.num_position_keys as usize {
            unsafe { Some(VectorKey::from_raw(self.position_keys.offset(id as isize))) }
        } else {
            None
        }
    }
    pub fn get_rotation_key(&self, id: usize) -> Option<QuatKey> {
        if id < self.num_rotation_keys as usize {
            unsafe { Some(QuatKey::from_raw(self.rotation_keys.offset(id as isize))) }
        } else {
            None
        }
    }
    pub fn get_scaling_key(&self, id: usize) -> Option<VectorKey> {
        if id < self.num_scaling_keys as usize {
            unsafe { Some(VectorKey::from_raw(self.scaling_keys.offset(id as isize))) }
        } else {
            None
        }
    }
}

impl<'a> Animation<'a> {
    pub fn get_node_anim(&self, id: usize) -> Option<NodeAnim> {
        if id < self.num_channels as usize {
            unsafe { Some(NodeAnim::from_raw(*(self.channels.offset(id as isize)))) }
        } else {
            None
        }
    }
}

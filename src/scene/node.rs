use std::slice::from_raw_parts;

use ffi::AiNode;

use math::Matrix4x4;
use super::mesh::*;

/// The `Node` type represents a node in the imported scene hierarchy.
pub struct Node(*mut AiNode);

impl Node {
    /// Returns the name of the node.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the node's transformation matrix.
    pub fn transformation(&self) -> Matrix4x4 {
        Matrix4x4(self.transformation)
    }

    /// Return the parent of this node. Returns `None` if this node is the root node.
    pub fn parent(&self) -> Option<Node> {
        if !self.parent.is_null() {
            Some(Node(self.parent))
        } else {
            None
        }
    }

    /// Returns the number of child nodes.
    pub fn num_children(&self) -> u32 {
        self.num_children
    }

    /// Returns a vector containing all of the child nodes under this node.
    pub fn children(&self) -> Vec<Node> {
        let len = self.num_children as usize;
        unsafe { from_raw_parts(self.children, len).iter().map(|x| Node::new(*x)).collect() }
    }

    /// Returns the number of meshes under this node.
    pub fn num_meshes(&self) -> u32 {
        self.num_meshes
    }

    /// Returns a vector containing all of the meshes under this node. These are indices into
    /// the meshes contained in the `Scene` struct.
    pub fn meshes(&self) -> Vec<usize> {
        let len = self.num_meshes as usize;
        unsafe { from_raw_parts(self.meshes, len).iter().map(|x| *x as usize).collect() }
    }
}

#[doc(hidden)]
pub trait NodeInternal {
    fn new(raw_node: *mut AiNode) -> Node {
        Node(raw_node)
    }
}

impl NodeInternal for Node {}


////////////////////////////////////////////////////////////////////////////////////////////////////
// Implement standard traits
////////////////////////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
mod private {
    use std::ops::Deref;
    use ffi::AiNode;

    impl Deref for super::Node {
        type Target = AiNode;
        fn deref<'a>(&'a self) -> &'a AiNode { unsafe { &*self.0 } }
    }
}

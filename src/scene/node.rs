use std::slice::from_raw_parts;

use ffi::AiNode;

use math::Matrix4x4;

define_type_and_iterator_indirect! {
    /// The `Node` type represents a node in the imported scene hierarchy.
    struct Node(&AiNode)
    /// Node iterator type.
    struct NodeIter
}

impl<'a> Node<'a> {
    /// Returns the name of the node.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the node's transformation matrix.
    pub fn transformation(&self) -> Matrix4x4 {
        Matrix4x4::from_raw(&self.transformation)
    }

    /// Return the parent of this node. Returns `None` if this node is the root node.
    pub fn parent(&self) -> Option<Node> {
        if !self.parent.is_null() {
            Some(Node::from_raw(self.parent))
        } else {
            None
        }
    }

    /// Returns the number of child nodes.
    pub fn num_children(&self) -> u32 {
        self.num_children
    }

    /// Returns a vector containing all of the child nodes under this node.
    pub fn child_iter(&self) -> NodeIter {
        NodeIter::new(self.children as *const *const AiNode,
                      self.num_children as usize)
    }

    /// Returns the number of meshes under this node.
    pub fn num_meshes(&self) -> u32 {
        self.num_meshes
    }

    /// Returns a vector containing all of the meshes under this node. These are indices into
    /// the meshes contained in the `Scene` struct.
    pub fn meshes(&self) -> &[u32] {
        let len = self.num_meshes as usize;
        unsafe { from_raw_parts(self.meshes, len) }
    }

    // TODO metadata
}

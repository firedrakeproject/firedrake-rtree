use rstar::{ParentNode, RTreeNode, RTreeObject, AABB};
use interval_tree::IntervalTreeNode;

use crate::error::RTreeError;
use crate::rtree::{Object2D, Object3D, RTreeDim, RTreeH};

enum NodeRef {
    ITreeNode(*const IntervalTreeNode),
    Parent2D(*const ParentNode<Object2D>),
    Parent3D(*const ParentNode<Object3D>),
    Node2D(*const RTreeNode<Object2D>),
    Node3D(*const RTreeNode<Object3D>),
    EmptyNode,  // root node of empty tree
}

pub enum RTreeNodeH {}

#[no_mangle]
pub extern "C" fn rtree_root_node(tree: *const RTreeH, node: *mut *mut RTreeNodeH) -> RTreeError {
    if tree.is_null() || node.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    let size = match rtree {
        RTreeDim::D1(tree) => tree.size(),
        RTreeDim::D2(tree) => tree.size(),
        RTreeDim::D3(tree) => tree.size(),
    };
    if size == 0 {
        let node_ref = NodeRef::EmptyNode;
        unsafe { *node = Box::into_raw(Box::new(node_ref)) as *mut RTreeNodeH };
        return RTreeError::Success;
    }
    let node_ref = match rtree {
        RTreeDim::D1(tree) => NodeRef::ITreeNode(tree.root().unwrap()),
        RTreeDim::D2(tree) => NodeRef::Parent2D(tree.root() as *const _),
        RTreeDim::D3(tree) => NodeRef::Parent3D(tree.root() as *const _),
    };
    unsafe { *node = Box::into_raw(Box::new(node_ref)) as *mut RTreeNodeH };
    RTreeError::Success
}

#[no_mangle]
pub extern "C" fn rtree_node_children(
    node: *const RTreeNodeH,
    children: *mut *mut *mut RTreeNodeH,
    nchildren: *mut usize,
) -> RTreeError {
    if node.is_null() || children.is_null() || nchildren.is_null() {
        return RTreeError::NullPointer;
    }
    let node_ref = unsafe { &*(node as *const NodeRef) };

    let child_node_refs: Vec<NodeRef> = match node_ref {
        NodeRef::EmptyNode => Vec::new(),
        NodeRef::ITreeNode(ptr) => {
            let node = unsafe { &**ptr };
            let mut children = Vec::new();
            if let Some(left) = &node.left {
                children.push(NodeRef::ITreeNode(left.as_ref() as *const _));
            }
            if let Some(right) = &node.right {
                children.push(NodeRef::ITreeNode(right.as_ref() as *const _));
            }
            children
        }
        NodeRef::Parent2D(ptr) => unsafe { &**ptr }
            .children()
            .iter()
            .map(|child| NodeRef::Node2D(child as *const _))
            .collect(),
        NodeRef::Parent3D(ptr) => unsafe { &**ptr }
            .children()
            .iter()
            .map(|child| NodeRef::Node3D(child as *const _))
            .collect(),
        NodeRef::Node2D(ptr) => match unsafe { &**ptr } {
            RTreeNode::Leaf(_) => Vec::new(),
            RTreeNode::Parent(parent) => parent
                .children()
                .iter()
                .map(|child| NodeRef::Node2D(child as *const _))
                .collect(),
        },
        NodeRef::Node3D(ptr) => match unsafe { &**ptr } {
            RTreeNode::Leaf(_) => Vec::new(),
            RTreeNode::Parent(parent) => parent
                .children()
                .iter()
                .map(|child| NodeRef::Node3D(child as *const _))
                .collect(),
        },
    };

    let mut child_ptrs: Vec<*mut RTreeNodeH> = child_node_refs
        .into_iter()
        .map(|node_ref| Box::into_raw(Box::new(node_ref)) as *mut RTreeNodeH)
        .collect();

    unsafe {
        *nchildren = child_ptrs.len();
        *children = child_ptrs.as_mut_ptr();
    }
    std::mem::forget(child_ptrs);
    RTreeError::Success
}

/// Writes the lower and upper corners of the AABB into `min_out` and `max_out`.
fn write_aabb<const DIM: usize>(aabb: AABB<[f64; DIM]>, min_out: *mut f64, max_out: *mut f64) {
    let lower = aabb.lower();
    let upper = aabb.upper();
    unsafe {
        std::ptr::copy_nonoverlapping(lower.as_ptr(), min_out, DIM);
        std::ptr::copy_nonoverlapping(upper.as_ptr(), max_out, DIM);
    }
}

#[no_mangle]
pub extern "C" fn rtree_node_envelope(
    node: *const RTreeNodeH,
    min_out: *mut f64,
    max_out: *mut f64,
) -> RTreeError {
    if node.is_null() || min_out.is_null() || max_out.is_null() {
        return RTreeError::NullPointer;
    }
    let node_ref = unsafe { &*(node as *const NodeRef) };

    match node_ref {
        NodeRef::EmptyNode => return RTreeError::EmptyNodeEnvelope,
        NodeRef::ITreeNode(ptr) => {
            let node = unsafe { &**ptr };
            let min = node.min;
            let max = node.max;
            unsafe {
                *min_out = min;
                *max_out = max;
            }
        },
        NodeRef::Parent2D(ptr) => write_aabb(unsafe { &**ptr }.envelope(), min_out, max_out),
        NodeRef::Parent3D(ptr) => write_aabb(unsafe { &**ptr }.envelope(), min_out, max_out),
        NodeRef::Node2D(ptr) => write_aabb(unsafe { &**ptr }.envelope(), min_out, max_out),
        NodeRef::Node3D(ptr) => write_aabb(unsafe { &**ptr }.envelope(), min_out, max_out),
    };

    RTreeError::Success
}

#[no_mangle]
pub extern "C" fn rtree_node_free(node: *mut RTreeNodeH) -> RTreeError {
    if node.is_null() {
        return RTreeError::NullPointer;
    }
    drop(unsafe { Box::from_raw(node as *mut NodeRef) });
    RTreeError::Success
}

#[no_mangle]
pub extern "C" fn rtree_node_children_free(children: *mut *mut RTreeNodeH, n: usize) -> RTreeError {
    if children.is_null() {
        return RTreeError::NullPointer;
    }
    let child_ptrs = unsafe { Vec::from_raw_parts(children, n, n) };
    for child in child_ptrs {
        if !child.is_null() {
            drop(unsafe { Box::from_raw(child as *mut NodeRef) });
        }
    }
    RTreeError::Success
}

use rstar::{ParentNode, RTreeNode, RTreeObject};

use crate::error::RTreeError;
use crate::{Object2D, Object3D, RTreeDim, RTreeH};

enum NodeRef {
    Parent2D(*const ParentNode<Object2D>),
    Parent3D(*const ParentNode<Object3D>),
    Node2D(*const RTreeNode<Object2D>),
    Node3D(*const RTreeNode<Object3D>),
}

pub enum RTreeNodeH {}


#[no_mangle]
pub extern "C" fn rtree_root_node(
    tree: *const RTreeH,
    node: *mut *mut RTreeNodeH,
) -> RTreeError {
    if tree.is_null() || node.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    let node_ref = match rtree {
        RTreeDim::D2(tree) => NodeRef::Parent2D(tree.root() as *const _),
        RTreeDim::D3(tree) => NodeRef::Parent3D(tree.root() as *const _),
    };
    unsafe { *node = Box::into_raw(Box::new(node_ref)) as *mut RTreeNodeH };
    RTreeError::Success
}


#[no_mangle]
pub extern "C" fn rtree_node_free(
    node: *mut RTreeNodeH,
) -> RTreeError {
    if node.is_null() {
        return RTreeError::NullPointer;
    }
    drop(unsafe { Box::from_raw(node as *mut NodeRef) });
    RTreeError::Success
}


#[no_mangle]
pub extern "C" fn rtree_node_children_free(
    children: *mut *mut RTreeNodeH,
    n: usize,
) -> RTreeError {
    if children.is_null() {
        return RTreeError::NullPointer;
    }
    let children_vec = unsafe { Vec::from_raw_parts(children, n, n) };
    for child in children_vec {
        if !child.is_null() {
            drop(unsafe { Box::from_raw(child as *mut NodeRef) });
        }
    }
    RTreeError::Success
}

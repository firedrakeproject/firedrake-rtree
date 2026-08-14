use interval_tree::{IntervalTree, IntervalTreeNode};
use rstar::primitives::{GeomWithData, Rectangle};
use rstar::{ParentNode, RTree, RTreeNode, RTreeObject, AABB};

use crate::error::RTreeError;

pub type Object2D = GeomWithData<Rectangle<[f64; 2]>, i64>;
pub type Object3D = GeomWithData<Rectangle<[f64; 3]>, i64>;

pub enum RTreeDim {
    D1(IntervalTree),
    D2(RTree<Object2D>),
    D3(RTree<Object3D>),
}

// Opaque handle for C api
pub enum RTreeH {}

/// Returns a new empty tree with the given dimension.
#[no_mangle]
pub extern "C" fn rtree_create(tree: *mut *mut RTreeH, dim: u32) -> RTreeError {
    if tree.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = match dim {
        1 => RTreeDim::D1(IntervalTree::bulk_load(&[], &[], &[])),
        2 => RTreeDim::D2(RTree::new()),
        3 => RTreeDim::D3(RTree::new()),
        _ => return RTreeError::InvalidDimension,
    };
    unsafe { *tree = Box::into_raw(Box::new(rtree)) as *mut RTreeH };
    RTreeError::Success
}

/// Frees the given tree.
#[no_mangle]
pub extern "C" fn rtree_free(tree: *mut RTreeH) -> RTreeError {
    if tree.is_null() {
        return RTreeError::NullPointer;
    }
    drop(unsafe { Box::from_raw(tree as *mut RTreeDim) });
    RTreeError::Success
}

fn _rtree_get_dimension(tree: &RTreeDim) -> usize {
    match tree {
        RTreeDim::D1(_) => 1,
        RTreeDim::D2(_) => 2,
        RTreeDim::D3(_) => 3,
    }
}

/// Returns the dimension of the tree.
#[no_mangle]
pub extern "C" fn rtree_get_dimension(tree: *const RTreeH, dim: *mut u32) -> RTreeError {
    if tree.is_null() || dim.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    unsafe { *dim = _rtree_get_dimension(rtree) as u32 };
    RTreeError::Success
}

fn _rtree_bulk_load<const DIM: usize>(
    mins: *const f64,
    maxs: *const f64,
    data: *const i64,
    n: usize,
) -> RTree<GeomWithData<Rectangle<[f64; DIM]>, i64>> {
    let mins = unsafe { std::slice::from_raw_parts(mins, n * DIM) };
    let maxs = unsafe { std::slice::from_raw_parts(maxs, n * DIM) };
    let data = unsafe { std::slice::from_raw_parts(data, n) };

    let objects = (0..n)
        .map(|i| {
            let min: [f64; DIM] = std::array::from_fn(|j| mins[i * DIM + j]);
            let max: [f64; DIM] = std::array::from_fn(|j| maxs[i * DIM + j]);
            GeomWithData::new(Rectangle::from_corners(min, max), data[i])
        })
        .collect();

    RTree::bulk_load(objects)
}

fn _interval_tree_bulk_load(
    mins: *const f64,
    maxs: *const f64,
    data: *const i64,
    n: usize,
) -> IntervalTree {
    let mins = unsafe { std::slice::from_raw_parts(mins, n) };
    let maxs = unsafe { std::slice::from_raw_parts(maxs, n) };
    let data = unsafe { std::slice::from_raw_parts(data, n) };

    IntervalTree::bulk_load(mins, maxs, data)
}

/// Returns a new tree containing the given objects. The input arrays must have the same length.
/// Returns an empty tree if the input arrays are empty.
/// Supported dimensions are currently 1, 2, and 3. Returns an InvalidDimension error for unsupported dimensions.
/// You must free the returned tree with `rtree_free`.
#[no_mangle]
pub extern "C" fn rtree_bulk_load(
    tree: *mut *mut RTreeH,
    mins: *const f64,
    maxs: *const f64,
    ids: *const i64,
    n: usize,
    dim: u32,
) -> RTreeError {
    if tree.is_null() {
        return RTreeError::NullPointer;
    }

    if n == 0 {
        let rtree = match dim {
            1 => RTreeDim::D1(IntervalTree::new()),
            2 => RTreeDim::D2(RTree::new()),
            3 => RTreeDim::D3(RTree::new()),
            _ => return RTreeError::InvalidDimension,
        };
        unsafe { *tree = Box::into_raw(Box::new(rtree)) as *mut RTreeH };
        return RTreeError::Success;
    }

    if mins.is_null() || maxs.is_null() || ids.is_null() {
        return RTreeError::NullPointer;
    }

    let rtree = match dim {
        1 => RTreeDim::D1(_interval_tree_bulk_load(mins, maxs, ids, n)),
        2 => RTreeDim::D2(_rtree_bulk_load::<2>(mins, maxs, ids, n)),
        3 => RTreeDim::D3(_rtree_bulk_load::<3>(mins, maxs, ids, n)),
        _ => return RTreeError::InvalidDimension,
    };

    unsafe { *tree = Box::into_raw(Box::new(rtree)) as *mut RTreeH };
    RTreeError::Success
}

/// Returns the ids of all objects in the tree that contain the given point.
/// If no objects contain the point, returns nids_out = 0.
/// You must free the returned ids with `rtree_free_ids`.
#[no_mangle]
pub extern "C" fn rtree_locate_all_at_point(
    tree: *const RTreeH,
    point: *const f64,
    ids_out: *mut *mut i64,
    nids_out: *mut usize,
) -> RTreeError {
    if tree.is_null() || point.is_null() || ids_out.is_null() || nids_out.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    let ids: Vec<i64> = match rtree {
        RTreeDim::D1(tree) => {
            let p: f64 = unsafe { *point };
            tree.locate_all_at_point(p)
        }
        RTreeDim::D2(tree) => {
            let p: [f64; 2] = unsafe { *(point as *const [f64; 2]) };
            tree.locate_all_at_point(p).map(|obj| obj.data).collect()
        }
        RTreeDim::D3(tree) => {
            let p: [f64; 3] = unsafe { *(point as *const [f64; 3]) };
            tree.locate_all_at_point(p).map(|obj| obj.data).collect()
        }
    };

    let n = ids.len();
    let mut ids = ids.into_boxed_slice();
    let ptr = ids.as_mut_ptr();
    std::mem::forget(ids);

    unsafe {
        *nids_out = n;
        *ids_out = ptr;
    }
    RTreeError::Success
}

/// Returns the ids of all objects in the tree that contain each of the given points.
/// `points` is an array of `n_points * dim` doubles
/// The candidate ids are returned in `ids_out`, and `offsets_out`
/// is an array of length `n_points + 1` such that the ids for point `i` are
/// `ids_out[offsets_out[i]..offsets_out[i + 1]]`.
/// You must free `ids_out` (with length `offsets_out[n_points]`) with `rtree_free_ids`,
/// and `offsets_out` (with length `n_points + 1`) with `rtree_free_offsets`.
#[no_mangle]
pub extern "C" fn rtree_locate_all_at_points(
    tree: *const RTreeH,
    points: *const f64,
    n_points: usize,
    ids_out: *mut *mut i64,
    offsets_out: *mut *mut usize,
) -> RTreeError {
    _rtree_locate_all_at_points_impl(
        tree,
        points,
        n_points,
        ids_out,
        offsets_out,
        false,
    )
}

fn _append_point_ids<I>(
    ids: &mut Vec<i64>,
    point_ids: &mut Vec<i64>,
    matches: I,
    deduplicate: bool,
) where
    I: IntoIterator<Item = i64>,
{
    if deduplicate {
        point_ids.clear();
        point_ids.extend(matches);
        point_ids.sort_unstable();
        point_ids.dedup();
        ids.extend_from_slice(point_ids);
    } else {
        ids.extend(matches);
    }
}

fn _rtree_locate_all_at_points_impl(
    tree: *const RTreeH,
    points: *const f64,
    n_points: usize,
    ids_out: *mut *mut i64,
    offsets_out: *mut *mut usize,
    deduplicate: bool,
) -> RTreeError {
    if tree.is_null() || ids_out.is_null() || offsets_out.is_null() {
        return RTreeError::NullPointer;
    }
    if n_points != 0 && points.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    let dim = _rtree_get_dimension(rtree);

    let mut ids: Vec<i64> = Vec::new();
    let mut point_ids: Vec<i64> = Vec::new();
    let mut offsets: Vec<usize> = Vec::with_capacity(n_points + 1);
    offsets.push(0);

    if n_points != 0 {
        let coords = unsafe { std::slice::from_raw_parts(points, n_points * dim) };
        for i in 0..n_points {
            let pt = &coords[i * dim..(i + 1) * dim];
            match rtree {
                RTreeDim::D1(tree) => _append_point_ids(
                    &mut ids,
                    &mut point_ids,
                    tree.locate_all_at_point(pt[0]),
                    deduplicate,
                ),
                RTreeDim::D2(tree) => {
                    let p: [f64; 2] = [pt[0], pt[1]];
                    _append_point_ids(
                        &mut ids,
                        &mut point_ids,
                        tree.locate_all_at_point(p).map(|obj| obj.data),
                        deduplicate,
                    );
                }
                RTreeDim::D3(tree) => {
                    let p: [f64; 3] = [pt[0], pt[1], pt[2]];
                    _append_point_ids(
                        &mut ids,
                        &mut point_ids,
                        tree.locate_all_at_point(p).map(|obj| obj.data),
                        deduplicate,
                    );
                }
            }
            offsets.push(ids.len());
        }
    }

    let mut ids = ids.into_boxed_slice();
    let mut offsets = offsets.into_boxed_slice();
    let ids_ptr = ids.as_mut_ptr();
    let offsets_ptr = offsets.as_mut_ptr();
    std::mem::forget(ids);
    std::mem::forget(offsets);

    unsafe {
        *ids_out = ids_ptr;
        *offsets_out = offsets_ptr;
    }
    RTreeError::Success
}

/// Returns the unique IDs of objects containing each point.
/// IDs are sorted in ascending order within each point's offset.
/// Only useful if your RTree contains multiple bounding boxes sharing the same ID.
/// You must free `ids_out` with `rtree_free_ids` and `offsets_out` with `rtree_free_offsets`.
#[no_mangle]
pub extern "C" fn rtree_locate_all_at_points_unique(
    tree: *const RTreeH,
    points: *const f64,
    n_points: usize,
    ids_out: *mut *mut i64,
    offsets_out: *mut *mut usize,
) -> RTreeError {
    _rtree_locate_all_at_points_impl(
        tree,
        points,
        n_points,
        ids_out,
        offsets_out,
        true,
    )
}

/// Returns the size of the tree, defined as the number of objects in the tree.
/// An empty tree has size 0.
#[no_mangle]
pub extern "C" fn rtree_size(tree: *const RTreeH, size_out: *mut usize) -> RTreeError {
    if tree.is_null() || size_out.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    let size = match rtree {
        RTreeDim::D1(tree) => tree.size(),
        RTreeDim::D2(tree) => tree.size(),
        RTreeDim::D3(tree) => tree.size(),
    };
    unsafe { *size_out = size };
    RTreeError::Success
}

fn _interval_tree_depth(node: &IntervalTreeNode) -> usize {
    let left_depth = node
        .left
        .as_deref()
        .map_or(0, |left| 1 + _interval_tree_depth(left));
    let right_depth = node
        .right
        .as_deref()
        .map_or(0, |right| 1 + _interval_tree_depth(right));
    left_depth.max(right_depth)
}

fn _rtree_depth<T: RTreeObject>(node: &ParentNode<T>) -> usize {
    node.children()
        .iter()
        .map(|child| match child {
            RTreeNode::Leaf(_) => 1,
            RTreeNode::Parent(parent) => 1 + _rtree_depth(parent),
        })
        .max()
        .unwrap_or(0)
}

/// Returns the depth of the tree, defined as the number of edges in the longest path from the root to a leaf.
/// An empty tree has depth 0.
#[no_mangle]
pub extern "C" fn rtree_depth(tree: *const RTreeH, depth_out: *mut usize) -> RTreeError {
    if tree.is_null() || depth_out.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    let size = match rtree {
        RTreeDim::D1(tree) => tree.size(),
        RTreeDim::D2(tree) => tree.size(),
        RTreeDim::D3(tree) => tree.size(),
    };
    if size == 0 {
        unsafe { *depth_out = 0 };
        return RTreeError::Success;
    }
    let depth = match rtree {
        RTreeDim::D1(tree) => _interval_tree_depth(tree.root().unwrap()) + 1,
        RTreeDim::D2(tree) => _rtree_depth(tree.root()),
        RTreeDim::D3(tree) => _rtree_depth(tree.root()),
    };
    unsafe { *depth_out = depth };
    RTreeError::Success
}

fn collect_rtree_bounding_boxes<const DIM: usize>(
    tree: &RTree<GeomWithData<Rectangle<[f64; DIM]>, i64>>,
    level: usize,
) -> Vec<AABB<[f64; DIM]>> {
    if tree.size() == 0 {
        return Vec::new();
    }
    let root = tree.root();
    if level == 0 {
        return vec![root.envelope()];
    }

    let mut nodes: Vec<&RTreeNode<GeomWithData<Rectangle<[f64; DIM]>, i64>>> =
        root.children().iter().collect();

    for _ in 1..level {
        let mut next = Vec::new();

        for node in &nodes {
            if let RTreeNode::Parent(parent) = node {
                next.extend(parent.children().iter());
            }
        }
        if next.is_empty() {
            break;
        }
        nodes = next;
    }
    nodes.into_iter().map(|node| node.envelope()).collect()
}

/// Returns bounding boxes at the specified level of the tree.
#[no_mangle]
pub extern "C" fn rtree_collect_bounding_boxes(
    tree: *const RTreeH,
    level: usize,
    mins_out: *mut *mut f64,
    maxs_out: *mut *mut f64,
    n_boxes_out: *mut usize,
) -> RTreeError {
    if tree.is_null() || mins_out.is_null() || maxs_out.is_null() || n_boxes_out.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };

    match rtree {
        RTreeDim::D1(tree) => {
            let boxes = tree.collect_intervals(level);
            let n_boxes = boxes.len();
            let mut mins = vec![0.0; n_boxes];
            let mut maxs = vec![0.0; n_boxes];
            for (i, (min, max)) in boxes.into_iter().enumerate() {
                mins[i] = min;
                maxs[i] = max;
            }
            let mins_ptr = mins.as_mut_ptr();
            let maxs_ptr = maxs.as_mut_ptr();
            std::mem::forget(mins);
            std::mem::forget(maxs);
            unsafe {
                *mins_out = mins_ptr;
                *maxs_out = maxs_ptr;
                *n_boxes_out = n_boxes;
            }
        }
        RTreeDim::D2(tree) => {
            let boxes = collect_rtree_bounding_boxes::<2>(tree, level);
            let n_boxes = boxes.len();
            let mut mins = vec![0.0; n_boxes * 2];
            let mut maxs = vec![0.0; n_boxes * 2];
            for (i, aabb) in boxes.into_iter().enumerate() {
                mins[i * 2..(i + 1) * 2].copy_from_slice(&aabb.lower());
                maxs[i * 2..(i + 1) * 2].copy_from_slice(&aabb.upper());
            }
            let mins_ptr = mins.as_mut_ptr();
            let maxs_ptr = maxs.as_mut_ptr();
            std::mem::forget(mins);
            std::mem::forget(maxs);
            unsafe {
                *mins_out = mins_ptr;
                *maxs_out = maxs_ptr;
                *n_boxes_out = n_boxes;
            }
        }
        RTreeDim::D3(tree) => {
            let boxes = collect_rtree_bounding_boxes::<3>(tree, level);
            let n_boxes = boxes.len();
            let mut mins = vec![0.0; n_boxes * 3];
            let mut maxs = vec![0.0; n_boxes * 3];
            for (i, aabb) in boxes.into_iter().enumerate() {
                mins[i * 3..(i + 1) * 3].copy_from_slice(&aabb.lower());
                maxs[i * 3..(i + 1) * 3].copy_from_slice(&aabb.upper());
            }
            let mins_ptr = mins.as_mut_ptr();
            let maxs_ptr = maxs.as_mut_ptr();
            std::mem::forget(mins);
            std::mem::forget(maxs);
            unsafe {
                *mins_out = mins_ptr;
                *maxs_out = maxs_ptr;
                *n_boxes_out = n_boxes;
            }
        }
    }
    RTreeError::Success
}

/// Frees the bounding boxes returned by `rtree_collect_bounding_boxes`.
#[no_mangle]
pub extern "C" fn rtree_free_bounding_boxes(
    mins: *mut f64,
    maxs: *mut f64,
    n_boxes: usize,
    dim: u32,
) -> RTreeError {
    let Some(n) = n_boxes.checked_mul(dim as usize) else {
        return RTreeError::InvalidDimension;
    };
    if n > 0 && (mins.is_null() || maxs.is_null()) {
        return RTreeError::NullPointer;
    }
    if !mins.is_null() {
        unsafe { drop(Vec::from_raw_parts(mins, n, n)) };
    }
    if !maxs.is_null() {
        unsafe { drop(Vec::from_raw_parts(maxs, n, n)) };
    }
    RTreeError::Success
}

/// Frees the ids returned by `rtree_locate_all_at_point`.
#[no_mangle]
pub extern "C" fn rtree_free_ids(ids: *mut i64, n: usize) -> RTreeError {
    if ids.is_null() {
        return RTreeError::NullPointer;
    }
    unsafe { drop(Vec::from_raw_parts(ids, n, n)) };
    RTreeError::Success
}

/// Frees the offsets returned by `rtree_locate_all_at_points`.
#[no_mangle]
pub extern "C" fn rtree_free_offsets(offsets: *mut usize, n: usize) -> RTreeError {
    if offsets.is_null() {
        return RTreeError::NullPointer;
    }
    unsafe { drop(Vec::from_raw_parts(offsets, n, n)) };
    RTreeError::Success
}

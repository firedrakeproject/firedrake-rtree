use rstar::primitives::{GeomWithData, Rectangle};
use rstar::RTree;

use crate::error::RTreeError;

pub type Object1D = GeomWithData<Rectangle<[f64; 1]>, usize>;
pub type Object2D = GeomWithData<Rectangle<[f64; 2]>, usize>;
pub type Object3D = GeomWithData<Rectangle<[f64; 3]>, usize>;

pub enum RTreeDim {
    D1(RTree<Object1D>),
    D2(RTree<Object2D>),
    D3(RTree<Object3D>),
}

// Opaque handle for C api
pub enum RTreeH {}

#[no_mangle]
pub extern "C" fn rtree_create(tree: *mut *mut RTreeH, dim: u32) -> RTreeError {
    if tree.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = match dim {
        1 => RTreeDim::D1(RTree::new()),
        2 => RTreeDim::D2(RTree::new()),
        3 => RTreeDim::D3(RTree::new()),
        _ => return RTreeError::InvalidDimension,
    };
    unsafe { *tree = Box::into_raw(Box::new(rtree)) as *mut RTreeH };
    RTreeError::Success
}

#[no_mangle]
pub extern "C" fn rtree_free(tree: *mut RTreeH) -> RTreeError {
    if tree.is_null() {
        return RTreeError::NullPointer;
    }
    drop(unsafe { Box::from_raw(tree as *mut RTreeDim) });
    RTreeError::Success
}

fn _rtree_get_dimension(tree: &RTreeDim) -> u32 {
    match tree {
        RTreeDim::D1(_) => 1,
        RTreeDim::D2(_) => 2,
        RTreeDim::D3(_) => 3,
    }
}

#[no_mangle]
pub extern "C" fn rtree_get_dimension(tree: *const RTreeH, dim: *mut u32) -> RTreeError {
    if tree.is_null() || dim.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    unsafe { *dim = _rtree_get_dimension(rtree) };
    RTreeError::Success
}

fn _rtree_bulk_load<const DIM: usize>(
    mins: *const f64,
    maxs: *const f64,
    data: *const usize,
    n: usize,
) -> RTree<GeomWithData<Rectangle<[f64; DIM]>, usize>> {
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

#[no_mangle]
pub extern "C" fn rtree_bulk_load(
    tree: *mut *mut RTreeH,
    mins: *const f64,
    maxs: *const f64,
    ids: *const usize,
    n: usize,
    dim: u32,
) -> RTreeError {
    if tree.is_null() || mins.is_null() || maxs.is_null() || ids.is_null() {
        return RTreeError::NullPointer;
    }

    let rtree = match dim {
        1 => RTreeDim::D1(_rtree_bulk_load::<1>(mins, maxs, ids, n)),
        2 => RTreeDim::D2(_rtree_bulk_load::<2>(mins, maxs, ids, n)),
        3 => RTreeDim::D3(_rtree_bulk_load::<3>(mins, maxs, ids, n)),
        _ => return RTreeError::InvalidDimension,
    };

    unsafe { *tree = Box::into_raw(Box::new(rtree)) as *mut RTreeH };
    RTreeError::Success
}

#[no_mangle]
pub extern "C" fn rtree_locate_all_at_point(
    tree: *const RTreeH,
    point: *const f64,
    ids_out: *mut *mut usize,
    nids_out: *mut usize,
) -> RTreeError {
    if tree.is_null() || point.is_null() || ids_out.is_null() || nids_out.is_null() {
        return RTreeError::NullPointer;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    let mut ids: Vec<usize> = match rtree {
        RTreeDim::D1(tree) => {
            let p: [f64; 1] = unsafe { *(point as *const [f64; 1]) };
            tree.locate_all_at_point(p).map(|obj| obj.data).collect()
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
    let ptr = ids.as_mut_ptr();
    std::mem::forget(ids);

    unsafe {
        *nids_out = n;
        *ids_out = ptr;
    }
    RTreeError::Success
}

#[no_mangle]
pub extern "C" fn rtree_free_ids(ids: *mut usize, n: usize) -> RTreeError {
    if ids.is_null() {
        return RTreeError::NullPointer;
    }
    unsafe { drop(Vec::from_raw_parts(ids, n, n)) };
    RTreeError::Success
}

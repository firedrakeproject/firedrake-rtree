use rstar::primitives::{GeomWithData, Rectangle};
use rstar::RTree;

type Object2D = GeomWithData<Rectangle<[f64; 2]>, usize>;
type Object3D = GeomWithData<Rectangle<[f64; 3]>, usize>;

enum RTreeDim  {
    D2(RTree<Object2D>),
    D3(RTree<Object3D>),
}

// Opaque handle for C api
pub enum RTreeH {}


#[no_mangle]
pub extern "C" fn rtree_create(
    tree: *mut *mut RTreeH,
    dim: u32,
) {
    if tree.is_null() {
        return;
    }
    let rtree = match dim {
        2 => RTreeDim::D2(RTree::new()),
        3 => RTreeDim::D3(RTree::new()),
        _ => return, // Invalid dimension
    };
    unsafe { *tree = Box::into_raw(Box::new(rtree)) as *mut RTreeH };
}


#[no_mangle]
pub extern "C" fn rtree_free(
    tree: *mut RTreeH,
) {
    if tree.is_null() {
        return;
    }
    drop(unsafe { Box::from_raw(tree as *mut RTreeDim) });
}


fn _rtree_get_dimension(tree: &RTreeDim) -> u32 {
    match tree {
        RTreeDim::D2(_) => 2,
        RTreeDim::D3(_) => 3,
    }
}

#[no_mangle]
pub extern "C" fn rtree_get_dimension(
    tree: *const RTreeH,
    dim: *mut u32,
) {
    if tree.is_null() || dim.is_null() {
        return;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    unsafe { *dim = _rtree_get_dimension(rtree) };
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
    data: *const usize,
    n: usize,
    dim: u32,
) {
    if tree.is_null() || mins.is_null() || maxs.is_null() || data.is_null() {
        return;
    }

    let rtree = match dim {
        2 => RTreeDim::D2(_rtree_bulk_load::<2>(mins, maxs, data, n)),
        3 => RTreeDim::D3(_rtree_bulk_load::<3>(mins, maxs, data, n)),
        _ => return, // Invalid dimension
    };

    unsafe { *tree = Box::into_raw(Box::new(rtree)) as *mut RTreeH };
}


#[no_mangle]
pub extern "C" fn rtree_locate_all_at_point(
    tree: *const RTreeH,
    point: *const f64,
    ids_out: *mut *mut usize,
    nids_out: *mut usize,
) {
    if tree.is_null() || point.is_null() || ids_out.is_null() || nids_out.is_null() {
        return;
    }
    let rtree = unsafe { &*(tree as *const RTreeDim) };
    let dim = _rtree_get_dimension(rtree) as usize;
    let point = unsafe { std::slice::from_raw_parts(point, dim) };

    let ids: Vec<usize> = match rtree {
        RTreeDim::D2(tree) => tree.locate_all_at_point(point.try_into().unwrap()).map(|obj| obj.data).collect(),
        RTreeDim::D3(tree) => tree.locate_all_at_point(point.try_into().unwrap()).map(|obj| obj.data).collect(),
    };

    unsafe {
        *ids_out = ids.as_ptr() as *mut usize;
        *nids_out = ids.len();
    }
}
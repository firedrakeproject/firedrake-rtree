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
    dim: usize,
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

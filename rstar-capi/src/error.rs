#[repr(u16)]
pub enum RTreeError {
    Success = 0,
    NullPointer = 1,
    InvalidDimension = 2,
    NodeNotLeaf = 3,
}

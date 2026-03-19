mod error;
mod node;
mod rtree;

#[cfg(feature = "python")]
mod python {
    use pyo3::prelude::*;

    #[pymodule]
    fn rtree_capi(_py: Python, _m: &Bound<'_, PyModule>) -> PyResult<()> {
        Ok(())
    }
}
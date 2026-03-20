mod error;
mod node;
mod rtree;

#[cfg(feature = "python")]
mod python {
    use pyo3::prelude::*;

    #[pymodule]
    fn firedrake_rtree(_py: Python, _m: &Bound<'_, PyModule>) -> PyResult<()> {
        Ok(())
    }
}
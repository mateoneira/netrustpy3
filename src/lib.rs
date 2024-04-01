pub mod algorithms;
pub mod graph;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
#[pymodule]
fn netrustpy3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<graph::Graph>()?;
    m.add_function(wrap_pyfunction!(algorithms::bfs, m)?)?;
    Ok(())
}

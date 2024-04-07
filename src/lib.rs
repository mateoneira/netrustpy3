pub mod algorithms;
pub mod graph;
pub use algorithms::bfs::bfs;
pub use algorithms::dfs::dfs;
pub use algorithms::shortest_path::single_source_dijkstra;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
#[pymodule]
fn netrustpy3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<graph::Graph>()?;
    m.add_class::<algorithms::bfs::BfsTree>()?;
    m.add_function(wrap_pyfunction!(dfs, m)?)?;
    m.add_function(wrap_pyfunction!(bfs, m)?)?;
    m.add_function(wrap_pyfunction!(single_source_dijkstra, m)?)?;
    Ok(())
}

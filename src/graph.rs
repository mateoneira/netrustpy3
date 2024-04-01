//! Basic graph module
//!
//! #Panics
//!
//! All methods will panic if given out-of-bounds element index.

use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};
///A compact graph representation using adjacency list
#[pyclass]
pub struct Graph {
    #[pyo3(get)]
    pub adj_list: HashMap<usize, Vec<usize>>,
}
#[pymethods]
impl Graph {
    #[new]
    pub fn new(vertices: HashSet<usize>) -> Graph {
        //! Created new graph
        let mut adj_list = HashMap::new();
        for &vertex in &vertices {
            adj_list.insert(vertex, Vec::new());
        }
        Graph { adj_list }
    }

    pub fn add_edge(&mut self, src: usize, dest: usize) {
        //! Adds an undirected edge to the graph
        self.adj_list.get_mut(&src).unwrap().push(dest);
        self.adj_list.get_mut(&dest).unwrap().push(src)
    }

    pub fn num_v(&self) -> usize {
        self.adj_list.len()
    }

    pub fn num_e(&self) -> usize {
        self.adj_list
            .values()
            .map(|neighbours| neighbours.len())
            .sum::<usize>()
            / 2
    }

    #[staticmethod]
    pub fn from_edge_list(adj_list: Vec<(usize, usize)>) -> Graph {
        //! Creates a graph from an adjacency list
        let mut nodes = HashSet::new();
        for &(src, dest) in &adj_list {
            nodes.insert(src);
            nodes.insert(dest);
        }

        let mut graph = Graph::new(nodes);
        for &(src, dest) in &adj_list {
            graph.add_edge(src, dest);
        }
        graph
    }

    pub fn print_adj_list(&self) {
        for (vertex, edges) in &self.adj_list {
            println!("Vertex {} -> {:?}", vertex, edges)
        }
    }
}

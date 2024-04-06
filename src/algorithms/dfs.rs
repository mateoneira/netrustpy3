use crate::graph::Graph;
use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct DfsNode {
    pub parent: Option<usize>,
    pub discovered: usize,
    pub finished: usize,
}

#[pyclass]
pub struct DfsForest {
    pub nodes: HashMap<usize, DfsNode>,
}

impl DfsForest {
    pub fn new() -> Self {
        DfsForest {
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: usize, value: (Option<usize>, usize, usize)) {
        let node = DfsNode {
            parent: value.0,
            discovered: value.1,
            finished: value.2,
        };
        self.nodes.insert(key, node);
    }

    pub fn get(&self, key: &usize) -> Option<&DfsNode> {
        self.nodes.get(key)
    }
}

fn _dfs(
    graph: &Graph,
    current: usize,
    parent: Option<usize>,
    visited: &mut HashSet<usize>,
    dfs_forest: &mut DfsForest,
    time: &mut usize,
) {
    *time += 1;
    let discovered = *time;
    visited.insert(current);

    if let Some(neighbours) = graph.adj_list.get(&current) {
        for (neighbour, _) in neighbours {
            if !visited.contains(&neighbour) {
                _dfs(graph, *neighbour, Some(current), visited, dfs_forest, time);
            }
        }
    }

    *time += 1;
    let finish = *time;

    dfs_forest.insert(current, (parent, discovered, finish))
}
#[pyfunction]
pub fn dfs(graph: &Graph, source: Option<usize>) -> DfsForest {
    //! Depth-first search
    //! Returns a DfsForest
    //! DfsForest containes predecessor subgraph of the depth-first searc
    //! each node contains it's parent, the first time it was discovered
    //! and the time it was finished

    let mut visited: HashSet<usize> = HashSet::new();
    let mut dfs_forest: DfsForest = DfsForest::new();
    let mut time: usize = 0;

    match source {
        Some(current) => {
            _dfs(
                &graph,
                current,
                None,
                &mut visited,
                &mut dfs_forest,
                &mut time,
            );
        }
        None => {
            let nodes = graph.get_nodes();
            for &current in &nodes {
                if !visited.contains(&current) {
                    _dfs(
                        &graph,
                        current,
                        None,
                        &mut visited,
                        &mut dfs_forest,
                        &mut time,
                    );
                }
            }
        }
    }

    dfs_forest
}

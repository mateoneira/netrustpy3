use crate::graph::Graph;
use pyo3::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[pyclass]
pub struct BfsNode {
    pub parent: Option<usize>,
    pub distance: usize,
}

///implement clone for BfsNode
/// This is necessary to return a copy of the node
/// when calling the get method of the BfsTree class
impl Clone for BfsNode {
    fn clone(&self) -> Self {
        BfsNode {
            parent: self.parent,
            distance: self.distance,
        }
    }
}

#[pyclass]
pub struct BfsTree {
    pub nodes: HashMap<usize, BfsNode>,
}

#[pymethods]
impl BfsTree {
    #[new]
    pub fn new() -> Self {
        BfsTree {
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: usize, value: (Option<usize>, usize)) {
        let node = BfsNode {
            parent: value.0,
            distance: value.1,
        };
        self.nodes.insert(key, node);
    }

    pub fn get(&self, key: usize) -> Option<BfsNode> {
        if let Some(node) = self.nodes.get(&key) {
            Some(node.clone())
        } else {
            None
        }
    }

    pub fn get_distance(&self, target: usize) -> usize {
        self.get(target).unwrap().distance
    }

    pub fn get_path(&self, target: usize) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        path.push(target);

        let mut current = self.get(target).unwrap();
        while let Some(node) = current.parent {
            path.push(node);
            current = self.get(node).unwrap();
        }
        path.reverse();
        path
    }
}

#[pyfunction]
pub fn bfs(graph: &Graph, source: usize) -> BfsTree {
    //! Breadth-first search algorithm
    //! Returns a BfsTree containing the parent-child relationships
    //! and distances from the source node
    //! # Arguments
    //! * `graph` - A graph object
    //! * `source` - The source node

    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut bfs_tree: BfsTree = BfsTree::new();

    bfs_tree.insert(source, (None, 0));
    queue.push_back(source);
    visited.insert(source);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbours) = graph.adj_list.get(&current) {
            for (neighbour, _) in neighbours {
                if !visited.contains(&neighbour) {
                    bfs_tree.insert(
                        *neighbour,
                        (Some(current), bfs_tree.get(current).unwrap().distance + 1),
                    );
                    visited.insert(*neighbour);
                    queue.push_back(*neighbour)
                }
            }
        }
    }

    bfs_tree
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bfs() {
        let mut graph = Graph::new((1..6).collect());
        graph.add_edge(1, 2, None);
        graph.add_edge(1, 5, None);
        graph.add_edge(2, 5, None);
        graph.add_edge(2, 3, None);
        graph.add_edge(2, 4, None);
        graph.add_edge(3, 4, None);
        graph.add_edge(4, 5, None);

        let bfs_tree: BfsTree = bfs(&graph, 1);
        let path: Vec<usize> = bfs_tree.get_path(3);
        assert_eq!(bfs_tree.get_distance(3), 2);
        assert_eq!(path, vec![1, 2, 3]);
    }
}

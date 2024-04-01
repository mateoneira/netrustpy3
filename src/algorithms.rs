use crate::graph::Graph;
use pyo3::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
type BfsTree = HashMap<usize, (Option<usize>, usize)>;

#[pyfunction]
pub fn bfs(graph: &Graph, source: usize) -> BfsTree {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut bfs_tree: BfsTree = HashMap::new();

    bfs_tree.insert(source, (None, 0));
    queue.push_back(source);
    visited.insert(source);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbours) = graph.adj_list.get(&current) {
            for &neighbour in neighbours {
                if !visited.contains(&neighbour) {
                    bfs_tree.insert(
                        neighbour,
                        (Some(current), bfs_tree.get(&current).unwrap().1 + 1),
                    );
                    visited.insert(neighbour);
                    queue.push_back(neighbour)
                }
            }
        }
    }

    bfs_tree
}

pub fn find_path_in_bfs_tree(bfs_tree: &BfsTree, target: usize) -> Vec<usize> {
    let mut path: Vec<usize> = Vec::new();
    path.push(target);

    let mut current = bfs_tree.get(&target).unwrap();
    while let Some(node) = current.0 {
        path.push(node);
        current = bfs_tree.get(&node).unwrap();
    }
    path.reverse();
    path
}

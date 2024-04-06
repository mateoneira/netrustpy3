use crate::graph::Graph;
use priority_queue::PriorityQueue;
use pyo3::prelude::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::f32::INFINITY;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct OrderedFloat(f32);

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Greater)
    }
}

///Solves single source shortest-paths
///on weighted graphs G=(V, E)
#[pyfunction]
pub fn single_source_dijkstra(graph: &Graph, src: usize) -> HashMap<usize, (f32, Vec<usize>)> {
    //initialization
    let mut shortest_paths: HashMap<usize, (f32, Vec<usize>)> = graph
        .get_nodes()
        .into_iter()
        .map(|node| {
            (
                node,
                if node == src {
                    (0f32, vec![])
                } else {
                    (INFINITY, vec![])
                },
            )
        })
        .collect();

    //create min-priority queue that will
    //maintain vertices and a distance value
    let mut queue = PriorityQueue::new();
    queue.push(src, Reverse(OrderedFloat(0f32)));

    while let Some((current, Reverse(current_distance))) = queue.pop() {
        let current_path = shortest_paths[&current].1.clone();
        if let Some(neighbours) = graph.adj_list.get(&current) {
            for &(neighbour, weight) in neighbours {
                if let Some((distance, path)) = shortest_paths.get_mut(&neighbour) {
                    let new_distance = current_distance.0 + weight;
                    if new_distance < *distance {
                        *distance = new_distance;
                        let mut new_path = current_path.clone();
                        new_path.push(neighbour);
                        *path = new_path;
                        queue.push(neighbour, Reverse(OrderedFloat(*distance)));
                    }
                }
            }
        }
    }
    shortest_paths
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_source_dijkstra() {
        // Create a sample graph
        let mut graph = Graph::new((0..4).collect());
        graph.add_edge(0, 1, Some(5.0));
        graph.add_edge(0, 2, Some(3.0));
        graph.add_edge(1, 2, Some(2.0));
        graph.add_edge(1, 3, Some(4.0));
        graph.add_edge(2, 3, Some(6.0));

        // Call the function under test
        let shortest_paths = single_source_dijkstra(&graph, 0);

        // Verify the results
        assert_eq!(shortest_paths[&0], (0.0, vec![]));
        assert_eq!(shortest_paths[&1], (5.0, vec![1]));
        assert_eq!(shortest_paths[&2], (3.0, vec![2]));
        assert_eq!(shortest_paths[&3], (9.0, vec![2, 3]));
    }
}

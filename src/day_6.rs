use std::{fmt::Debug, hash::Hash};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, btree_map::BTreeMap, HashMap};

use anyhow::Error;

struct Tree<T: Hash + Clone + Eq + Debug> {
    node_to_index: HashMap<T, usize>,
    adjacency_list: BTreeMap<usize, Vec<usize>>,
    num_nodes: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pub cost: usize,
    pub node: usize,
}

/// Makes State min-heapable
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Tree<String> {
    fn new(input: &str) -> Self {
        let mut node_to_index = HashMap::new();
        let mut adjacency_list = BTreeMap::new();
        let mut index = 0;
        for line in input.split('\n') {
            let mut parts = line.split(')');
            let (node_1, node_2) = (parts.next().unwrap(), parts.next().unwrap());
            if !node_to_index.contains_key(node_1) {
                node_to_index.insert(node_1.to_owned(), index);
                index += 1;
            }
            if !node_to_index.contains_key(node_2) {
                node_to_index.insert(node_2.to_owned(), index);
                index += 1;
            }
            let (index_1, index_2) = (node_to_index[node_1], node_to_index[node_2]);
            adjacency_list
                .entry(index_1)
                .or_insert_with(Vec::new)
                .push(index_2);
            adjacency_list
                .entry(index_2)
                .or_insert_with(Vec::new)
                .push(index_1);
        }
        let num_nodes = node_to_index.len();
        Tree {
            node_to_index,
            adjacency_list,
            num_nodes,
        }
    }
}

impl<T: Hash + Clone + Eq + Debug> Tree<T> {
    fn get_distances_to_start(&self, start_node: usize) -> Vec<Option<usize>> {
        let mut distances = (0..self.num_nodes).map(|_| None).collect::<Vec<_>>();
        let mut heap = BinaryHeap::with_capacity(self.num_nodes);
        heap.push(State {
            cost: 0,
            node: start_node,
        });
        while let Some(State { cost, node }) = heap.pop() {
            if distances[node].is_some() {
                continue;
            }
            distances[node] = Some(cost);
            if let Some(edge_list) = self.adjacency_list.get(&node) {
                for child in edge_list {
                    let next = State {
                        cost: cost + 1,
                        node: *child,
                    };
                    if distances[next.node].is_none() {
                        heap.push(next);
                    }
                }
            }
        }
        distances
    }

    fn get_distance_n1_to_n2(&self, start_node: usize, end_node: usize) -> Option<usize> {
        let mut distances = (0..self.num_nodes)
            .map(|_| ::std::usize::MAX)
            .collect::<Vec<_>>();
        distances[start_node] = 0;
        let mut heap = BinaryHeap::with_capacity(self.num_nodes);
        heap.push(State {
            cost: distances[start_node],
            node: start_node,
        });
        while let Some(State { cost, node }) = heap.pop() {
            if node == end_node {
                return Some(cost);
            }
            if cost > distances[node] {
                continue;
            }
            if let Some(edge_list) = self.adjacency_list.get(&node) {
                for child in edge_list {
                    let next = State {
                        cost: cost + 1,
                        node: *child,
                    };
                    if next.cost < distances[next.node] {
                        distances[next.node] = next.cost;
                        heap.push(next);
                    }
                }
            }
        }
        None
    }
}

pub fn solve_day_6_1(input: &str) -> Result<usize, Error> {
    let tree = Tree::new(input);
    Ok(tree
        .get_distances_to_start(tree.node_to_index["COM"])
        .into_iter()
        .filter_map(|i| i)
        .sum())
}

pub fn solve_day_6_2(input: &str) -> Result<usize, Error> {
    let tree = Tree::new(input);
    Ok(tree
        .get_distance_n1_to_n2(tree.node_to_index["YOU"], tree.node_to_index["SAN"])
        .unwrap()
        - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Error> {
        assert_eq!(
            42,
            solve_day_6_1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")?
        );
        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Error> {
        assert_eq!(
            4,
            solve_day_6_2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN")?
        );
        Ok(())
    }
}

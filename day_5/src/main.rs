/* file:    main.rs
 * author:  garnt
 * date:    12/22/2024
 * desc:    Advent of Code 2024 day 5.
 */

use petgraph::{csr::DefaultIx, graph::NodeIndex, stable_graph::StableGraph};
use std::collections::HashMap;
use std::hash::Hash;

// builds a directed graph, with nodes pointing to any nodes that must come
// after them
fn build_priority_graph<'a, T>(nodes: &Vec<T>, edges: &Vec<(T, T)>) -> StableGraph<T, ()>
where
    T: Eq,
    T: Hash,
    T: Clone,
    T: std::fmt::Debug,
{
    let mut graph: StableGraph<T, ()> = StableGraph::new();
    let mut name_id_map: HashMap<T, NodeIndex<DefaultIx>> = HashMap::new();

    for node_name in nodes {
        let id = graph.add_node(node_name.clone());
        name_id_map.insert(node_name.clone(), id);
    }

    for pair in edges {
        if name_id_map.contains_key(&pair.0) && name_id_map.contains_key(&pair.1) {
            graph.add_edge(
                *name_id_map.get(&pair.0).unwrap(),
                *name_id_map.get(&pair.1).unwrap(),
                (),
            );
        }
    }

    // yield the constructed graph
    graph
}

fn priority_graph_to_ordered_list<T>(mut graph: StableGraph<T, ()>) -> Vec<T>
where
    T: Clone,
{
    let mut list: Vec<T> = Vec::new();
    while graph.node_count() > 0 {
        // find the index of a node that has nothing left with higher priority
        // (no edges directed away from it)
        let mut removed_idx: Option<NodeIndex> = None;
        for node_idx in graph.node_indices() {
            if graph.neighbors(node_idx).count() == 0 {
                removed_idx = Some(node_idx);
                break;
            }
        }

        // if we found something, put it next in the list, then remove the node
        // from the graph
        if let Some(idx) = removed_idx {
            list.push(graph.node_weight(idx).unwrap().clone());
            graph.remove_node(idx);
        } else {
            panic!("Ran out of nodes to remove!");
        }
    }

    // the list is backwards, so reverse it and return
    list.reverse();
    list
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();

    // populate the edges and sets from the input
    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut node_sets: Vec<Vec<usize>> = Vec::new();
    let mut all_edges_read: bool = false;
    for line in input.lines() {
        // there's an empty line between the edges and the node sets
        if line.is_empty() {
            all_edges_read = true;
            continue;
        } else if all_edges_read {
            // parse a set
            let set_vec: Vec<usize> = line
                .split(',')
                .map(|a_str| a_str.parse::<usize>().unwrap())
                .collect();
            node_sets.push(set_vec);
        } else {
            // parse an edge
            let mut str_iter = line.split('|');
            let a: usize = str_iter.next().unwrap().parse::<usize>().unwrap();
            let b: usize = str_iter.next().unwrap().parse::<usize>().unwrap();
            edges.push((a, b));
        }
    }

    // metadata
    println!(
        "edge count: {} - set count: {}",
        edges.len(),
        node_sets.len()
    );

    let mut correct_sum: usize = 0;
    let mut incorrect_sum: usize = 0;
    for node_set in node_sets {
        // make a complete edge priority graph, then make our list
        let graph = build_priority_graph(&node_set, &edges);
        let ordered_list = priority_graph_to_ordered_list(graph);

        if node_set.iter().eq(ordered_list.iter()) {
            let middle_item: &usize = node_set.get(node_set.len() / 2).unwrap();
            correct_sum += middle_item;
        } else {
            let middle_item: &usize = ordered_list.get(ordered_list.len() / 2).unwrap();
            incorrect_sum += middle_item;
        }
    }

    println!("Part 1: {}", correct_sum);
    println!("Part 2: {}", incorrect_sum);
}

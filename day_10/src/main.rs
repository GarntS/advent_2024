/* file:    main.rs
 * author:  garnt
 * date:    12/29/2024
 * desc:    Advent of Code 2024 day 10.
 */

use petgraph::dot::{Config, Dot};
use petgraph::{csr::DefaultIx, graph::NodeIndex, stable_graph::StableGraph};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct VectorN<T, const N_DIMS: usize> {
    coords: [T; N_DIMS],
}

impl<T, const N_DIMS: usize> From<[T; N_DIMS]> for VectorN<T, N_DIMS>
where
    T: Copy,
    T: Default,
{
    fn from(val: [T; N_DIMS]) -> Self {
        Self { coords: val }
    }
}

impl<T, const N_DIMS: usize> From<&[T]> for VectorN<T, N_DIMS>
where
    T: Copy,
    T: Default,
{
    fn from(val_slice: &[T]) -> Self {
        let mut new_coords: [T; N_DIMS] = [T::default(); N_DIMS];
        new_coords.copy_from_slice(val_slice);
        Self { coords: new_coords }
    }
}

impl<T, const N_DIMS: usize> VectorN<T, N_DIMS> {
    /// constructs a new VectorN with the default values
    fn new() -> Self
    where
        T: Copy,
        T: Default,
    {
        Self {
            coords: [T::default(); N_DIMS],
        }
    }
}

impl<T, const N_DIMS: usize> std::ops::Index<usize> for VectorN<T, N_DIMS> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        if index < N_DIMS {
            &self.coords[index]
        } else {
            panic!("Invalid index into VectorN<{}> - {}", N_DIMS, index);
        }
    }
}

impl<T, const N_DIMS: usize> std::ops::Add for VectorN<T, N_DIMS>
where
    T: std::ops::Add<Output = T>,
    T: Copy,
    T: Default,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_coords: [T; N_DIMS] = [T::default(); N_DIMS];
        for i in 0..N_DIMS {
            new_coords[i] = self.coords[i] + other.coords[i];
        }
        Self { coords: new_coords }
    }
}

impl<T, const N_DIMS: usize> std::ops::Sub for VectorN<T, N_DIMS>
where
    T: std::ops::Sub<Output = T>,
    T: Copy,
    T: Default,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut new_coords: [T; N_DIMS] = [T::default(); N_DIMS];
        for i in 0..N_DIMS {
            new_coords[i] = self.coords[i] - other.coords[i];
        }
        Self { coords: new_coords }
    }
}

#[derive(Clone, Copy, Debug)]
enum DirectionSteps {
    HorizLeft,
    HorizRight,
    VertUp,
    VertDown,
    /*DiagUL,
    DiagUR,
    DiagDL,
    DiagDR,*/
}

impl DirectionSteps {
    // returns an (x_step, y_step) pair used for iteration
    fn to_step_vec(&self) -> VectorN<isize, 2> {
        match *self {
            DirectionSteps::HorizLeft => [-1, 0],
            DirectionSteps::HorizRight => [1, 0],
            DirectionSteps::VertUp => [0, -1],
            DirectionSteps::VertDown => [0, 1],
            /*DirectionSteps::DiagUL => [-1, -1],
            DirectionSteps::DiagUR => [1, -1],
            DirectionSteps::DiagDL => [-1, 1],
            DirectionSteps::DiagDR => [1, 1],*/
        }
        .into()
    }

    // returns an iterator over all the variants of DirectionSteps
    fn iterate_all() -> impl Iterator<Item = DirectionSteps> {
        [
            DirectionSteps::HorizLeft,
            DirectionSteps::HorizRight,
            DirectionSteps::VertUp,
            DirectionSteps::VertDown,
            /*DirectionSteps::DiagUL,
            DirectionSteps::DiagUR,
            DirectionSteps::DiagDL,
            DirectionSteps::DiagDR,*/
        ]
        .iter()
        .copied()
    }
}

// I'm on record that I hate this function
fn hamfist_into_2d_arr<const WIDTH: usize, const HEIGHT: usize>(
    input: &str,
) -> Result<[[usize; WIDTH]; HEIGHT], ()> {
    let mut usize_arr: [[usize; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    for (idx, line) in input.lines().enumerate() {
        if line.len() != WIDTH {
            return Err(());
        }

        let line_arr: [usize; WIDTH] = line
            .chars()
            .map(|char| char.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        let _ = std::mem::replace(usize_arr.get_mut(idx).ok_or(())?, line_arr);
    }

    // return our newly-constructed char array
    Ok(usize_arr)
}

// NOTE: points are 0-based
// NOTE: points provided as (x, y) even though indexes to array are backwards
fn val_at_point<const WIDTH: usize, const HEIGHT: usize>(
    arr: &[[usize; WIDTH]; HEIGHT],
    point: VectorN<usize, 2>,
) -> usize {
    let x: usize = point[0].min(WIDTH - 1);
    let y: usize = point[1].min(HEIGHT - 1);
    arr[y][x]
}

fn find_neighbors_with_val<const WIDTH: usize, const HEIGHT: usize>(
    arr: &[[usize; WIDTH]; HEIGHT],
    point: VectorN<usize, 2>,
    search_val: usize,
) -> Vec<VectorN<usize, 2>> {
    let mut neighbor_positions: Vec<VectorN<usize, 2>> = Vec::new();
    for direction in DirectionSteps::iterate_all() {
        // grab step, check bounds
        let step_pair = direction.to_step_vec();

        // add the step to our point to find the position we're checking
        let cur_point: VectorN<usize, 2> =
            if let Some(cur_x) = point[0].checked_add_signed(step_pair[0]) {
                if let Some(cur_y) = point[1].checked_add_signed(step_pair[1]) {
                    [cur_x, cur_y].into()
                } else {
                    continue;
                }
            } else {
                continue;
            };

        // make sure our point is within bounds
        if cur_point[0] >= WIDTH || cur_point[1] >= HEIGHT {
            continue;
        }

        if val_at_point::<WIDTH, HEIGHT>(arr, cur_point) == search_val {
            // we found the value we were looking for, so add the position
            neighbor_positions.push(cur_point);
        }
    }

    // yield the located positions
    neighbor_positions
}

/// builds a directed graph representing the trail
fn build_trail_graph<const WIDTH: usize, const HEIGHT: usize>(
    arr: &[[usize; WIDTH]; HEIGHT],
    max_val: usize,
) -> StableGraph<usize, ()> {
    let mut graph: StableGraph<usize, ()> = StableGraph::new();
    let mut pos_id_map: HashMap<VectorN<usize, 2>, NodeIndex<DefaultIx>> = HashMap::new();

    // check each position, looking for start points with value 0
    for start_x in 0..WIDTH {
        for start_y in 0..HEIGHT {
            let start_pos: VectorN<usize, 2> = [start_x, start_y].into();

            // start at 0
            if val_at_point(arr, start_pos) != 0 {
                continue;
            }

            // add the new start position to the graph
            let node_id = graph.add_node(val_at_point(arr, start_pos));
            pos_id_map.insert(start_pos, node_id);

            let mut visit_queue: Vec<VectorN<usize, 2>> = vec![start_pos];
            while let Some(cur_pos) = visit_queue.pop() {
                // if the value at the current position is max_val, stop
                if val_at_point(arr, cur_pos) == max_val {
                    continue;
                }

                // iterate over each neighbor position that contains the next
                // value in-sequence
                for neighbor_pos in
                    find_neighbors_with_val(arr, cur_pos, val_at_point(arr, cur_pos) + 1)
                {
                    if pos_id_map.contains_key(&neighbor_pos) {
                        // if a node already exists for the point we're going to
                        // add to the queue, we've already walked it. Make an
                        // edge to it, and don't add it to the queue again.
                        let cur_idx = pos_id_map.get(&cur_pos).unwrap();
                        let neighbor_idx = pos_id_map.get(&neighbor_pos).unwrap();
                        graph.add_edge(*cur_idx, *neighbor_idx, ());
                    } else {
                        // if a node hasn't been created yet for the new point,
                        // add it to the graph and pos_id_map, and add it to the
                        // visit queue
                        // add to graph + map
                        let neighbor_idx = graph.add_node(val_at_point(arr, neighbor_pos));
                        pos_id_map.insert(neighbor_pos, neighbor_idx);

                        // add edge to graph
                        let cur_idx = pos_id_map.get(&cur_pos).unwrap().clone();
                        graph.add_edge(cur_idx, neighbor_idx, ());

                        // add pos to visit_queue
                        visit_queue.push(neighbor_pos);
                    }
                }
            }
        }
    }

    graph
}

/// returns the number of nodes with the provided weight that are reachable from
/// the provided start node
fn n_reachable_nodes_with_weight<T>(
    graph: &StableGraph<T, ()>,
    idx: NodeIndex<DefaultIx>,
    weight: T,
) -> usize
where
    T: Copy,
    T: PartialEq,
{
    let mut count: usize = 0;
    let mut visited_idx_set: HashSet<NodeIndex<DefaultIx>> = HashSet::new();
    let mut visit_queue: Vec<NodeIndex<DefaultIx>> = vec![idx];
    while let Some(cur_idx) = visit_queue.pop() {
        if visited_idx_set.contains(&cur_idx) {
            continue;
        }
        visited_idx_set.insert(cur_idx);
        if graph.node_weight(cur_idx).is_some_and(|w| *w == weight) {
            count += 1;
        } else {
            for neighbor in graph.neighbors(cur_idx) {
                visit_queue.push(neighbor);
            }
        }
    }

    count
}

/// returns the number of nodes with the provided weight that are reachable from
/// the provided start node
fn n_paths_to_nodes_with_weight<T>(
    graph: &StableGraph<T, ()>,
    idx: NodeIndex<DefaultIx>,
    weight: T,
) -> usize
where
    T: Copy,
    T: PartialEq,
{
    let mut count: usize = 0;
    let mut visit_queue: Vec<NodeIndex<DefaultIx>> = vec![idx];
    while let Some(cur_idx) = visit_queue.pop() {
        if graph.node_weight(cur_idx).is_some_and(|w| *w == weight) {
            count += 1;
        } else {
            for neighbor in graph.neighbors(cur_idx) {
                visit_queue.push(neighbor);
            }
        }
    }

    count
}

/// the entrypoint
fn main() {
    // these have to be constant due to a compiler bug.
    // test values
    //const WIDTH: usize = 8;
    //const HEIGHT: usize = 8;
    // final values
    const WIDTH: usize = 53;
    const HEIGHT: usize = 53;

    // metadata
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "width: {} - height: {}",
        input.lines().next().unwrap().len(),
        input.lines().count(),
    );

    // part 1
    let hamfist_array: [[usize; WIDTH]; HEIGHT] =
        hamfist_into_2d_arr::<WIDTH, HEIGHT>(&input).unwrap();
    let trail_graph = build_trail_graph::<WIDTH, HEIGHT>(&hamfist_array, 9);
    println!(
        "{:?}",
        Dot::with_config(&trail_graph, &[Config::EdgeNoLabel])
    );

    let mut total_score: usize = 0;
    for idx in trail_graph.node_indices() {
        if trail_graph.node_weight(idx).is_some_and(|w| *w == 0) {
            let n_reachable = n_reachable_nodes_with_weight(&trail_graph, idx, 9);
            total_score += n_reachable;
        }
    }
    println!("Part 1: {}", &total_score);

    total_score = 0;
    for idx in trail_graph.node_indices() {
        if trail_graph.node_weight(idx).is_some_and(|w| *w == 0) {
            let n_reachable = n_paths_to_nodes_with_weight(&trail_graph, idx, 9);
            total_score += n_reachable;
        }
    }
    println!("Part 2: {}", &total_score);
}

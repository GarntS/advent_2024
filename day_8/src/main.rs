/* file:    main.rs
 * author:  garnt
 * date:    12/27/2024
 * desc:    Advent of Code 2024 day 8.
 */

use itertools::Itertools;
use std::collections::HashMap;

// returns true if a, b, and c are colinear
fn are_points_colinear(a: (isize, isize), b: (isize, isize), c: (isize, isize)) -> bool {
    (c.0 - b.0) * (b.1 - a.1) == (b.0 - a.0) * (c.1 - b.1)
}

// returns true if both coordinates are within the range [min, max)
fn point_in_range<T>(coord: (T, T), min: T, max: T) -> bool
where
    T: PartialOrd,
{
    coord.0 >= min && coord.0 < max && coord.1 >= min && coord.1 < max
}

fn main() {
    // metadata
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let width: usize = input.lines().next().unwrap().len();
    let height: usize = input.lines().count();
    println!("width: {} - height: {}", width, height);
    assert_eq!(width, height);

    // search for every tower and make a map of their locations
    let mut name_coord_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for line in input.lines().enumerate() {
        for cur_char in line.1.chars().enumerate() {
            // ignore empty slots
            if cur_char.1 == '.' {
                continue;
            }

            if name_coord_map.contains_key(&cur_char.1) {
                // if there's already an entry in the HashMap, add to it
                name_coord_map
                    .get_mut(&cur_char.1)
                    .unwrap()
                    .push((cur_char.0 as isize, line.0 as isize));
            } else {
                // otherwise, make a new vec and add it to the HashMap
                let new_pair_vec: Vec<(isize, isize)> =
                    vec![(cur_char.0 as isize, line.0 as isize)];
                name_coord_map.insert(cur_char.1, new_pair_vec);
            }
        }
    }

    // generate a list containing all the possible pairings of coordinates of
    // two matching towers
    let mut tower_lines: Vec<(char, [(isize, isize); 2])> = Vec::new();
    for (name, coord_pairs) in name_coord_map.iter() {
        for combo in coord_pairs.iter().combinations(2) {
            let a: [(isize, isize); 2] = combo
                .iter()
                .map(|&pair| (pair.0, pair.1))
                .collect::<Vec<(isize, isize)>>()
                .try_into()
                .unwrap();
            tower_lines.push((*name, a));
        }
    }

    // for each point pair, calculate where its antinodes could exist, and mark
    // the bitflag
    let mut has_antinode: Vec<bool> = [false].repeat(width * height);
    for (_, [point_a, point_b]) in tower_lines.iter() {
        let ab_vec = (point_b.0 - point_a.0, point_b.1 - point_a.1);
        let ba_vec = (point_a.0 - point_b.0, point_a.1 - point_b.1);
        let ab_antinode = (point_b.0 + ab_vec.0, point_b.1 + ab_vec.1);
        let ba_antinode = (point_a.0 + ba_vec.0, point_a.1 + ba_vec.1);
        if point_in_range(ab_antinode, 0, width as isize) {
            let flag_coord: usize = (ab_antinode.1 as usize * width) + ab_antinode.0 as usize;
            *has_antinode.get_mut(flag_coord).unwrap() = true;
        }

        if point_in_range(ba_antinode, 0, width as isize) {
            let flag_coord: usize = (ba_antinode.1 as usize * width) + ba_antinode.0 as usize;
            *has_antinode.get_mut(flag_coord).unwrap() = true;
        }
    }

    println!(
        "Part 1: {}",
        &has_antinode.iter().counts().get(&true).unwrap()
    );

    // for each point that could contain an antinode, check if it does
    let mut n_antinodes: usize = 0;
    for line in input.lines().enumerate() {
        for cur_char in line.1.chars().enumerate() {
            // for each pair of towers, check if the current point is colinear
            for (_, [point_a, point_b]) in tower_lines.iter() {
                if are_points_colinear((cur_char.0 as isize, line.0 as isize), *point_a, *point_b) {
                    // there can only be one antinode at a point, so break after
                    // incrementing the count
                    n_antinodes += 1;
                    break;
                }
            }
        }
    }

    println!("Part 2: {}", n_antinodes);
}

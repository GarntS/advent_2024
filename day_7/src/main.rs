/* file:    main.rs
 * author:  garnt
 * date:    12/27/2024
 * desc:    Advent of Code 2024 day 7.
 */

use indicatif::ProgressIterator;
use malachite_nz::natural::Natural;

fn try_concat<T>(a: &T, b: &T) -> Result<T, <T as std::str::FromStr>::Err>
where
    T: std::str::FromStr,
    T: std::fmt::Display,
{
    let ab_str: String = format!("{}{}", *a, *b);
    ab_str.parse::<T>()
}

fn try_find_arith_path(target: Natural, inputs: &[Natural], cur_val: Option<Natural>) -> bool {
    if inputs.is_empty() {
        return false;
    }

    let cur_val = cur_val.unwrap_or_default();
    if cur_val.clone() + &inputs[0] == target || cur_val.clone() * &inputs[0] == target {
        // found a solve!
        true
    } else if try_find_arith_path(
        target.clone(),
        &inputs[1..inputs.len()],
        Some(cur_val.clone() + &inputs[0]),
    ) {
        // found a solve!
        true
    } else if try_find_arith_path(
        target.clone(),
        &inputs[1..inputs.len()],
        Some(cur_val.clone() * &inputs[0]),
    ) {
        // found a solve!
        true
    } else {
        // if we haven't succeeded, it's scuffed
        false
    }
}

fn try_find_arith_path_with_concat(
    target: &Natural,
    inputs: &[Natural],
    cur_val: Option<Natural>,
) -> bool {
    if inputs.is_empty() {
        return false;
    }

    let cur_val = cur_val.unwrap_or_default();
    if cur_val.clone() + &inputs[0] == *target
        || cur_val.clone() * &inputs[0] == *target
        || try_concat(&cur_val, &inputs[0]).is_ok_and(|val| val == *target)
    {
        // found a solve!
        true
    } else if try_find_arith_path_with_concat(
        target,
        &inputs[1..inputs.len()],
        Some(cur_val.clone() + &inputs[0]),
    ) {
        // found a solve!
        true
    } else if try_find_arith_path_with_concat(
        target,
        &inputs[1..inputs.len()],
        Some(cur_val.clone() * &inputs[0]),
    ) {
        // found a solve!
        true
    } else if try_concat(&cur_val, &inputs[0]).is_ok()
        && try_find_arith_path_with_concat(
            target,
            &inputs[1..inputs.len()],
            Some(try_concat(&cur_val, &inputs[0]).unwrap()),
        )
    {
        // found a solve!
        true
    } else {
        // if we haven't succeeded, it's scuffed
        false
    }
}

fn main() {
    // metadata
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    println!("n_lines: {}", input.lines().count());

    // parse the text into tuples of (usize, Vec<usize>)
    let mut pairs: Vec<(Natural, Vec<Natural>)> = Vec::new();
    for line in input.lines() {
        let result_str = line.split(':').next().unwrap();
        let result: Natural = result_str.parse::<Natural>().unwrap();
        let inputs_str = line.split(':').nth(1).unwrap().trim();
        let inputs_vec: Vec<Natural> = inputs_str
            .split(' ')
            .map(|x| x.parse::<Natural>().unwrap())
            .collect();
        pairs.push((result, inputs_vec));
    }

    // part 1
    let mut workable_sum: Natural = Natural::default();
    for pair in pairs.iter().progress() {
        if try_find_arith_path(pair.0.clone(), pair.1.as_slice(), None) {
            workable_sum += &pair.0;
        }
    }
    println!("Part 1: {}", workable_sum);

    // part 2
    workable_sum = Natural::default();
    for pair in pairs.iter().progress() {
        if try_find_arith_path_with_concat(&pair.0, pair.1.as_slice(), None) {
            workable_sum += &pair.0;
        }
    }
    println!("Part 2: {}", workable_sum);
}

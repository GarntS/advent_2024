/* file:    main.rs
 * author:  garnt
 * date:    12/21/2024
 * desc:    Advent of Code 2024 day 2.
 */

use std::cmp::Ordering;

fn slice_is_sorted<T>(slice: &[T]) -> bool
where
    T: Ord,
{
    slice.windows(2).all(|w| w[0] <= w[1]) || slice.windows(2).all(|w| w[0] >= w[1])
}

fn nums_in_range(a: isize, b: isize) -> bool {
    let abs_diff: usize = a.abs_diff(b);
    abs_diff > 0 && abs_diff < 4
}

fn process_line(line: &str) -> bool {
    let nums_vec: Vec<isize> = line
        .split(' ')
        .map(|num_str| num_str.parse::<isize>().unwrap())
        .collect();

    slice_is_sorted(nums_vec.as_slice())
        && nums_vec
            .windows(2)
            .all(|pair| nums_in_range(pair[0], pair[1]))
}

fn slice_is_sorted_ranged_skip(slice: &[isize], order: Ordering) -> bool {
    fn do_cmp(a: isize, b: isize, order: Ordering) -> bool {
        if order == Ordering::Less {
            return a.cmp(&b).is_le();
        } else {
            return a.cmp(&b).is_ge();
        }
    }

    let mut has_skipped: bool = false;
    let mut skipped_value: Option<isize> = None;

    // check if sorted least-to-greatest
    for pair in slice.windows(2) {
        if let Some(skipped) = skipped_value {
            if do_cmp(skipped, pair[1], order) && nums_in_range(skipped, pair[1]) {
                // lookin good
                skipped_value = None;
            } else {
                return false;
            }
        } else {
            if do_cmp(pair[0], pair[1], order) && nums_in_range(pair[0], pair[1]) {
                // everything lookin good
                continue;
            } else if !has_skipped {
                has_skipped = true;
                skipped_value = Some(pair[0]);
            } else {
                return false;
            }
        }
    }
    return true;
}

fn process_line_with_removal(line: &str) -> bool {
    let nums_vec: Vec<isize> = line
        .split(' ')
        .map(|num_str| num_str.parse::<isize>().unwrap())
        .collect();

    let is_lt = slice_is_sorted_ranged_skip(nums_vec.as_slice(), Ordering::Less);
    let is_gt = slice_is_sorted_ranged_skip(nums_vec.as_slice(), Ordering::Greater);

    println!("{:?} | {} - {}", nums_vec, is_lt, is_gt);

    // return our results
    is_lt || is_gt
}

fn main() {
    // metadata
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    println!("n_lines: {}", input.lines().count());

    // part 1
    let n_safe: usize = input.lines().map(process_line).filter(|b| *b).count();
    println!("part 1: {}", n_safe);

    // part 2
    let n_safe_2: usize = input
        .lines()
        .map(process_line_with_removal)
        .filter(|b| *b)
        .count();
    println!("part 2: {}", n_safe_2);
}

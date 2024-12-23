/* file:    main.rs
 * author:  garnt
 * date:    12/22/2024
 * desc:    Advent of Code 2024 day 4.
 */

#[derive(Clone, Copy, Debug)]
enum DirectionSteps {
    HorizLeft,
    HorizRight,
    VertUp,
    VertDown,
    DiagUL,
    DiagUR,
    DiagDL,
    DiagDR,
}

impl DirectionSteps {
    // returns an (x_step, y_step) pair used for iteration
    fn to_step_pair(&self) -> (isize, isize) {
        match *self {
            DirectionSteps::HorizLeft => (-1, 0),
            DirectionSteps::HorizRight => (0, 0),
            DirectionSteps::VertUp => (0, -1),
            DirectionSteps::VertDown => (0, 1),
            DirectionSteps::DiagUL => (-1, -1),
            DirectionSteps::DiagUR => (1, -1),
            DirectionSteps::DiagDL => (-1, 1),
            DirectionSteps::DiagDR => (1, 1),
        }
    }

    // returns an iterator over all the variants of DirectionSteps
    fn iterate_all() -> impl Iterator<Item = DirectionSteps> {
        [
            DirectionSteps::HorizLeft,
            DirectionSteps::HorizRight,
            DirectionSteps::VertUp,
            DirectionSteps::VertDown,
            DirectionSteps::DiagUL,
            DirectionSteps::DiagUR,
            DirectionSteps::DiagDL,
            DirectionSteps::DiagDR,
        ]
        .iter()
        .copied()
    }
}

// I'm on record that I hate this function
fn hamfist_into_2d_arr(input: &str) -> Result<[[char; 140]; 140], ()> {
    let mut char_arr: [[char; 140]; 140] = [[' '; 140]; 140];
    for (idx, line) in input.lines().enumerate() {
        if line.len() != 140 {
            return Err(());
        }

        let line_arr: [char; 140] = line.chars().collect::<Vec<char>>().try_into().unwrap();
        let _ = std::mem::replace(char_arr.get_mut(idx).ok_or(())?, line_arr);
    }

    // return our newly-constructed char array
    Ok(char_arr)
}

// NOTE: points are 0-based
// NOTE: points provided as (x, y) even though indexes to array are backwards
fn char_at_point(arr: &[[char; 140]; 140], point: (usize, usize)) -> char {
    let x: usize = point.0.min(139);
    let y: usize = point.1.min(139);
    arr[y][x]
}

fn find_radially(arr: &[[char; 140]; 140], point: (usize, usize), pattern: &str) -> usize {
    assert!(pattern.len() > 0);

    let mut n_found: usize = 0;
    'dir_loop: for direction in DirectionSteps::iterate_all() {
        // grab step, check bounds
        let step_pair = direction.to_step_pair();
        let s_len: usize = pattern.len();
        let x_bound: isize = point.0 as isize + (step_pair.0 * (s_len - 1) as isize);
        let y_bound: isize = point.1 as isize + (step_pair.1 * (s_len - 1) as isize);
        if x_bound < 0 || x_bound >= 140 || y_bound < 0 || y_bound >= 140 {
            continue;
        }

        for i in 0..s_len {
            // do signed math then cast back
            let cur_point: (usize, usize) = (
                (point.0 as isize + (step_pair.0 * i as isize) as isize) as usize,
                (point.0 as isize + (step_pair.0 * i as isize) as isize) as usize,
            );

            if char_at_point(arr, cur_point) != pattern.chars().nth(i).unwrap() {
                continue 'dir_loop;
            }
        }

        // if we got here, we found it, so increment
        n_found += 1;
    }

    n_found
}

fn main() {
    // metadata
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "width: {} - height: {}",
        input.lines().next().unwrap().len(),
        input.lines().count(),
    );

    // part 1
    let hamfist_array: [[char; 140]; 140] = hamfist_into_2d_arr(&input).unwrap();
    let mut found_count: usize = 0;
    for start_x in 0..140 {
        for start_y in 0..140 {
            found_count += find_radially(&hamfist_array, (start_x, start_y), "XMAS");
        }
    }
    println!("Part 1: {}", found_count);
}

#[derive(Debug, Default)]
struct ForgivingFoldState<T> {
    has_forgiven: bool,
    prev_relevant: Option<T>,
    count: usize,
}

fn slice_is_sorted<T>(slice: &[T]) -> bool
where
    T: Ord,
{
    slice.windows(2).all(|w| w[0] <= w[1]) || slice.windows(2).all(|w| w[0] >= w[1])
}

fn nums_in_range(n_slice: &[isize]) -> bool {
    let abs_diff: usize = n_slice[1].abs_diff(n_slice[0]);
    abs_diff > 0 && abs_diff < 4
}

fn process_line(line: &str) -> bool {
    let nums_vec: Vec<isize> = line
        .split(' ')
        .map(|num_str| num_str.parse::<isize>().unwrap())
        .collect();

    slice_is_sorted(nums_vec.as_slice()) && nums_vec.windows(2).all(nums_in_range)
}

fn process_line_with_removal(line: &str) -> bool {
    let nums_vec: Vec<isize> = line
        .split(' ')
        .map(|num_str| num_str.parse::<isize>().unwrap())
        .collect();

    let is_sorted: bool = {
        nums_vec
        .as_slice()
        .windows(2)
        .fold(ForgivingFoldState::<isize>::default(), |state, w| {
            if w[0] >= w[1] {
                state.count += 1;
                state.prev_relevant = None;
                return state;
            } else {
                if acc.has_forgiven {
                    return false;
                }
            }
        }).result == 0
    }
    {
        slice.windows(2).all(|w| w[0] <= w[1]slice.windows(2).all(|w| w[0] <= w[1]) || slice.windows(2).all(|w| w[0] >= w[1])) || slice.windows(2).all(|w| w[0] >= w[1])
    };

    slice_is_sorted(nums_vec.as_slice()) && nums_vec.windows(2).all(nums_in_range)
}

fn main() {
    // part 1
    let n_safe: usize = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(process_line)
        .filter(|b| *b)
        .count();
    println!("part 1: {}", n_safe);

    // part 2
    let n_safe_2: usize = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(process_line_with_removal)
        .filter(|b| *b)
        .count();
    println!("part 2: {}", n_safe_2);
}

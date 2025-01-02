/* file:    main.rs
 * author:  garnt
 * date:    01/02/2025
 * desc:    Advent of Code 2024 day 13.
 */

use z3::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Problem {
    target: [usize; 2],
    a_values: [usize; 2],
    b_values: [usize; 2],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ProblemParseState {
    Empty,
    ParsedA,
    ParsedB,
}

/// parses the input into structs
fn parse_problems(input: &str) -> Result<Vec<Problem>, ()> {
    let mut problems: Vec<Problem> = Vec::new();
    let mut parse_state: ProblemParseState = ProblemParseState::Empty;
    let mut last_a: [usize; 2] = [0; 2];
    let mut last_b: [usize; 2] = [0; 2];
    for line in input.lines() {
        match parse_state {
            ProblemParseState::Empty => {
                // skip empty lines between entries
                if line == "" {
                    continue;
                }

                // parse the A values
                assert!(line.contains("Button A:"));
                let x_plus_ofs: usize = line.find("X+").ok_or(())?;
                let comma_ofs: usize = line[x_plus_ofs..line.len()].find(",").ok_or(())?;
                let x_add: usize = line[(x_plus_ofs + 2)..(x_plus_ofs + comma_ofs)]
                    .parse::<usize>()
                    .or(Err(()))?;
                let y_plus_ofs: usize = line.find("Y+").unwrap();
                let y_add: usize = line[(y_plus_ofs + 2)..line.len()]
                    .parse::<usize>()
                    .or(Err(()))?;
                last_a = [x_add, y_add];

                // update state
                parse_state = ProblemParseState::ParsedA;
            }
            ProblemParseState::ParsedA => {
                // parse the B values
                assert!(line.contains("Button B:"));
                let x_plus_ofs: usize = line.find("X+").ok_or(())?;
                let comma_ofs: usize = line[x_plus_ofs..line.len()].find(",").ok_or(())?;
                let x_add: usize = line[(x_plus_ofs + 2)..(x_plus_ofs + comma_ofs)]
                    .parse::<usize>()
                    .or(Err(()))?;
                let y_plus_ofs: usize = line.find("Y+").unwrap();
                let y_add: usize = line[(y_plus_ofs + 2)..line.len()]
                    .parse::<usize>()
                    .or(Err(()))?;
                last_b = [x_add, y_add];

                // update state
                parse_state = ProblemParseState::ParsedB;
            }
            ProblemParseState::ParsedB => {
                // parse the Prize values
                assert!(line.contains("Prize:"));
                let x_eq_ofs: usize = line.find("X=").ok_or(())?;
                let comma_ofs: usize = line[x_eq_ofs..line.len()].find(",").ok_or(())?;
                let x_val: usize = line[(x_eq_ofs + 2)..(x_eq_ofs + comma_ofs)]
                    .parse::<usize>()
                    .or(Err(()))?;
                let y_eq_ofs: usize = line.find("Y=").unwrap();
                let y_val: usize = line[(y_eq_ofs + 2)..line.len()]
                    .parse::<usize>()
                    .or(Err(()))?;

                // add the new problem to the vec
                problems.push(Problem {
                    target: [x_val, y_val],
                    a_values: last_a,
                    b_values: last_b,
                });

                // update state
                parse_state = ProblemParseState::Empty;
            }
        }
    }

    // double check that we haven't had a partial parse
    assert_eq!(parse_state, ProblemParseState::Empty);

    // yield the parsed problems
    Ok(problems)
}

/// uses z3 to try and solve a single problem
fn solve_single_problem(problem: &Problem) -> Option<usize> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let optimize = Optimize::new(&ctx);

    // define our a and b variables, constrained by [0, 100]
    let a = ast::Int::new_const(&ctx, "a");
    let b = ast::Int::new_const(&ctx, "b");
    let a_le_100 = a.le(&ast::Int::from_u64(&ctx, 100));
    let b_le_100 = b.le(&ast::Int::from_u64(&ctx, 100));

    // define constants based on the problem struct
    let a_xstep = ast::Int::from_u64(&ctx, problem.a_values[0] as u64);
    let a_ystep = ast::Int::from_u64(&ctx, problem.a_values[1] as u64);
    let b_xstep = ast::Int::from_u64(&ctx, problem.b_values[0] as u64);
    let b_ystep = ast::Int::from_u64(&ctx, problem.b_values[1] as u64);
    let target_x = ast::Int::from_u64(&ctx, problem.target[0] as u64);
    let target_y = ast::Int::from_u64(&ctx, problem.target[1] as u64);

    // generate variables for x and y based on a and b
    let x = (a_xstep * &a) + (b_xstep * &b);
    let y = (a_ystep * &a) + (b_ystep * &b);

    let x_le = x.le(&target_x);
    let x_not_lt = x.lt(&target_x).not();
    let x_eq = ast::Bool::and(&ctx, &[&x_le, &x_not_lt]);
    let y_le = y.le(&target_y);
    let y_not_lt = y.lt(&target_y).not();
    let y_eq = ast::Bool::and(&ctx, &[&y_le, &y_not_lt]);

    // calculate the number of tokens
    let n_tokens = (&a * ast::Int::from_u64(&ctx, 3)) + &b;

    optimize.assert(&a_le_100);
    optimize.assert(&b_le_100);
    optimize.assert(&x_eq);
    optimize.assert(&y_eq);
    optimize.minimize(&n_tokens);

    // return our result
    if optimize.check(&[]) == SatResult::Sat {
        let model = optimize.get_model().unwrap();
        Some(model.eval(&n_tokens, true).unwrap().as_u64().unwrap() as usize)
    } else {
        None
    }
}

/// uses z3 to try and solve a single problem
fn solve_single_problem_unconstrained(problem: &Problem) -> Option<usize> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let optimize = Optimize::new(&ctx);

    // define our a and b variables
    let a = ast::Int::new_const(&ctx, "a");
    let b = ast::Int::new_const(&ctx, "b");

    // define constants based on the problem struct
    let a_xstep = ast::Int::from_u64(&ctx, problem.a_values[0] as u64);
    let a_ystep = ast::Int::from_u64(&ctx, problem.a_values[1] as u64);
    let b_xstep = ast::Int::from_u64(&ctx, problem.b_values[0] as u64);
    let b_ystep = ast::Int::from_u64(&ctx, problem.b_values[1] as u64);
    let target_x = ast::Int::from_u64(&ctx, problem.target[0] as u64);
    let target_y = ast::Int::from_u64(&ctx, problem.target[1] as u64);

    // generate variables for x and y based on a and b
    let x = (a_xstep * &a) + (b_xstep * &b);
    let y = (a_ystep * &a) + (b_ystep * &b);

    let x_le = x.le(&target_x);
    let x_not_lt = x.lt(&target_x).not();
    let x_eq = ast::Bool::and(&ctx, &[&x_le, &x_not_lt]);
    let y_le = y.le(&target_y);
    let y_not_lt = y.lt(&target_y).not();
    let y_eq = ast::Bool::and(&ctx, &[&y_le, &y_not_lt]);

    // calculate the number of tokens
    let n_tokens = (&a * ast::Int::from_u64(&ctx, 3)) + &b;

    optimize.assert(&x_eq);
    optimize.assert(&y_eq);
    optimize.minimize(&n_tokens);

    // return our result
    if optimize.check(&[]) == SatResult::Sat {
        let model = optimize.get_model().unwrap();
        Some(model.eval(&n_tokens, true).unwrap().as_u64().unwrap() as usize)
    } else {
        None
    }
}

/// the entrypoint
fn main() {
    // parse the input into structs
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let problems: Vec<Problem> = parse_problems(&input).unwrap();

    // metadata
    println!("# of problems: {}", problems.len());

    // part 1
    let total_n_tokens: usize = problems.iter().map(solve_single_problem).flatten().sum();
    println!("Part 1: {}", total_n_tokens);

    // part 2
    let adjusted_problems: Vec<Problem> = problems
        .iter()
        .map(|problem| Problem {
            target: [
                problem.target[0] + 10000000000000,
                problem.target[1] + 10000000000000,
            ],
            a_values: problem.a_values,
            b_values: problem.b_values,
        })
        .collect();
    let total_n_tokens: usize = adjusted_problems
        .iter()
        .map(solve_single_problem_unconstrained)
        .flatten()
        .sum();
    println!("Part 2: {}", total_n_tokens);
}

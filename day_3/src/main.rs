/* file:    main.rs
 * author:  garnt
 * date:    12/21/2024
 * desc:    Advent of Code 2024 day 2.
 */

use regex::Regex;

fn visit_mul_do_dont(acc: (bool, usize), cap_str: &str) -> (bool, usize) {
    let mul_re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
    match cap_str {
        "do()" => (true, acc.1),
        "don't()" => (false, acc.1),
        _ => {
            if acc.0 {
                let mul_caps = mul_re.captures_iter(cap_str).next().unwrap();
                let [a_str, b_str] = mul_caps.extract::<2>().1;
                let product = a_str.parse::<usize>().unwrap() * b_str.parse::<usize>().unwrap();
                (acc.0, acc.1 + product)
            } else {
                acc
            }
        }
    }
}

fn main() {
    // construct mul regex, mul/do/dont regex
    let mul_re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
    let mul_do_dont_re = Regex::new(r"(mul\(\d{1,3}\,\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();

    // metadata
    let input: String = std::fs::read_to_string("input.txt").unwrap();

    // part 1
    let mul_sum: usize = mul_re
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap())
        .sum();
    println!("part one: {:?}", &mul_sum);

    // part 2
    let mul_sum_with_do_dont: usize = mul_do_dont_re
        .captures_iter(&input)
        .map(|c| c.extract().1)
        .fold((true, 0 as usize), |acc, c: [&str; 1]| {
            visit_mul_do_dont(acc, c[0])
        })
        .1;
    println!("part one: {:?}", &mul_sum_with_do_dont);
}

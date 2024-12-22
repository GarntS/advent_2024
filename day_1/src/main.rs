fn iter_cmp(lefts: &Vec<i32>, rights: &Vec<i32>) -> i32 {
    let mut similar: i32 = 0;
    for l_val in lefts {
        similar += l_val * rights.iter().filter(|r_val| *l_val == **r_val).count() as i32;
    }
    return similar;
}

fn main() {
    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: Vec<i32> = Vec::new();
    for line in std::fs::read_to_string("input.txt").unwrap().lines() {
        let mut halves_iter = line.split_whitespace().take(2);
        lefts.push(halves_iter.next().unwrap().parse::<i32>().unwrap());
        rights.push(halves_iter.next().unwrap().parse::<i32>().unwrap());
    }
    lefts.sort();
    rights.sort();

    let mut pairs: Vec<(i32, i32)> = Vec::new();
    pairs = lefts
        .iter()
        .zip(rights.iter())
        .map(|refs| (refs.0.to_owned(), refs.1.to_owned()))
        .collect();

    println!(
        "pt 1: {:?}",
        pairs
            .iter()
            .map(|tuple| (tuple.0 - tuple.1).abs())
            .sum::<i32>()
    );

    println!("pt 2: {}", iter_cmp(&lefts, &rights));
}

use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");

    // Part 1
    let counts = input.split("\n\n").map(|group| -> usize {
        group
            .split("\n")
            .map(|number| number.parse().unwrap_or(0))
            .sum()
    });

    let mut c = counts.collect::<Vec<_>>();

    c.sort();
    let last_idx = c.len() - 1;

    println!("highest count is: {}", c[last_idx]);

    // Part 2

    let rev_c: usize = c.iter().rev().take(3).sum();
    println!("sum count is: {}", rev_c);
}

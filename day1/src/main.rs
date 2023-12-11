use std::fs;
fn main() {
    println!(
        "{}",
        fs::read_to_string("../../inputs/aoc_1_input.txt")
            .unwrap()
            .split("\n")
            .filter_map(|e| if e.len() > 0 {
                Some(e.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>())
            } else {
                None
            })
            .map(|c| {
                format!("{}{}", c.first().unwrap(), c.last().unwrap())
                    .parse::<i32>()
                    .unwrap()
            })
            .reduce(|a, b| a + b)
            .unwrap()
    )
}

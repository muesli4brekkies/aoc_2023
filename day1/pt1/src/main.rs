use std::fs;

fn main() {
    let result = fs::read_to_string("/home/muesli/git/aoc_2023/input.txt")
        .unwrap()
        .split("\n")
        .filter_map(|e| {
            if !e.is_empty() {
                Some(e.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>())
            } else {
                None
            }
        })
        .map(|c| {
            format!("{}{}", c.first().unwrap(), c.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .reduce(|a.b|a+b)
        .unwrap();
    println!("{result}")
}

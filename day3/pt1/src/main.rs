use std::fs;

fn main() {
    let file_str = fs::read_to_string("/home/muesli/git/aoc_2023/day3/input.txt").unwrap();
    let n_lines = file_str.lines().collect::<Vec<_>>().len();
    let line_len = file_str.lines().collect::<Vec<_>>()[0].len();
    let result: Vec<_> = file_str.lines().enumerate().collect();
    println!("{result:?}");
    println!("{n_lines} {line_len}");
}

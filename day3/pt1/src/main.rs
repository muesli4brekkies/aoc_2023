use std::fs;
// line width = 140 chars
fn main() {
    let file_string: String =
        fs::read_to_string("/home/muesli/git/aoc_2023/day3/input.txt").unwrap();
    let file_string_vec = file_string.lines().collect::<Vec<&str>>();
    let width = file_string_vec[0].len() as i32 + 1;
    let dummy_line: &str = &".".repeat(width as usize + 1);
    let result = file_string
        .lines()
        .enumerate()
        .map(|line| {
            line.1
                .split(char::is_numeric)
                .map(|n| {
                    let last_line = file_string_vec
                        .iter()
                        .nth((line.0 as i32 - 1) as usize)
                        .unwrap_or(&dummy_line);
                    let next_line = file_string_vec
                        .iter()
                        .nth(line.0 + 1)
                        .unwrap_or(&dummy_line);
                    let ind = line.1.find(n).unwrap();
                    let previous = &last_line[ind - 1..ind];
                    let next = &next_line[ind + n.len()..ind + n.len() + 1];
                    println!("{line:?}{ind} {previous} {next}");
                    n
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("{result:?}");
}

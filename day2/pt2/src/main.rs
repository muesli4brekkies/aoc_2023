use std::fs;
fn findmin(col: &str, strings: &[&str]) -> i32 {
    strings
        .iter()
        .filter(|string| string.split(' ').collect::<Vec<&str>>()[1] == col)
        .fold(0, |a, string| {
            let b = string.split(' ').collect::<Vec<&str>>()[0]
                .parse::<i32>()
                .unwrap();
            if b < a {
                a
            } else {
                b
            }
        })
}

fn main() {
    let file_string = fs::read_to_string("/home/muesli/git/aoc_2023/day2/input.txt").unwrap();
    let result: i32 = file_string
        .lines()
        .map(|e| {
            e.split(|c| c == ';' || c == ':' || c == ',')
                .map(str::trim)
                .collect::<Vec<&str>>()
        })
        .fold(0, |a, strings| {
            a + ["red", "green", "blue"]
                .iter()
                .fold(1, |a, col| a * findmin(col, &strings))
        });

    println!("{}", result)
}

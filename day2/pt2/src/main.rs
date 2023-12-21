use std::fs;
fn findmin(col: &str, strings: &Vec<&str>) -> i32 {
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
    let COLOURS: Vec<&str> = vec!["red", "green", "blue"];
    let file_string = fs::read_to_string("/home/muesli/git/aoc_2023/day2/input.txt").unwrap();
    let result: i32 = file_string
        .lines()
        .map(|e| {
            e.split(|c| c == ';' || c == ':' || c == ',')
                .map(str::trim)
                .collect::<Vec<_>>()
        })
        .map(|strings| {
            COLOURS
                .iter()
                .map(|col| findmin(col, &strings))
                .reduce(|a, b| a * b)
                .unwrap()
        })
        .reduce(|a, b| a + b)
        .unwrap();

    println!("{}", result)
}

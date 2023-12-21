use std::fs;
struct Cols {
    red: i32,
    green: i32,
    blue: i32,
}
const LIMS: Cols = Cols {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let result = fs::read_to_string("/home/muesli/git/aoc_2023/day2/input.txt")
        .unwrap()
        .lines()
        .map(|e| {
            e.split(|c| c == ';' || c == ':' || c == ',')
                .map(str::trim)
                .collect::<Vec<_>>()
        })
        .filter(|strings| {
            strings.iter().all(|string| {
                let words: Vec<&str> = string.split(' ').collect();
                let (n, colour) = (words[0].parse::<i32>().unwrap_or(0), words[1]);
                match colour {
                    "red" => n <= LIMS.red,
                    "blue" => n <= LIMS.blue,
                    "green" => n <= LIMS.green,
                    _ => true,
                }
            })
        })
        .map(|c| {
            c[0].split(' ').collect::<Vec<_>>()[1]
                .parse::<i32>()
                .unwrap()
        })
        .reduce(|a, b| a + b)
        .unwrap();

    println!("{}", result)
}

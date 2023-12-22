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
    let result: i32 = fs::read_to_string("/home/muesli/git/aoc_2023/day2/input.txt")
        .unwrap()
        .lines()
        .filter_map(|line| {
            if line
                .split(|c| c == ';' || c == ':' || c == ',')
                .map(str::trim)
                .all(|string| {
                    let words: Vec<&str> = string.split(' ').collect();
                    let (n, colour) = (words[0].parse::<i32>().unwrap_or(0), words[1]);
                    match colour {
                        "red" => n <= LIMS.red,
                        "blue" => n <= LIMS.blue,
                        "green" => n <= LIMS.green,
                        _ => true,
                    }
                })
            {
                Some(
                    line.split(' ').collect::<Vec<&str>>()[1]
                        .replace(':', "")
                        .parse::<i32>()
                        .unwrap(),
                )
            } else {
                None
            }
        })
        .sum();

    println!("{}", result)
}

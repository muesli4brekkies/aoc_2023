use std::fs;

fn main() {
    let result = fs::read_to_string("/home/muesli/git/aoc_2023/day2/input.txt")
        .unwrap()
        .lines()
        .fold(0, |a, strings| {
            a + ["red", "green", "blue"].into_iter().fold(1, |a, col| {
                a * strings
                    .split(|c| c == ';' || c == ':' || c == ',')
                    .map(str::trim)
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
            })
        });

    println!("{}", result)
}

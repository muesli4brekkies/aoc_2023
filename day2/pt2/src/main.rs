use std::fs;
fn main() {
    println!(
        "{}",
        fs::read_to_string("/home/muesli/git/aoc_2023/day2/input.txt")
            .unwrap()
            .lines()
            .fold(0, |a, strings| a + ["red", "green", "blue"]
                .into_iter()
                .fold(1, |a, col| a * strings
                    .split(|c| c == ';' || c == ':' || c == ',')
                    .fold(0, |a, string| {
                        let n = string[0..3].trim().parse::<i32>().unwrap_or(0);
                        if n < a || !string.ends_with(col) {
                            a
                        } else {
                            n
                        }
                    })))
    )
}

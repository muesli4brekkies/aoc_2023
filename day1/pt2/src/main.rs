use std::fs;

fn replacer(input: &str) -> String {
    input
        .replace("one", "1ne")
        .replace("two", "2wo")
        .replace("three", "3hree")
        .replace("four", "4our")
        .replace("five", "5ive")
        .replace("six", "6ix")
        .replace("seven", "7even")
        .replace("eight", "8ight")
        .replace("nine", "9ine")
}

fn words_to_nums(mut input: String) -> String {
    println!("in - {}", input);
    for i in 0..(input.len() + 1) {
        let chunk = &input[0..i];
        let remainder = &input[i..];
        input = format!("{}{}", replacer(chunk), remainder);
    }
    println!("out - {}", input);
    input
}

fn strip_nums(input: &str) -> Option<Vec<char>> {
    if input.is_empty() {
        None
    } else {
        Some(
            words_to_nums(input.to_string())
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<_>>(),
        )
    }
}

fn format_n_parse(input: Vec<char>) -> i32 {
    format!("{}{}", input.first().unwrap(), input.last().unwrap())
        .parse::<i32>()
        .unwrap()
}

fn main() {
    let result = fs::read_to_string("/home/muesli/git/aoc_2023/input.txt")
        .unwrap()
        .split("\n")
        .filter_map(strip_nums)
        .map(format_n_parse)
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{result}")
}

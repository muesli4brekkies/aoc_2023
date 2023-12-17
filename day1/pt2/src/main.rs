use std::fs;

fn words_to_digits(input:&str) -> &str {
	input
}

fn strip_numerics(input:&str) ->Vec<char> {
               input.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>()
}

fn get_firstlast_and_parse(input:Vec<char>) -> i32 {
	format!("{}{}", input.first().unwrap(), input.last().unwrap())
	    .parse::<i32>()
	    .unwrap()
}

fn main() {
        let result = fs::read_to_string("/home/muesli/git/aoc_2023/input.txt")
            .unwrap()
            .split("\n")
            .filter(|e|!e.is_empty())
	    .map(words_to_digits)
	    .map(strip_numerics)
            .map(get_firstlast_and_parse)
            .reduce(|a,b|a+b)
            .unwrap();
	println!("{result}")
}

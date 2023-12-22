use std::fs;

fn main() {
    let file_str = fs::read_to_string("/home/muesli/git/aoc_2023/day3/input.txt").unwrap();
    let file_lines = file_str.lines().collect::<Vec<&str>>();
    let result = file_str
        .lines()
        .filter_map(|l| {
            let q = l.char_indices().filter(|e| e.1 == '*').collect::<Vec<_>>();
            if !q.is_empty() {
                Some(q)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!("{result:?}")
}

use std::fs;

fn main() {
    let file_str = fs::read_to_string("/home/muesli/git/aoc_2023/day3/input.txt").unwrap();
    let file_lines = file_str.lines().collect::<Vec<&str>>();
    let line_width = file_lines[0].len();
    let result: i32 = file_str.lines().enumerate().fold(0, |a, line| {
        a + {
            let mut i = 0;
            line.1
                .char_indices()
                .filter_map(move |c| {
                    if i > c.0 {
                        None
                    } else if !c.1.is_numeric() {
                        i += 1;
                        None
                    } else {
                        i += 3;
                        Some((
                            c.0,
                            line.1[c.0..if c.0 + 3 > line_width {
                                line_width
                            } else {
                                c.0 + 3
                            }]
                                .chars()
                                .filter(|c| c.is_numeric())
                                .collect::<String>(),
                        ))
                    }
                })
                .fold(0, |a, n| {
                    a + {
                        let range = if n.0 == 0 { 0 } else { n.0 - 1 }
                            ..if n.0 + n.1.len() < line_width {
                                n.0 + n.1.len() + 1
                            } else {
                                line_width
                            };
                        if format!(
                            "{}{}{}",
                            &line.1[range.clone()],
                            &file_lines[if line.0 == 0 { 0 } else { line.0 - 1 }][range.clone()],
                            &match file_lines.get(line.0 + 1) {
                                Some(r) => r,
                                None => line.1,
                            }[range.clone()],
                        )
                        .contains(|c: char| c != '.' && !c.is_numeric())
                        {
                            n.1.parse::<i32>().unwrap()
                        } else {
                            0
                        }
                    }
                })
        }
    });
    println!("{result}");
}

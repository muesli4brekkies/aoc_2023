use std::fs;

fn main() {
    let file_str = fs::read_to_string("/home/muesli/git/aoc_2023/day3/input.txt").unwrap();
    let file_lines = file_str.lines().collect::<Vec<&str>>();
    let line_width = file_str.lines().collect::<Vec<_>>()[0].len();
    let result: i32 = file_str.lines().enumerate().fold(0, |a, line| {
        a + {
            let mut i = 0;
            line.1
                .char_indices()
                .filter_map(move |c| {
                    if i <= c.0 {
                        if c.1.is_numeric() {
                            let j = i;
                            i += 3;
                            Some((
                                j,
                                line.1[j..if j + 3 > line_width {
                                    line_width
                                } else {
                                    j + 3
                                }]
                                    .chars()
                                    .filter(|c| c.is_numeric())
                                    .collect::<String>(),
                            ))
                        } else {
                            i += 1;
                            None
                        }
                    } else {
                        None
                    }
                })
                .fold(0, |a, n| {
                    a + {
                        let range = if n.0 as i32 - 1 > 0 { n.0 - 1 } else { 0 }
                            ..if n.0 + n.1.len() < line_width {
                                n.0 + n.1.len() + 1
                            } else {
                                line_width
                            };
                        if format!(
                            "{}{}{}",
                            &line.1[range.clone()],
                            &match file_lines.get((line.0 as i32 - 1) as usize) {
                                Some(r) => r.to_string(),
                                None => ".".repeat(line_width),
                            }[range.clone()],
                            &match file_lines.get(line.0 + 1) {
                                Some(r) => r.to_string(),
                                None => ".".repeat(line_width),
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

use std::collections::HashMap;

fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn is_symbol(ch: char) -> bool {
    !(ch.is_numeric() || ch == '.')
}

fn pad_lines(content: String) -> Vec<String> {
    let mut padded_lines = Vec::new();
    for line in content.lines() {
        let mut padded_line = line.to_string();
        padded_line.push('.');
        padded_lines.push(padded_line);
    }
    padded_lines
}

fn part1() {
    let content = open_input("bin/day3/input.txt");
    let lines = pad_lines(content);
    let mut sum_parts = 0;
    for (r, line) in lines.iter().enumerate() {
        let mut num_as_str = String::new();
        let mut idxs = Vec::new();
        for (c, char) in line.chars().enumerate() {
            if char.is_numeric() {
                num_as_str.push(char);
                idxs.push(c);
            } else if num_as_str.len() > 0 {
                // now check
                let num = num_as_str
                    .parse::<u32>()
                    .expect("Could not turn string into number");
                let start_idx = idxs[0];
                let end_idx = idxs[idxs.len() - 1];
                let mut should_add = false;
                if start_idx > 0 {
                    // check on the left
                    if is_symbol(line.chars().collect::<Vec<char>>()[start_idx - 1]) {
                        should_add = true;
                    }
                }
                if end_idx < line.len() - 1 {
                    // check on the right
                    if is_symbol(line.chars().collect::<Vec<char>>()[end_idx + 1]) {
                        should_add = true;
                    }
                }
                if r > 0 {
                    // check above
                    if lines[r - 1][start_idx..=end_idx]
                        .chars()
                        .into_iter()
                        .filter(|c| is_symbol(*c))
                        .collect::<Vec<char>>()
                        .len()
                        > 0
                    {
                        should_add = true;
                    }
                    // check above on left
                    if start_idx > 0 {
                        if is_symbol(lines[r - 1].chars().nth(start_idx - 1).unwrap()) {
                            should_add = true;
                        }
                    }
                    // check above on right
                    if end_idx < line.len() - 1 {
                        if is_symbol(lines[r - 1].chars().nth(end_idx + 1).unwrap()) {
                            should_add = true;
                        }
                    }
                }
                if r < lines.len() - 1 {
                    // check below
                    if lines[r + 1][start_idx..=end_idx]
                        .chars()
                        .into_iter()
                        .filter(|c| is_symbol(*c))
                        .collect::<Vec<char>>()
                        .len()
                        > 0
                    {
                        should_add = true;
                    }
                    // check below on left
                    if start_idx > 0 {
                        if is_symbol(lines[r + 1].chars().nth(start_idx - 1).unwrap()) {
                            should_add = true;
                        }
                    }
                    // check below on right
                    if end_idx < line.len() - 1 {
                        if is_symbol(lines[r + 1].chars().nth(end_idx + 1).unwrap()) {
                            should_add = true;
                        }
                    }
                }
                // add number
                if should_add {
                    sum_parts += num;
                }

                // clear both accumulators
                num_as_str.clear();
                idxs.clear();
            }
        }
    }
    println!("{sum_parts}");
}

fn is_gear(ch: char) -> bool {
    ch == '*'
}

fn part2() {
    let content = open_input("bin/day3/input.txt");
    let lines = pad_lines(content);
    let mut gear_to_number_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (r, line) in lines.iter().enumerate() {
        let mut num_as_str = String::new();
        let mut idxs = Vec::new();
        for (c, char) in line.chars().enumerate() {
            if char.is_numeric() {
                num_as_str.push(char);
                idxs.push(c);
            } else if num_as_str.len() > 0 {
                // now check
                let num = num_as_str
                    .parse::<u32>()
                    .expect("Could not turn string into number");
                let start_idx = idxs[0];
                let end_idx = idxs[idxs.len() - 1];
                let mut idx_to_add = None;
                if start_idx > 0 {
                    // check on the left
                    if is_gear(line.chars().collect::<Vec<char>>()[start_idx - 1]) {
                        idx_to_add = Some((r, start_idx - 1));
                    }
                }
                if end_idx < line.len() - 1 {
                    // check on the right
                    if is_gear(line.chars().collect::<Vec<char>>()[end_idx + 1]) {
                        idx_to_add = Some((r, end_idx + 1));
                    }
                }
                if r > 0 {
                    // check above
                    let _ = lines[r - 1][start_idx..=end_idx]
                        .chars()
                        .into_iter()
                        .enumerate()
                        .for_each(|(idx, c)| {
                            if is_gear(c) {
                                idx_to_add = Some((r - 1, start_idx + idx));
                            }
                        });
                    // check above on left
                    if start_idx > 0 {
                        if is_gear(lines[r - 1].chars().nth(start_idx - 1).unwrap()) {
                            idx_to_add = Some((r - 1, start_idx - 1));
                        }
                    }
                    // check above on right
                    if end_idx < line.len() - 1 {
                        if is_gear(lines[r - 1].chars().nth(end_idx + 1).unwrap()) {
                            idx_to_add = Some((r - 1, end_idx + 1));
                        }
                    }
                }
                if r < lines.len() - 1 {
                    // check below
                    let _ = lines[r + 1][start_idx..=end_idx]
                        .chars()
                        .into_iter()
                        .enumerate()
                        .for_each(|(idx, c)| {
                            if is_gear(c) {
                                idx_to_add = Some((r + 1, start_idx + idx));
                            }
                        });
                    // check below on left
                    if start_idx > 0 {
                        if is_gear(lines[r + 1].chars().nth(start_idx - 1).unwrap()) {
                            idx_to_add = Some((r + 1, start_idx - 1));
                        }
                    }
                    // check below on right
                    if end_idx < line.len() - 1 {
                        if is_gear(lines[r + 1].chars().nth(end_idx + 1).unwrap()) {
                            idx_to_add = Some((r + 1, end_idx + 1));
                        }
                    }
                }
                match idx_to_add {
                    None => (),
                    Some(k) => match gear_to_number_map.remove(&k) {
                        None => {
                            gear_to_number_map.insert(k, vec![num]);
                        }
                        Some(mut v) => {
                            v.push(num);
                            gear_to_number_map.insert(k, v);
                        }
                    },
                }

                // clear both accumulators
                num_as_str.clear();
                idxs.clear();
            }
        }
    }
    let sum_gear_ratios = gear_to_number_map.into_iter().fold(0, |acc, (_, val)| {
        if val.len() == 2 {
            return acc + val.into_iter().product::<u32>();
        }
        return acc;
    });
    println!("{sum_gear_ratios}");
}

fn main() {
    part1();
    part2();
}

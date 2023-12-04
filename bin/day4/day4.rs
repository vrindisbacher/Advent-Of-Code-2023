use std::collections::HashMap;

fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn pad_lines(content: String) -> Vec<String> {
    let mut padded_lines = Vec::new();
    for line in content.lines() {
        let mut padded_line = line.to_string();
        padded_line.push(' ');
        padded_lines.push(padded_line);
    }
    padded_lines
}

fn part1() {
    let content = open_input("bin/day4/input.txt");
    let mut sum_pts = 0;
    for line in pad_lines(content) {
        let mut curr_num_as_str = String::new();
        let mut past_pipe = false;
        let mut winning_numbers = Vec::new();
        let mut curr_score = 0;
        for ch in line.split(":").collect::<Vec<&str>>()[1].chars() {
            if ch.is_numeric() {
                curr_num_as_str.push(ch);
            } else {
                if ch == '|' {
                    past_pipe = true;
                }
                if curr_num_as_str.len() > 0 {
                    let num = curr_num_as_str
                        .parse::<u32>()
                        .expect("Could not parse string to int");

                    if past_pipe {
                        if winning_numbers.contains(&num) {
                            if curr_score > 0 {
                                // score doubles for the rest
                                curr_score *= 2;
                            } else {
                                // 1 for the first
                                curr_score = 1;
                            }
                        }
                    } else {
                        winning_numbers.push(num);
                    }

                    // clear acc
                    curr_num_as_str.clear();
                }
            }
        }
        sum_pts += curr_score;
    }
    println!("{sum_pts}");
}

fn get_line_idx(line: &str) -> usize {
    let split = line.split(":").collect::<Vec<&str>>()[0]
        .split(" ")
        .collect::<Vec<&str>>();
    let card_idx = split[split.len() - 1]
        .parse::<usize>()
        .expect("Could not parse card # into int");
    card_idx
}

fn get_matches(content: String) -> HashMap<usize, u32> {
    let mut res = HashMap::new();
    for line in pad_lines(content) {
        let card_idx = get_line_idx(line.as_str());
        let mut curr_num_as_str = String::new();
        let mut past_pipe = false;
        let mut winning_numbers = Vec::new();
        let mut num_copy_cards = 0;
        for ch in line.split(":").collect::<Vec<&str>>()[1].chars() {
            if ch.is_numeric() {
                curr_num_as_str.push(ch);
            } else {
                if ch == '|' {
                    past_pipe = true;
                }
                if curr_num_as_str.len() > 0 {
                    let num = curr_num_as_str
                        .parse::<u32>()
                        .expect("Could not parse string to int");

                    if past_pipe {
                        if winning_numbers.contains(&num) {
                            // copy cards
                            num_copy_cards += 1
                        }
                    } else {
                        winning_numbers.push(num);
                    }

                    // clear acc
                    curr_num_as_str.clear();
                }
            }
        }
        res.insert(card_idx, num_copy_cards);
    }
    res
}

fn part2() {
    let content = open_input("bin/day4/input.txt");
    let matches = get_matches(content);
    let mut card_idxs = matches.keys().cloned().collect::<Vec<usize>>();
    let mut idx = 0;
    while idx < card_idxs.len() {
        let card_idx = card_idxs[idx];
        let num_matches = matches
            .get(&card_idx)
            .expect("Didn't find match for card idx");
        for copy_card_idx in (card_idx + 1)..(card_idx + 1 + *num_matches as usize) {
            card_idxs.push(copy_card_idx);
        }
        idx += 1;
    }
    println!("{}", card_idxs.len());
}

fn main() {
    part1();
    part2();
}

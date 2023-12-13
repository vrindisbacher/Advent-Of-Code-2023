use rayon::prelude::*;

fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn one_if_is_match_else_zero(vec_string: &mut Vec<char>, counts: &Vec<usize>) -> u32 {
    let mut acc = Vec::new();
    let mut new_counts = Vec::new();
    vec_string.push('.');
    for ch in vec_string {
        if *ch == '#' {
            acc.push(ch);
        } else if acc.len() > 0 {
            new_counts.push(acc.len());
            acc.clear();
        }
    }
    if counts == &new_counts {
        1
    } else {
        0
    }
}

fn get_all_strings<'a>(
    string: &'a mut Vec<char>,
    idx: usize,
    counts: &Vec<usize>,
    mut acc: u32,
) -> u32 {
    if idx == string.len() {
        return acc + one_if_is_match_else_zero(string, counts);
    }
    if string[idx] == '?' {
        string[idx] = '.';
        acc = get_all_strings(string, idx + 1, counts, acc);

        string[idx] = '#';
        acc = get_all_strings(string, idx + 1, counts, acc);

        string[idx] = '?';
    } else {
        acc = get_all_strings(string, idx + 1, counts, acc);
    }
    return acc;
}

fn unfold_string(x: &str) -> (Vec<char>, Vec<usize>) {
    let vec_string = x
        .split_whitespace()
        .nth(0)
        .expect("No first")
        .chars()
        .collect::<Vec<char>>();
    let mut total_str = vec_string.clone();
    let mut copy = vec_string.clone();
    copy.insert(0, '?');
    for _ in 0..5 {
        let mut clone = copy.clone();
        total_str.append(&mut clone);
    }
    let combs = x
        .split_whitespace()
        .nth(1)
        .expect("No second")
        .split(",")
        .map(|x| x.parse().expect("Not a number"))
        .collect::<Vec<usize>>();
    let mut total_combs = combs.clone();
    let copy = combs.clone();
    for _ in 0..5 {
        let mut clone = copy.clone();
        total_combs.append(&mut clone);
    }
    (total_str, total_combs)
}

fn part2() {
    let content = open_input("bin/day12/input.txt");
    let input = content
        .lines()
        .map(|x| unfold_string(x))
        .collect::<Vec<(Vec<char>, Vec<usize>)>>();
    let sum = input
        .into_par_iter()
        .fold(
            || 0,
            |acc, (mut string, combs)| acc + get_all_strings(&mut string, 0, &combs, 0),
        )
        .sum::<u32>();
    println!("{sum}");
}

fn part1() {
    let content = open_input("bin/day12/input.txt");
    let input = content
        .lines()
        .map(|x| {
            (
                x.split_whitespace()
                    .nth(0)
                    .expect("No first")
                    .chars()
                    .collect(),
                x.split_whitespace()
                    .nth(1)
                    .expect("No Second")
                    .split(",")
                    .map(|x| x.parse().expect("Not a number"))
                    .collect(),
            )
        })
        .collect::<Vec<(Vec<char>, Vec<usize>)>>();

    let sum = input
        .into_par_iter()
        .fold(
            || 0,
            |acc, (mut string, combs)| acc + get_all_strings(&mut string, 0, &combs, 0),
        )
        .sum::<u32>();
    println!("{sum}");
}

fn main() {
    part1();
    part2();
}

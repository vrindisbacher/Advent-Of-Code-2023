use std::collections::HashSet;

fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn part2() {
    let content = open_input("bin/day9/input.txt");
    let histories: Vec<Vec<i32>> = content
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Not a number"))
                .collect()
        })
        .collect();
    let mut sum = 0;
    for history in histories {
        let mut sequences = vec![history];
        while sequences[sequences.len() - 1]
            .iter()
            .collect::<HashSet<&i32>>()
            .len()
            != 1
        {
            let next = &sequences[sequences.len() - 1];
            let diffs: Vec<i32> = (1..next.len())
                .map(|idx| next[idx] - next[idx - 1])
                .collect();
            sequences.push(diffs);
        }
        let mut idx = sequences.len() - 1;
        let next = &sequences[sequences.len() - 1];
        let mut num = next[next.len() - 1];
        while idx > 0 {
            let next_up = &sequences[idx - 1];
            num = next_up[0] - num;
            idx -= 1;
        }
        sum += num;
    }
    println!("{sum}");
}

fn part1() {
    let content = open_input("bin/day9/input.txt");
    let histories: Vec<Vec<i32>> = content
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Not a number"))
                .collect()
        })
        .collect();
    let mut sum = 0;
    for history in histories {
        let mut sequences = vec![history];
        while sequences[sequences.len() - 1]
            .iter()
            .collect::<HashSet<&i32>>()
            .len()
            != 1
        {
            let next = &sequences[sequences.len() - 1];
            let diffs: Vec<i32> = (1..next.len())
                .map(|idx| next[idx] - next[idx - 1])
                .collect();
            sequences.push(diffs);
        }
        let mut idx = sequences.len() - 1;
        let next = &sequences[sequences.len() - 1];
        let mut num = next[next.len() - 1];
        while idx > 0 {
            let next_up = &sequences[idx - 1];
            num += next_up[next_up.len() - 1];
            idx -= 1;
        }
        sum += num;
    }
    println!("{sum}");
}

fn main() {
    part1();
    part2();
}

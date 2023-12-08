use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug)]
struct Direction {
    left: String,
    right: String,
}

fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn get_steps(content: &String) -> Vec<char> {
    content
        .lines()
        .nth(0)
        .expect("no first line")
        .trim_end()
        .trim_start()
        .chars()
        .collect::<Vec<char>>()
}

fn get_graph(content: &String) -> HashMap<String, Direction> {
    let mut direction_map = HashMap::new();
    let graph_lines = &content.lines().collect::<Vec<&str>>()[2..];
    for line in graph_lines {
        let source = line.split(" = ").nth(0).expect("no key").to_string();
        let dest = line.split(" = ").nth(1).expect("no dest");
        let left = dest
            .split(", ")
            .nth(0)
            .expect("No lhs")
            .split("(")
            .nth(1)
            .expect("No left paren")
            .to_string();
        let right = dest
            .split(", ")
            .nth(1)
            .expect("No rhs")
            .split(")")
            .nth(0)
            .expect("No right paren")
            .to_string();
        direction_map.insert(source, Direction { left, right });
    }
    direction_map
}

fn part2() {
    let content = open_input("bin/day8/input.txt");
    let steps = get_steps(&content);
    let graph = get_graph(&content);
    let nodes: Vec<&String> = graph
        .keys()
        .filter(|x| x.chars().nth(x.len() - 1).unwrap() == 'A')
        .collect();
    let mut fastest_way_to_final = Vec::new();
    for node in nodes {
        let mut num_steps : u64 = 0;
        let mut instr_ptr = 0;
        let mut loc = node;
        while loc.chars().nth(loc.len() - 1).unwrap() != 'Z' {
            if instr_ptr > steps.len() - 1 {
                instr_ptr = 0
            }
            let next_instr = steps[instr_ptr];
            match next_instr {
                'L' => {
                    loc = &graph.get(loc).expect("No mapping for location").left;
                }
                'R' => {
                    loc = &graph.get(loc).expect("No mapping for location").right;
                }
                _ => panic!(),
            }
            num_steps += 1;
            instr_ptr += 1;
        }
        fastest_way_to_final.push(num_steps);
    }
    let res = fastest_way_to_final
        .into_iter()
        .fold(1, |acc, x| lcm(acc, x));
    println!("{res}");
}

fn part1() {
    let content = open_input("bin/day8/input.txt");
    let steps = get_steps(&content);
    let graph = get_graph(&content);
    let mut loc = "AAA";
    let mut instr_ptr = 0;
    let mut num_steps = 0;
    while loc != "ZZZ" {
        if instr_ptr > steps.len() - 1 {
            instr_ptr = 0
        }
        let next_instr = steps[instr_ptr];
        match next_instr {
            'L' => {
                loc = graph
                    .get(loc)
                    .expect("No mapping for location")
                    .left
                    .as_str()
            }
            'R' => {
                loc = graph
                    .get(loc)
                    .expect("No mapping for location")
                    .right
                    .as_str()
            }
            _ => panic!(),
        }
        num_steps += 1;
        instr_ptr += 1;
    }
    println!("{num_steps}");
}

fn main() {
    part1();
    part2();
}

fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn part1() {
    let input = open_input("bin/day1/input1.txt");
    let mut nums = Vec::new();
    for line in input.lines() {
        let mut acc = String::new();
        let mut last = None;
        for char in line.chars() {
            if char.is_numeric() {
                if acc.len() == 0 {
                    acc.push(char);
                } else {
                    last = Some(char);
                }
            }
        }
        match last {
            None => acc += &acc.clone(),
            Some(ch) => acc.push(ch),
        };
        let int = acc
            .parse::<i32>()
            .expect("Could not parse string into a valid int");
        nums.push(int);
    }
    let res = nums.iter().fold(0, |acc, i| acc + i);
    println!("{res}");
}

fn line_to_numbers(line: &str) -> String {
    line.to_string()
        .replace("zero", "zero0zero")
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

fn part2() {
    let input = open_input("bin/day1/input2.txt");
    let mut nums = Vec::new();
    for line in input.lines() {
        let mut acc = String::new();
        let mut last = None;
        for char in line_to_numbers(line).chars() {
            if char.is_numeric() {
                if acc.len() == 0 {
                    acc.push(char);
                } else {
                    last = Some(char);
                }
            }
        }
        match last {
            None => acc += &acc.clone(),
            Some(ch) => acc.push(ch),
        };
        let int = acc
            .parse::<i32>()
            .expect("Could not parse string into a valid int");
        nums.push(int);
    }
    let res = nums.iter().fold(0, |acc, i| acc + i);
    println!("{res}");
}

fn main() {
    part1();
    part2();
}

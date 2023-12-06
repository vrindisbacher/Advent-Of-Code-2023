fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn get_races(content: &String) -> Vec<(u32, u32)> {
    let times = content
        .lines()
        .nth(0)
        .expect("No first line")
        .split(":")
        .nth(1)
        .expect("Nothing after :")
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse::<u32>().expect("Not a number!"))
        .collect::<Vec<u32>>();
    let distances = content
        .lines()
        .nth(1)
        .expect("No second line")
        .split(":")
        .nth(1)
        .expect("Nothing after :")
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse::<u32>().expect("Not a number!"))
        .collect::<Vec<u32>>();
    times.into_iter().zip(distances).collect()
}

fn get_race_joined(content: &String) -> (u64, u64) {
    let time = content
        .lines()
        .nth(0)
        .expect("No first line")
        .split(":")
        .nth(1)
        .expect("Nothing after :")
        .replace(" ", "")
        .parse::<u64>()
        .expect("Line was not a number");

    let distance = content
        .lines()
        .nth(1)
        .expect("No second line")
        .split(":")
        .nth(1)
        .expect("Nothing after :")
        .replace(" ", "")
        .parse::<u64>()
        .expect("Line was not a number");

    (time, distance)
}

fn part1() {
    let content = open_input("bin/day6/input.txt");
    let races = get_races(&content);
    let res: u32 = races
        .into_iter()
        .map(|(time, distance)| (0..time).filter(|i| (i * (time - i)) > distance).count() as u32)
        .product();
    println!("{res}");
}

fn part2() {
    let content = open_input("bin/day6/input.txt");
    let (time, distance) = get_race_joined(&content);
    let res = (0..time).filter(|i| i * (time - i) > distance).count() as u64;
    println!("{res}");
}

fn main() {
    part1();
    part2();
}

fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn part1() {
    let input = open_input("bin/day2/input1.txt");
    let mut sum_of_ids = 0;
    for line in input.lines() {
        let mut is_valid_game = true;
        let game_split = line.split(":").collect::<Vec<&str>>();
        let game_id = game_split[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .expect("Could not get a valid game ID");
        let game_info = game_split[1];
        let sets = game_info.split(";").collect::<Vec<&str>>();
        for set in sets {
            let mut num_blue = 0;
            let mut num_green = 0;
            let mut num_red = 0;
            let cubes = set.split(",").collect::<Vec<&str>>();
            for cube in cubes {
                let cube_info = cube.split(" ").collect::<Vec<&str>>();
                let num = cube_info[1]
                    .parse::<i32>()
                    .expect("Could not parse number of cubes into an int");
                let color = cube_info[2];
                match color {
                    "blue" => num_blue += num,
                    "red" => num_red += num,
                    "green" => num_green += num,
                    _ => (),
                }
            }
            if num_blue > 14 || num_green > 13 || num_red > 12 {
                is_valid_game = false;
            }
        }
        if is_valid_game {
            sum_of_ids += game_id;
        }
    }
    println!("{sum_of_ids}");
}

fn main() {
    part1()
}

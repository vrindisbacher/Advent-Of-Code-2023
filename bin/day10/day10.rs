fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn get_next_pipe_char(pipe_network: &Vec<Vec<char>>, coords: (usize, usize)) -> (usize, usize) {
    let left = *pipe_network[coords.0].get(coords.1 - 1).unwrap_or(&'.');
    if left == '-' || left == 'F' || left == 'L' {
        return (coords.0, coords.1 - 1);
    }

    let right = *pipe_network[coords.0].get(coords.1 + 1).unwrap_or(&'.');
    if right == '-' || right == '7' || right == 'J' {
        return (coords.0, coords.1 + 1);
    }

    let up = *pipe_network
        .get(coords.0 - 1)
        .unwrap_or(&vec![])
        .get(coords.1)
        .unwrap_or(&'.');
    if up == '|' || up == 'F' || up == '7' {
        return (coords.0 - 1, coords.1);
    }

    let down = *pipe_network
        .get(coords.0 + 1)
        .unwrap_or(&vec![])
        .get(coords.1)
        .unwrap_or(&'.');
    if down == '|' || down == 'L' || down == 'J' {
        return (coords.0 + 1, coords.1);
    }
    panic!()
}

fn get_movement_from_coords(last_coords: (usize, usize), curr_coords: (usize, usize)) -> Direction {
    match (
        curr_coords.0 as i32 - last_coords.0 as i32,
        curr_coords.1 as i32 - last_coords.1 as i32,
    ) {
        (0, 1) => Direction::East,
        (0, -1) => Direction::West,
        (1, 0) => Direction::North,
        (-1, 0) => Direction::South,
        _ => panic!(),
    }
}

fn part1() {
    let content = open_input("bin/day10/input.txt");
    let pipe_network: Vec<Vec<char>> = content.lines().map(|x| x.chars().collect()).collect();
    // find the start
    let mut starting_coords = (0, 0);
    for (r_idx, r) in pipe_network.iter().enumerate() {
        let pos = r.iter().position(|s| *s == 'S');
        if pos.is_some() {
            starting_coords = (r_idx, pos.unwrap());
        }
    }
    let mut coords = get_next_pipe_char(&pipe_network, starting_coords);
    let mut next_char = pipe_network[coords.0][coords.1];
    let mut steps = vec![1];
    let mut direction = get_movement_from_coords(starting_coords, coords);
    while next_char != 'S' {
        let next_coords = match direction {
            Direction::North => match next_char {
                '|' => (coords.0 + 1, coords.1),
                'L' => (coords.0, coords.1 + 1),
                'J' => (coords.0, coords.1 - 1),
                _ => panic!("stuck"),
            },
            Direction::South => match next_char {
                '|' => (coords.0 - 1, coords.1),
                '7' => (coords.0, coords.1 - 1),
                'F' => (coords.0, coords.1 + 1),
                _ => panic!("stuck"),
            },
            Direction::East => match next_char {
                '-' => (coords.0, coords.1 + 1),
                'J' => (coords.0 - 1, coords.1),
                '7' => (coords.0 + 1, coords.1),
                _ => panic!("stuck"),
            },
            Direction::West => match next_char {
                '-' => (coords.0, coords.1 - 1),
                'L' => (coords.0 - 1, coords.1),
                'F' => (coords.0 + 1, coords.1),
                _ => panic!("stuck"),
            },
        };
        steps.push(steps[steps.len() - 1] + 1);
        direction = get_movement_from_coords(coords, next_coords);
        coords = next_coords;
        next_char = pipe_network[coords.0][coords.1];
    }
    let max = steps[steps.len() / 2 - 1];
    println!("{max}");
}

fn part2() {
    let content = open_input("bin/day10/input.txt");
    let pipe_network: Vec<Vec<char>> = content.lines().map(|x| x.chars().collect()).collect();

    let mut starting_coords = (0, 0);
    for (r_idx, r) in pipe_network.iter().enumerate() {
        let pos = r.iter().position(|s| *s == 'S');
        if pos.is_some() {
            starting_coords = (r_idx, pos.unwrap());
        }
    }
    let mut coords = get_next_pipe_char(&pipe_network, starting_coords);
    let mut next_char = pipe_network[coords.0][coords.1];
    let mut loop_tiles = vec![starting_coords, coords];
    let mut direction = get_movement_from_coords(starting_coords, coords);
    while next_char != 'S' {
        let next_coords = match direction {
            Direction::North => match next_char {
                '|' => (coords.0 + 1, coords.1),
                'L' => (coords.0, coords.1 + 1),
                'J' => (coords.0, coords.1 - 1),
                _ => panic!("stuck"),
            },
            Direction::South => match next_char {
                '|' => (coords.0 - 1, coords.1),
                '7' => (coords.0, coords.1 - 1),
                'F' => (coords.0, coords.1 + 1),
                _ => panic!("stuck"),
            },
            Direction::East => match next_char {
                '-' => (coords.0, coords.1 + 1),
                'J' => (coords.0 - 1, coords.1),
                '7' => (coords.0 + 1, coords.1),
                _ => panic!("stuck"),
            },
            Direction::West => match next_char {
                '-' => (coords.0, coords.1 - 1),
                'L' => (coords.0 - 1, coords.1),
                'F' => (coords.0 + 1, coords.1),
                _ => panic!("stuck"),
            },
        };
        loop_tiles.push(next_coords);
        direction = get_movement_from_coords(coords, next_coords);
        coords = next_coords;
        next_char = pipe_network[coords.0][coords.1];
    }
    let mut sum = 0;
    loop_tiles = loop_tiles[0..loop_tiles.len() - 1].to_vec();
    for i in 0..loop_tiles.len() {
        sum += (loop_tiles[i].1 as i64 * loop_tiles[(i + 1) % loop_tiles.len()].0 as i64)
            - (loop_tiles[i].0 as i64 * loop_tiles[(i + 1) % loop_tiles.len()].1 as i64);
    }
    let area = sum.abs() / 2;
    let res = area + 1 - (loop_tiles.len() as i64 / 2);
    println!("{res}");
}

// 725: Too high

fn main() {
    part1();
    part2();
}

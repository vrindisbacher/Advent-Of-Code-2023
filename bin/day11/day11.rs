fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn get_row_and_col_offsets_pt1(
    input_graph: &Vec<Vec<char>>,
) -> (Vec<usize>, Vec<usize>) {
    let mut offset = 0;
    let mut amount_to_add_row = Vec::new();
    for row in input_graph.iter() {
        let has_num = row.iter().filter(|x| **x == '#').count() > 0;
        if !has_num {
            amount_to_add_row.push(offset + 1);
            offset += 1;
        } else {
            amount_to_add_row.push(offset);
        }
    }

    let mut offset = 0;
    let mut amount_to_add_col = Vec::new();
    for col_idx in 0..input_graph[0].len() {
        let has_num = (0..input_graph.len())
            .filter(|row_idx| input_graph[*row_idx][col_idx] == '#')
            .count()
            > 0;
        if !has_num {
            amount_to_add_col.push(offset + 1);
            offset += 1;
        } else {
            amount_to_add_col.push(offset);
        }
    }
    (amount_to_add_row, amount_to_add_col)
}

fn get_row_and_col_offsets_pt2(
    input_graph: &Vec<Vec<char>>,
    multiplier: usize,
) -> (Vec<usize>, Vec<usize>) {
    let mut offset = 0;
    let mut amount_to_add_row = Vec::new();
    for row in input_graph.iter() {
        let has_num = row.iter().filter(|x| **x == '#').count() > 0;
        if !has_num {
            amount_to_add_row.push(multiplier + offset - 1);
            offset += multiplier - 1;
        } else {
            amount_to_add_row.push(offset);
        }
    }

    let mut offset = 0;
    let mut amount_to_add_col = Vec::new();
    for col_idx in 0..input_graph[0].len() {
        let has_num = (0..input_graph.len())
            .filter(|row_idx| input_graph[*row_idx][col_idx] == '#')
            .count()
            > 0;
        if !has_num {
            amount_to_add_col.push(multiplier + offset - 1);
            offset += multiplier - 1;
        } else {
            amount_to_add_col.push(offset);
        }
    }
    (amount_to_add_row, amount_to_add_col)
}

fn part1() {
    let content = open_input("bin/day11/input.txt");
    let input_graph = content
        .lines()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let (row_offset, col_offset) = get_row_and_col_offsets_pt1(&input_graph);
    let mut idxs = Vec::new();
    for (row_idx, row) in input_graph.iter().enumerate() {
        for (col_idx, ch) in row.iter().enumerate() {
            if *ch == '#' {
                idxs.push((row_idx + row_offset[row_idx], col_idx + col_offset[col_idx]));
            }
        }
    }
    let mut sum = 0;
    for (start_row_idx, start_col_idx) in idxs.iter() {
        for (end_row_idx, end_col_idx) in idxs.iter() {
            sum += (*end_row_idx as i32 - *start_row_idx as i32).abs()
                + (*end_col_idx as i32 - *start_col_idx as i32).abs();
        }
    }
    println!("{}", sum / 2);
}

fn part2() {
    let content = open_input("bin/day11/input.txt");
    let input_graph = content
        .lines()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let (row_offset, col_offset) = get_row_and_col_offsets_pt2(&input_graph, 1000000);
    let mut idxs = Vec::new();
    for (row_idx, row) in input_graph.iter().enumerate() {
        for (col_idx, ch) in row.iter().enumerate() {
            if *ch == '#' {
                idxs.push((row_idx + row_offset[row_idx], col_idx + col_offset[col_idx]));
            }
        }
    }
    let mut sum = 0;
    for (start_row_idx, start_col_idx) in idxs.iter() {
        for (end_row_idx, end_col_idx) in idxs.iter() {
            sum += (*end_row_idx as i32 - *start_row_idx as i32).abs()
                + (*end_col_idx as i32 - *start_col_idx as i32).abs();
        }
    }
    println!("{}", sum / 2);
}

fn main() {
    part1();
    part2();
}

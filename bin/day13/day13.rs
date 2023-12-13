fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn find_column_reflection(input: &Vec<Vec<char>>) -> (usize, usize) {
    let mut fst = 0;
    let mut snd = 0;
    for col_idx in 0..(input[0].len() - 1) {
        let mut equal = true;
        for row_idx in 0..input.len() {
            if input[row_idx][col_idx] != input[row_idx][col_idx + 1] {
                equal = false;
                break;
            }
        }
        if equal {
            // check the left and right
            let left_dist = col_idx - 0;
            let right_dist = input[0].len() - col_idx - 2;
            let range;
            if left_dist > right_dist {
                range =
                    ((left_dist - right_dist)..col_idx).zip(((col_idx + 2)..input[0].len()).rev());
            } else {
                range = (0..col_idx)
                    .zip(((col_idx + 2)..(input[0].len() - (right_dist - left_dist))).rev());
            }

            for (col_idx1, col_idx2) in range {
                for row_idx_check in 0..input.len() {
                    if input[row_idx_check][col_idx1] != input[row_idx_check][col_idx2] {
                        equal = false;
                        break;
                    }
                }
            }
        }

        if equal {
            fst = col_idx;
            snd = col_idx + 1;
            break;
        }
    }
    (fst, snd)
}

fn find_row_reflection(input: &Vec<Vec<char>>) -> (usize, usize) {
    let mut fst = 0;
    let mut snd = 0;
    for row_idx in 0..(input.len() - 1) {
        let mut equal = true;
        for col_idx in 0..input[0].len() {
            if input[row_idx][col_idx] != input[row_idx + 1][col_idx] {
                equal = false;
                break;
            }
        }
        if equal {
            let left_dist = row_idx - 0;
            let right_dist = input.len() - row_idx - 2;
            let range;
            if left_dist > right_dist {
                range = ((left_dist - right_dist)..row_idx).zip(((row_idx + 2)..input.len()).rev());
            } else {
                range = (0..row_idx)
                    .zip(((row_idx + 2)..(input.len() - (right_dist - left_dist))).rev());
            }
            for (row_idx1, row_idx2) in range {
                for col_idx_check in 0..input[0].len() {
                    if input[row_idx1][col_idx_check] != input[row_idx2][col_idx_check] {
                        equal = false;
                        break;
                    }
                }
            }
        }
        if equal {
            fst = row_idx;
            snd = row_idx + 1;
            break;
        }
    }
    (fst, snd)
}

fn part1() {
    let content = open_input("bin/day13/input.txt");
    let all_inputs: Vec<Vec<Vec<char>>> = content
        .split("\n\n")
        .map(|i| {
            i.lines()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect();
    let mut sum = 0;
    for input in all_inputs {
        let (fst_col, snd_col) = find_column_reflection(&input);
        if (0, 0) != (fst_col, snd_col) {
            sum += snd_col;
        } else {
            let (fst_row, snd_row) = find_row_reflection(&input);
            if (0, 0) != (fst_row, snd_row) {
                sum += snd_row * 100;
            }
        }
    }
    println!("{sum}");
}

fn main() {
    part1();
}

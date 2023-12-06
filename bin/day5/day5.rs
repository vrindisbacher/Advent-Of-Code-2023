use rayon::prelude::*;
use std::collections::HashMap;

fn open_input(filename: &str) -> String {
    let content = std::fs::read_to_string(filename).expect(&format!("Could not open {filename}"));
    content
}

fn get_seed_numbers(content: &String) -> Vec<i64> {
    content
        .lines()
        .nth(0)
        .expect("There was no first line")
        .split(":")
        .nth(1)
        .expect("Nothing after :")
        .trim()
        .split(" ")
        .flat_map(|x| x.parse::<i64>())
        .collect()
}

fn get_maps_from_input(
    content: String,
) -> (
    HashMap<String, String>,
    HashMap<(String, String), Vec<(i64, i64, i64)>>,
) {
    let rest_of_input = content.lines().collect::<Vec<&str>>()[2..].join("\n");
    let rest_of_lines = rest_of_input.split("\n\n").collect::<Vec<&str>>();
    let mut str_pair_to_corresponding_nums = HashMap::new();
    let mut source_to_dest = HashMap::new();
    for map_input in rest_of_lines {
        let map_name = map_input
            .split(":")
            .nth(0)
            .expect("No first line in map input");
        let from_resource = map_name
            .split("-")
            .nth(0)
            .expect("Could not get from resource in map name");
        let to_resource = map_name
            .split("-")
            .nth(2)
            .expect("Could not get to resource in map name")
            .split(" ")
            .nth(0)
            .expect("Could not get to resource in map name");

        source_to_dest.insert(from_resource.to_string(), to_resource.to_string());

        let rest_of_input = map_input
            .split(":")
            .nth(1)
            .expect("No rest of lines in map input");

        let mut all_mappings = Vec::new();
        for v in rest_of_input.trim().split("\n").map(|x| {
            x.trim()
                .split(" ")
                .map(|x| x.parse::<i64>().expect("Value in map was not a number"))
                .collect::<Vec<i64>>()
        }) {
            let source_start_val = v[0];
            let dest_start_val = v[1];
            let num_values = v[2];
            all_mappings.push((source_start_val, dest_start_val, num_values));
        }
        str_pair_to_corresponding_nums.insert(
            (from_resource.to_string(), to_resource.to_string()),
            all_mappings,
        );
    }
    (source_to_dest, str_pair_to_corresponding_nums)
}

fn part1() {
    let content = open_input("bin/day5/input.txt");
    // get the seed numbers
    let seed_numbers = get_seed_numbers(&content);
    // get the rest of the input
    let (source_to_dest_map, pair_to_corresponding_num_map) = get_maps_from_input(content);
    let res = seed_numbers
        .into_par_iter()
        .map(|seed_num| {
            let mut dest = "seed";
            let mut value = seed_num;
            while dest != "location" {
                let next_dest = source_to_dest_map.get(dest).expect("No dest for source");
                let all_mappings = pair_to_corresponding_num_map
                    .get(&(dest.to_string(), next_dest.to_string()))
                    .expect("No map for source dest pair");
                for (dest_start, src_start, range_len) in all_mappings {
                    if (value >= *src_start) && value < *range_len + *src_start {
                        value = (value - *src_start) + *dest_start;
                        break;
                    }
                }
                dest = next_dest;
            }
            value
        })
        .min();
    println!("{res:?}");
}

fn seed_numbers_from_range(seed_numbers: Vec<i64>) -> Vec<i64> {
    (0..seed_numbers.len())
        .into_par_iter()
        .step_by(2)
        .map(|i| {
            let initial = seed_numbers[i];
            let range_seed = seed_numbers[i + 1];
            (0..range_seed).into_par_iter().map(move |x| initial + x)
        })
        .flatten()
        .collect::<Vec<i64>>()
}

fn part2() {
    let content = open_input("bin/day5/input.txt");
    // get the seed numbers
    let seed_numbers = seed_numbers_from_range(get_seed_numbers(&content));
    println!("Got seeds");
    // get the rest of the input
    let (source_to_dest_map, pair_to_corresponding_num_map) = get_maps_from_input(content);
    let res = seed_numbers
        .into_par_iter()
        .map(|seed_num| {
            let mut dest = "seed";
            let mut value = seed_num;
            while dest != "location" {
                let next_dest = source_to_dest_map.get(dest).expect("No dest for source");
                let all_mappings = pair_to_corresponding_num_map
                    .get(&(dest.to_string(), next_dest.to_string()))
                    .expect("No map for source dest pair");
                for (dest_start, src_start, range_len) in all_mappings {
                    if (value >= *src_start) && value < *range_len + *src_start {
                        value = (value - *src_start) + *dest_start;
                        break;
                    }
                }
                dest = next_dest;
            }
            value
        })
        .min();
    println!("{res:?}");
}

fn main() {
    part1();
    part2();
}

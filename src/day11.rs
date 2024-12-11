use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    vec,
};

use colored::Colorize;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day11.txt"));

    if part_opt.is_none() {
        println!("Running Day 11, Part 1");
        part1(&input);
        println!();
        println!("Running Day 11, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 11, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 11, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

fn part1(input_file: &PathBuf) {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );

    let lines = BufReader::new(input_file).lines();
    let mut stones: Vec<u64> = Vec::new();
    for line in lines {
        let line = line.expect("Could not read line");
        stones = line
            .split_ascii_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        break;
    }

    println!("Initial: {:?}", stones);

    let num_blinks = 25;

    // Cache (initial_state -> (step_count -> [final_states]))
    let mut cache: HashMap<u64, HashMap<u64, Vec<u64>>> = HashMap::new();
    let final_stones = blink(&stones, &mut cache, num_blinks);

    // println!(
    //     "After {} blinks: {:?}",
    //     num_blinks.to_string().as_str().red(),
    //     final_stones
    // );
    println!(
        "Number of stones: {}",
        format!("{}", final_stones.len()).as_str().green().bold(),
    );
}

fn blink(
    stones: &Vec<u64>,
    cache: &mut HashMap<u64, HashMap<u64, Vec<u64>>>,
    num_blinks: u64,
) -> Vec<u64> {
    if num_blinks == 0 {
        return stones.clone();
    }

    let mut ret: Vec<u64> = Vec::new();

    for stone in stones.iter() {
        // Cache hit. Easy case.

        let stone_entry = cache.entry(*stone).or_insert_with(|| {
            let mut val: HashMap<u64, Vec<u64>> = HashMap::new();
            val.insert(0, vec![*stone]);
            val
        });

        if stone_entry.contains_key(&num_blinks) {
            // println!(
            //     "Cache Hit! {:?} -> {:?} -> {:?}",
            //     stone,
            //     num_blinks,
            //     stone_entry.get(&num_blinks).unwrap()
            // );
            ret.extend(stone_entry.get(&num_blinks).unwrap());
            continue;
        }

        // Cache miss. Find the last step calculated
        let mut last_step_calculated = *stone_entry
            .keys()
            .filter(|k| **k < num_blinks)
            .max()
            .unwrap_or(&0);

        let latest_stone_state = if last_step_calculated == 0 {
            let next_state = calculate_next_stones(*stone);
            stone_entry.insert(last_step_calculated + 1, next_state.clone());
            last_step_calculated = 1;
            next_state
        } else {
            stone_entry.get(&last_step_calculated).unwrap().clone()
        };

        // Calculate the next state
        let final_state = blink(
            &latest_stone_state,
            cache,
            num_blinks - last_step_calculated,
        );
        ret.extend(&final_state);
        cache
            .get_mut(stone)
            .unwrap()
            .insert(num_blinks, final_state);
    }

    return ret;
}

fn calculate_next_stones(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let stone_str = stone.to_string();
    let num_digits = stone_str.chars().count();
    if num_digits % 2 == 0 {
        let first_half = &stone_str[..num_digits / 2];
        let second_half = &stone_str[num_digits / 2..];
        return vec![
            first_half.parse::<u64>().unwrap(),
            second_half.parse::<u64>().unwrap(),
        ];
    }

    return vec![stone * 2024];
}

fn part2(input_file: &PathBuf) {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );

    let lines = BufReader::new(input_file).lines();
    let mut stones: Vec<u64> = Vec::new();
    for line in lines {
        let line = line.expect("Could not read line");
        stones = line
            .split_ascii_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        break;
    }

    let num_blinks = 75;

    let mut cache: HashMap<u64, HashMap<u64, u64>> = HashMap::new();

    println!("Initial: {:?}", stones);

    let total_stones: u64 = stones
        .iter()
        .map(|s| get_number_of_stones_after_n_blinks(*s, &mut cache, num_blinks))
        .sum();
    println!(
        "Number of stones: {}",
        total_stones.to_string().as_str().green().bold(),
    );
}

fn get_number_of_stones_after_n_blinks(
    stone: u64,
    cache: &mut HashMap<u64, HashMap<u64, u64>>,
    n: usize,
) -> u64 {
    if n == 0 {
        return 1;
    }

    if cache.contains_key(&stone) && cache.get(&stone).unwrap().contains_key(&(n as u64)) {
        return *cache.get(&stone).unwrap().get(&(n as u64)).unwrap();
    }

    let next_state = calculate_next_stones(stone);
    let total: u64 = next_state
        .iter()
        .map(|s| get_number_of_stones_after_n_blinks(*s, cache, n - 1))
        .sum();
    cache
        .entry(stone)
        .or_insert_with(HashMap::new)
        .insert(n as u64, total);
    return total;
}

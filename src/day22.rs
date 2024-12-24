use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use colored::Colorize;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day22.txt"));

    if part_opt.is_none() {
        println!("Running Day 22, Part 1");
        part1(&input);
        println!();
        println!("Running Day 22, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 22, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 22, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

fn get_initial_numbers_from_file(input_file: &PathBuf) -> Vec<u64> {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );
    let lines = BufReader::new(input_file).lines();

    return lines
        .into_iter()
        .map(|l| l.expect("Could not read line."))
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
}

fn mix(a: u64, b: u64) -> u64 {
    return a ^ b;
}

fn prune(a: u64) -> u64 {
    return a & (0b1_0000_0000_0000_0000_0000_0000 - 1);
}

fn next_secret_1(secret: u64) -> u64 {
    let result = secret << 6; // secret * 64;
    let result = mix(result, secret);
    let result = prune(result);
    return result;
}

fn next_secret_2(secret: u64) -> u64 {
    let result = secret >> 5; // secret / 32
    let result = mix(result, secret);
    let result = prune(result);
    return result;
}

fn next_secret_3(secret: u64) -> u64 {
    let result = secret << 11; // secret * 2048;
    let result = mix(result, secret);
    let result = prune(result);
    return result;
}

fn next_secret(secret: u64) -> u64 {
    let result = next_secret_1(secret);
    let result = next_secret_2(result);
    let result = next_secret_3(result);
    return result;
}

fn part1(input_file: &PathBuf) {
    let first_secrets = get_initial_numbers_from_file(input_file);
    // println!("{:#?}", first_secrets);

    let mut secret_sum: u64 = 0;
    for secret in first_secrets {
        let mut working_secret = secret;
        for i in 1..=2000 {
            let next_secret = next_secret(working_secret);
            if i == 2000 {
                println!("{:>10} : {}", secret, next_secret);
            }
            working_secret = next_secret;
        }
        secret_sum += working_secret;
    }

    println!();
    println!("Sum of secrets: {}", secret_sum.to_string().green().bold());
}

fn part2(input_file: &PathBuf) {
    let first_secrets = get_initial_numbers_from_file(input_file);

    // Brute force FTW!
    let seq_maps: Vec<HashMap<(i64, i64, i64, i64), u64>> = first_secrets
        .iter()
        .map(|s| get_seq_price_map(*s))
        .collect();

    let seq_set: HashSet<(i64, i64, i64, i64)> =
        seq_maps.iter().flat_map(|m| m.keys()).map(|k| *k).collect();

    println!("Number of unique sequences: {}", seq_set.len());

    let max_sum = seq_set
        .iter()
        .map(|s| {
            (
                s,
                seq_maps
                    .iter()
                    .map(|m| m.get(s).unwrap_or(&0))
                    .map(|v| *v)
                    .sum::<u64>(),
            )
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(seq, sum)| (*seq, sum))
        .unwrap();

    println!(
        "Best Sequence : {}",
        format!("{:?}", max_sum.0).as_str().cyan().bold()
    );
    println!("Best Sum      : {}", max_sum.1.to_string().green().bold());
}

fn get_seq_price_map(secret: u64) -> HashMap<(i64, i64, i64, i64), u64> {
    let mut seq_to_price: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();

    let mut past_diffs: VecDeque<i64> = VecDeque::with_capacity(5);
    let mut working_secret = secret;
    let mut working_price = secret % 10;

    for _ in 0..2000 {
        let next_secret = next_secret(working_secret);
        let next_price = next_secret % 10;
        let diff = next_price as i64 - working_price as i64;
        past_diffs.push_back(diff);
        if past_diffs.len() > 4 {
            past_diffs.pop_front().unwrap();
        }

        if past_diffs.len() == 4 {
            seq_to_price
                .entry(vec_to_tuple(&past_diffs))
                .or_insert(next_price);
        }

        working_price = next_price;
        working_secret = next_secret;
    }

    return seq_to_price;
}

fn vec_to_tuple(v: &VecDeque<i64>) -> (i64, i64, i64, i64) {
    return (v[0], v[1], v[2], v[3]);
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    result,
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
    todo!("Implement Part2");
}

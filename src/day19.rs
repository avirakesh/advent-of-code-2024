use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use colored::Colorize;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day19.txt"));

    if part_opt.is_none() {
        println!("Running Day 19, Part 1");
        part1(&input);
        println!();
        println!("Running Day 19, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 19, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 19, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

fn read_towel_and_pattern_from_file(input_file: &PathBuf) -> (Vec<String>, Vec<String>) {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );
    let mut lines = BufReader::new(input_file).lines().into_iter();

    let line = lines.next().unwrap().expect("Could not read line");
    let towels: Vec<String> = line
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.to_string())
        .collect();

    lines.next(); // Skip the empty line between towels and pattern.

    let mut pattern: Vec<String> = vec![];
    while let Some(line) = lines.next() {
        pattern.push(line.expect("Could not read line").trim().to_string());
    }

    return (towels, pattern);
}

fn part1(input_file: &PathBuf) {
    let (towels, patterns) = read_towel_and_pattern_from_file(input_file);
    println!("Towels: {:?}", towels);
    // println!("Patterns: {:#?}", patterns);

    let mut num_possible_patterns: usize = 0;
    for pattern in patterns {
        if can_make_pattern(&towels, &pattern) {
            println!("{}  {}", "✓".green().bold(), pattern);
            num_possible_patterns += 1;
        } else {
            println!("{}  {}", "✗".red().bold(), pattern);
        }
    }

    println!();
    println!(
        "Number of possible patterns: {}",
        num_possible_patterns.to_string().green().bold()
    );
}

fn can_make_pattern(towels: &[String], pattern: &String) -> bool {
    let mut pattern_to_make: Vec<&str> = Vec::new();

    pattern_to_make.push(pattern.as_str());

    while !pattern_to_make.is_empty() {
        let current_pattern = pattern_to_make.pop().unwrap();
        // println!("Current pattern to make: {:?}", current_pattern);
        if current_pattern.is_empty() {
            return true;
        }

        pattern_to_make.extend(
            towels
                .iter()
                .filter(|t| current_pattern.starts_with(*t))
                .map(|t| current_pattern.strip_prefix(t.as_str()).unwrap()),
        );
    }

    return false;
}

fn part2(input_file: &PathBuf) {
    todo!("Implement Part2")
}

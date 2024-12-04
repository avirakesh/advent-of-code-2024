use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use regex::Regex;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day3.txt"));

    if part_opt.is_none() {
        println!("Running Day 3, Part 1");
        part1(&input);
        println!();
        println!("Running Day 3, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 3, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 3, Part 2");
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

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for line_res in lines {
        let line = line_res.expect("Could not read line");
        println!("{:?}", line);
        let matches: Vec<(i32, i32)> = re
            .captures_iter(line.as_str())
            .map(|e| {
                (
                    e.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    e.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                )
            })
            .collect();
        println!("Matches found: {:?}", matches);
        sum += matches.iter().map(|f| f.0 * f.1).sum::<i32>();
    }

    println!("Sum: {}", sum);
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

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)").unwrap();
    let do_str = "do()";
    let dont_str = "don't()";

    let mut sum = 0;
    let mut is_enabled = true;
    for line_res in lines {
        let line = line_res.expect("Could not read line");
        println!("{:?}", line);
        let matches = re.captures_iter(line.as_str());

        let mut factor_list: Vec<(i32, i32)> = Vec::new();

        for m in matches {
            let matc = m.get(0).unwrap().as_str();
            if matc == do_str {
                is_enabled = true;
                continue;
            }

            if matc == dont_str {
                is_enabled = false;
                continue;
            }

            if is_enabled {
                factor_list.push((
                    m.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    m.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                ));
            }
        }

        println!("{:?}", factor_list);
        sum += factor_list.iter().map(|f| f.0 * f.1).sum::<i32>();
    }

    println!("Sum: {}", sum);
}

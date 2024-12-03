use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day2.txt"));

    if part_opt.is_none() {
        println!("Running Day 2, Part 1");
        part1(&input);
        println!();
        println!("Running Day 2, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 1, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 2, Part 2");
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

    let mut ordered_reports = 0;
    for line_res in lines {
        let line = line_res.expect("Could not read line");
        let report: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|s| {
                s.parse::<i32>()
                    .expect(format!("Failed to convert {} to u32", s).as_str())
            })
            .collect();

        print!("Report: {:?}", &report);
        if is_report_ordered_within_margin(&report) {
            ordered_reports += 1;
            println!(": ordered");
        } else {
            println!(": unordered");
        }
    }
    println!("Ordered reports: {}", ordered_reports);
}

fn is_report_ordered_within_margin(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let order = (report[1] - report[0]).signum();
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff.signum() != order {
            return false;
        }

        let diff: i32 = diff.abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn part2(input_file: &PathBuf) {
    todo!("Implement Part 2");
}

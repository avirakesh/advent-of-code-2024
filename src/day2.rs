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
            println!("Running Day 2, Part 1");
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
            println!(": safe");
        } else {
            println!(": unsafe");
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
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );
    let lines = BufReader::new(input_file).lines();

    let mut safe_reports = 0;
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
            safe_reports += 1;
            println!(": safe");
            continue;
        }

        let (negative_count, zero_count, positive_count) = get_report_order_counts(&report);
        if zero_count > 1 {
            println!(": unsafe -- too many repeated elements");
            continue;
        } else if negative_count > 1 && positive_count > 1 {
            println!(": unsafe -- order can't be fixed");
            continue;
        }

        if report.len() == 3 {
            if check_if_report_can_be_fixed(&report, -1) || check_if_report_can_be_fixed(&report, 1)
            {
                safe_reports += 1;
                println!(": safe -- fixable");
            } else {
                println!(": unsafe -- not-fixable");
            }
            continue;
        }

        let order = (positive_count - negative_count).signum();

        if check_if_report_can_be_fixed(&report, order) {
            safe_reports += 1;
            println!(": safe -- fixable");
        } else {
            println!(": unsafe -- not-fixable");
        }
    }
    println!("Safe Report: {}", safe_reports);
}

fn check_if_report_can_be_fixed(report: &Vec<i32>, order: i32) -> bool {
    if report.len() < 3 {
        return true; // Short reports are always safe.
    }

    let mut error_count = 0;
    let mut cursor = 0;
    for i in 1..report.len() {
        if error_count > 1 {
            break; // Too many errors, don't bother fixing.
        }

        let diff = report[i] - report[cursor];
        if diff.signum() != order {
            error_count += 1;
            continue;
        }

        let diff = diff.abs();
        if diff < 1 || diff > 3 {
            error_count += 1;
            continue;
        }
        cursor = i; // Move the pointer to the last checked element.
    }

    if error_count < 2 {
        return true; // Either no errors, or one error we can skip over
    }

    // There were more than 1 errors. Maybe skipping the first element fixes it?
    let mut error_count = 1; // Can't fix any more things.
    let mut cursor = 1; // skip the first element
    for i in 2..report.len() {
        if error_count > 1 {
            break; // Too many errors, don't bother fixing.
        }

        let diff = report[i] - report[cursor];
        if diff.signum() != order {
            error_count += 1;
            continue;
        }

        let diff = diff.abs();
        if diff < 1 || diff > 3 {
            error_count += 1;
            continue;
        }
        cursor = i; // Move the pointer to the last checked element.
    }
    return error_count < 2;
}

fn get_report_order_counts(report: &Vec<i32>) -> (i32, i32, i32) {
    let mut order_positive_count = 0;
    let mut order_netagive_count = 0;
    let mut order_zero_count = 0;
    for i in 1..report.len() {
        let order = (report[i] - report[i - 1]).signum();
        match order {
            -1 => order_netagive_count += 1,
            0 => order_zero_count += 1,
            1 => order_positive_count += 1,
            _ => unreachable!(),
        }
    }
    return (order_netagive_count, order_zero_count, order_positive_count);
}

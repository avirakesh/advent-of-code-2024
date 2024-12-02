use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day1.txt"));

    if part_opt.is_none() {
        println!("Running Day 1, Part 1");
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
        },
        2 => {
            println!("Running Day 2, Part 2");
            part2(&input)
        },
        _ => panic!("Invalid Part :(")
    }
}

fn part1(input_file: &PathBuf) {
    let input_file = File::open(input_file).expect(format!("Could not open input file: {}", input_file.to_string_lossy()).as_str());
    let lines = BufReader::new(input_file).lines();

    let mut list1: Vec<i32>  = Vec::new();
    let mut list2: Vec<i32>  = Vec::new();

    for line in lines {
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        list1.push(parts[0].parse::<i32>().unwrap());
        list2.push(parts[1].parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();
    println!("list1: {:?}", list1);
    println!("list2: {:?}", list2);

    let diffs:Vec<i32> = list1.into_iter().zip(list2.into_iter()).map(|x| {(x.0 - x.1).abs()}).collect();
    println!("diffs: {:?}", diffs);
    let sum:i32 = diffs.iter().sum();
    println!("Sum: {}", sum);
}

fn part2(input_file: &PathBuf) {
    todo!("Implement Day 1 Part 2");
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
    path::PathBuf,
};

use colored::Colorize;
use regex::Regex;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day13.txt"));

    if part_opt.is_none() {
        println!("Running Day 13, Part 1");
        part1(&input);
        println!();
        println!("Running Day 13, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 13, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 13, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    fn from_tuple(p: (i32, i32)) -> Self {
        Coord { x: p.0, y: p.1 }
    }
}

impl Mul<i32> for Coord {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add<Coord> for Coord {
    type Output = Self;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Problem {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
}

impl Problem {
    fn from_lines(lines: &Vec<String>) -> Self {
        let regexes = vec![
            Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap(),
            Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap(),
            Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap(),
        ];

        assert_eq!(lines.len(), regexes.len());
        for (line, regex) in lines.iter().zip(regexes.iter()) {
            assert!(regex.is_match(line));
        }

        let button_a = regexes[0].captures(&lines[0]).unwrap();
        let button_a: (i32, i32) = (button_a[1].parse().unwrap(), button_a[2].parse().unwrap());

        let button_b = regexes[1].captures(&lines[1]).unwrap();
        let button_b: (i32, i32) = (button_b[1].parse().unwrap(), button_b[2].parse().unwrap());

        let prize = regexes[2].captures(&lines[2]).unwrap();
        let prize: (i32, i32) = (prize[1].parse().unwrap(), prize[2].parse().unwrap());

        return Self {
            button_a: Coord::from_tuple(button_a),
            button_b: Coord::from_tuple(button_b),
            prize: Coord::from_tuple(prize),
        };
    }

    fn solve(&self) -> Result<(i32, i32), ()> {
        let a = &self.button_a;
        let b = &self.button_b;
        let p = &self.prize;

        let demoninator = (b.x * a.y) - (a.x * b.y);
        let numerator = (p.x * a.y) - (a.x * p.y);

        if demoninator == 0 || numerator % demoninator != 0 {
            return Err(());
        }

        let n_b = numerator / demoninator;
        // println!("n_b = {} / {} = {}", numerator, demoninator, n_b);

        let demoninator = a.y;
        let numerator = p.y - (n_b * b.y);
        if demoninator == 0 || numerator % demoninator != 0 {
            return Err(());
        }

        let n_a = numerator / demoninator;
        // println!("n_a = {} / {} = {}", (p.y - (n_b * b.y)), a.y, n_a);

        let sanity_check = (self.button_a * n_a) + (self.button_b * n_b);
        if (self.prize == sanity_check) {
            // println!("Sanity check passed!");
            return Ok((n_a, n_b));
        } else {
            // println!(
            //     "Sanity check failed. Expected: {:?}; Actual: {:?}",
            //     self.prize, sanity_check
            // );
            return Err(());
        }
    }
}

fn get_problems_from_file(input_file: &PathBuf) -> Vec<Problem> {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );

    let regexes = vec![
        Regex::new(r"Button A: X\+\d+, Y\+\d+").unwrap(),
        Regex::new(r"Button B: X\+\d+, Y\+\d+").unwrap(),
        Regex::new(r"Prize: X=\d+, Y=\d+").unwrap(),
    ];

    let lines = BufReader::new(input_file).lines();

    let mut problem_raw: Vec<String> = Vec::with_capacity(regexes.len());
    let mut problems: Vec<Problem> = Vec::new();
    for line in lines {
        let line = line.expect("Could not read line");
        if problem_raw.len() == regexes.len() {
            problems.push(Problem::from_lines(&problem_raw));
            problem_raw.clear();
            continue;
        }

        if regexes[problem_raw.len()].is_match(&line) {
            problem_raw.push(line);
        } else {
            panic!("Invalid input found: {}", line);
        }
    }

    if problem_raw.len() == regexes.len() {
        problems.push(Problem::from_lines(&problem_raw));
    }

    return problems;
}

fn part1(input_file: &PathBuf) {
    let problems = get_problems_from_file(input_file);

    let solutions: Vec<(i32, i32)> = problems
        .iter()
        .map(|p| p.solve())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .filter(|s| s.0 <= 100 && s.1 <= 100)
        .collect();

    let tokens = solutions
        .iter()
        .map(|(n_a, n_b)| n_a * 3 + n_b)
        .sum::<i32>();
    println!(
        "Tokens Spent: {}",
        tokens.to_string().as_str().green().bold()
    );
}

fn part2(input_file: &PathBuf) {
    todo!("Implement Part 2");
}

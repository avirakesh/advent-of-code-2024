use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};
use std::io::BufRead;
use std::{fs::File, io::BufReader, path::PathBuf};

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day7.txt"));

    if part_opt.is_none() {
        println!("Running Day 7, Part 1");
        part1(&input);
        println!();
        println!("Running Day 7, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 7, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 7, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Plus,
    Mutiply,
    Concat,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Operator::Plus => write!(f, "+"),
            Operator::Mutiply => write!(f, "*"),
            Operator::Concat => write!(f, "||"),
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    target: u64,
    operands: Vec<u64>,
}

impl Problem {
    fn from_line(line: &str) -> Problem {
        // Parse the line into a problem
        let split: Vec<&str> = line.split(":").collect();
        let target = split[0].trim().parse::<u64>().unwrap();

        let operands_str = split[1].trim();
        let operands: Vec<u64> = operands_str
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        return Self { target, operands };
    }

    fn get_possible_solution_operators(&self, check_concat: bool) -> Vec<Vec<Operator>> {
        // There is an assumption that the values never get smaller
        // after an operation. Given that we're working with +, *, and ||, the
        // only situation this assumption might fail is with 'some number * 0'.
        // All other operations will always result in larger (or equal) number.
        // Manually verified that 0 is never an operand in the input :P
        let mut working_operands: VecDeque<(u64, Vec<Operator>)> = VecDeque::new();
        working_operands.push_back((self.operands[0], Vec::new()));

        for operand in self.operands.iter().skip(1) {
            let working_operands_count = working_operands.len();

            if working_operands_count == 0 {
                break;
            }

            for _ in 0..working_operands_count {
                let (current_value, current_operands) = working_operands.pop_front().unwrap();

                // Check if + is a valid option
                let next_operand = current_value + operand;
                if next_operand <= self.target {
                    let mut next_operators = current_operands.clone();
                    next_operators.push(Operator::Plus);
                    working_operands.push_back((next_operand, next_operators));
                }

                // Check if * is a valid option
                let next_operand = current_value * operand;
                if next_operand <= self.target {
                    let mut next_operators = current_operands.clone();
                    next_operators.push(Operator::Mutiply);
                    working_operands.push_back((next_operand, next_operators));
                }

                if check_concat {
                    // Check if || is a valid option
                    let next_operand = format!("{}{}", current_value, operand);
                    let next_operand = next_operand.parse::<u64>().unwrap();
                    if next_operand <= self.target {
                        let mut next_operators = current_operands.clone();
                        next_operators.push(Operator::Concat);
                        working_operands.push_back((next_operand, next_operators));
                    }
                }
            }
        }

        return working_operands
            .into_iter()
            .filter(|v| v.0 == self.target)
            .map(|v| v.1)
            .collect();
    }

    fn pretty_print_solution(&self, operators: &Vec<Operator>) {
        if operators.len() != self.operands.len() - 1 {
            panic!("Length of operators should be one less than length of operators");
        }

        print!("    {} = ", self.target);
        for (operand, operator) in self.operands.iter().zip(operators.iter()) {
            print!("{} {} ", operand, operator);
        }
        println!("{}", self.operands.last().unwrap());
    }
}

fn part1(input_file: &PathBuf) {
    let problems = get_problems_from_file(input_file);

    let mut num_solved_problems = 0;
    let mut solved_problems_sum = 0;
    println!("Solved Problems:");
    for problem in problems.iter() {
        let solutions = problem.get_possible_solution_operators(/* check_concat: */ false);

        for solution in solutions.iter() {
            problem.pretty_print_solution(solution);
        }
        if solutions.len() > 0 {
            num_solved_problems += 1;
            solved_problems_sum += problem.target;
            println!();
        }
    }

    println!("Number of solved problems: {}", num_solved_problems);
    println!("Sum of solved problems: {}", solved_problems_sum);
}

fn get_problems_from_file(input_file: &PathBuf) -> Vec<Problem> {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );
    let lines = BufReader::new(input_file).lines();

    let mut problems = Vec::new();
    for line in lines {
        let line = line.expect("Could not read line");
        problems.push(Problem::from_line(&line));
    }

    return problems;
}

fn part2(input_file: &PathBuf) {
    let problems = get_problems_from_file(input_file);

    let mut num_solved_problems = 0;
    let mut solved_problems_sum = 0;
    println!("Solved Problems:");
    for problem in problems.iter() {
        let solutions = problem.get_possible_solution_operators(/* check_concat: */ true);

        for solution in solutions.iter() {
            problem.pretty_print_solution(solution);
        }
        if solutions.len() > 0 {
            num_solved_problems += 1;
            solved_problems_sum += problem.target;
            println!();
        }
    }

    println!("Number of solved problems: {}", num_solved_problems);
    println!("Sum of solved problems: {}", solved_problems_sum);
}

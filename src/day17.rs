use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use colored::Colorize;
use itertools::Itertools;
use regex::Regex;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day17.txt"));

    if part_opt.is_none() {
        println!("Running Day 17, Part 1");
        part1(&input);
        println!();
        println!("Running Day 17, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 17, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 17, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, Clone)]
struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    instr_ptr: usize, // Instruction Pointer
    program: Vec<u8>,
}

impl Computer {
    fn from_file(input_file: &PathBuf) -> Self {
        let input_file = File::open(input_file).expect(
            format!(
                "Could not open input file: {}",
                input_file.to_string_lossy()
            )
            .as_str(),
        );
        let lines = BufReader::new(input_file).lines();

        let mut reg_a: Option<i64> = None;
        let mut reg_b: Option<i64> = None;
        let mut reg_c: Option<i64> = None;
        let mut program: Option<Vec<u8>> = None;

        let register_a_regex = Regex::new(r"Register A: (\d+)").unwrap();
        let register_b_regex = Regex::new(r"Register B: (\d+)").unwrap();
        let register_c_regex = Regex::new(r"Register C: (\d+)").unwrap();
        let program_regex = Regex::new(r"Program: ([\d,]+)").unwrap();

        for line in lines {
            let line = line.expect("Could not read line.");

            if let Some(register_a_captures) = register_a_regex.captures(&line) {
                reg_a = Some(
                    register_a_captures
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                );
                continue;
            }

            if let Some(register_b_captures) = register_b_regex.captures(&line) {
                reg_b = Some(
                    register_b_captures
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                );
                continue;
            }

            if let Some(register_c_captures) = register_c_regex.captures(&line) {
                reg_c = Some(
                    register_c_captures
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                );
                continue;
            }

            if let Some(program_captures) = program_regex.captures(&line) {
                let program_str = program_captures.get(1).unwrap().as_str();
                program = Some(
                    program_str
                        .split(',')
                        .map(|c| c.to_string().parse::<u8>().unwrap())
                        .collect(),
                );
            }
        }

        return Self {
            reg_a: reg_a.expect("Register A not found."),
            reg_b: reg_b.expect("Register B not found."),
            reg_c: reg_c.expect("Register C not found."),
            instr_ptr: 0,
            program: program.expect("Program not found."),
        };
    }

    fn process_one_instruction(&mut self) -> (bool, Option<u8>) {
        // Returns (exited, output)
        if self.instr_ptr >= self.program.len() {
            return (true, None);
        }

        let opcode = self.program[self.instr_ptr];
        let operand = self.program[self.instr_ptr + 1];
        self.instr_ptr += 2;

        let output = match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => unreachable!("Unexpected opcode: {}", opcode),
        };

        return (false, output);
    }

    fn adv(&mut self, operand: u8) -> Option<u8> {
        let resolved_operand = self.resolve_combo_operand(operand);
        // print!("adv({} -> {})", operand, resolved_operand);
        if resolved_operand < 0 {
            panic!(
                "Attempting to raise to negative power: {}",
                resolved_operand
            );
        }

        let numerator = self.reg_a;
        self.reg_a = numerator >> resolved_operand;
        // println!(" reg_a <- {}", self.reg_a);
        return None;
    }

    fn bxl(&mut self, operand: u8) -> Option<u8> {
        let operand = operand as i64;
        // print!("bxl({})", operand);
        self.reg_b = self.reg_b ^ operand;
        // println!(" reg_b <- {}", self.reg_b);
        return None;
    }

    fn bst(&mut self, operand: u8) -> Option<u8> {
        let resolved_operand = self.resolve_combo_operand(operand);
        // print!("bst({} -> {})", operand, resolved_operand);
        self.reg_b = resolved_operand & 0b0111;
        // println!(" reg_b <- {}", self.reg_b);
        return None;
    }

    fn jnz(&mut self, operand: u8) -> Option<u8> {
        // print!("jnz({})", operand);
        if self.reg_a == 0 {
            // println!();
            return None;
        }
        self.instr_ptr = operand as usize;
        // println!(" instr_ptr <- {}", self.instr_ptr);
        return None;
    }

    fn bxc(&mut self, _: u8) -> Option<u8> {
        // print!("bxc()");
        self.reg_b = self.reg_b ^ self.reg_c;
        // println!(" reg_b <- {}", self.reg_b);
        return None;
    }

    fn out(&mut self, operand: u8) -> Option<u8> {
        let resolved_operand = self.resolve_combo_operand(operand);
        // println!(
        //     "out({} -> {}) -> {}",
        //     operand,
        //     resolved_operand,
        //     resolved_operand & 0b0111
        // );

        return Some((resolved_operand & 0b0111) as u8);
    }

    fn bdv(&mut self, operand: u8) -> Option<u8> {
        let resolved_operand = self.resolve_combo_operand(operand);
        // println!("bdv({} -> {})", operand, resolved_operand);
        if resolved_operand < 0 {
            panic!(
                "Attempting to raise to negative power: {}",
                resolved_operand
            );
        }

        let numerator = self.reg_a;
        self.reg_b = numerator >> resolved_operand;
        // println!(" reg_b <- {}", self.reg_b);
        return None;
    }

    fn cdv(&mut self, operand: u8) -> Option<u8> {
        let resolved_operand = self.resolve_combo_operand(operand);
        // print!("cdv({} -> {})", operand, resolved_operand);
        if resolved_operand < 0 {
            panic!(
                "Attempting to raise to negative power: {}",
                resolved_operand
            );
        }

        let numerator = self.reg_a;
        self.reg_c = numerator >> resolved_operand;
        // println!(" reg_c <- {}", self.reg_c);
        return None;
    }

    fn resolve_combo_operand(&self, operand: u8) -> i64 {
        return match operand {
            0..=3 => operand as i64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!("Invalid unresolved operand: {}", operand),
        };
    }
}

fn part1(input_file: &PathBuf) {
    let mut computer = Computer::from_file(input_file);
    println!("Program: {:?}", computer.program);

    let mut outputs: Vec<u8> = Vec::new();
    loop {
        // println!("{:?}", computer);
        let (exited, out) = computer.process_one_instruction();
        if out.is_some() {
            outputs.push(out.unwrap());
        }
        if exited {
            break;
        }
    }

    let t: String =
        Itertools::intersperse(outputs.iter().map(|n| n.to_string()), ",".to_string()).collect();
    println!("Output: {}", t.green().bold());
}

fn part2(input_file: &PathBuf) {
    let computer = Computer::from_file(input_file);

    let target_output: Vec<u8> = vec![2, 4, 1, 5, 7, 5, 1, 6, 4, 3, 5, 5, 0, 3, 3, 0];

    let mut active_inputs: HashSet<i64> = HashSet::new();
    active_inputs.insert(0);
    let mut found_len = 0;

    for l in 0..target_output.len() {
        let curr_target = &target_output[(target_output.len() - l - 1)..target_output.len()];
        let mut next_inputs: HashSet<i64> = HashSet::new();

        println!();
        println!("curr_target: {:?}", curr_target);

        for active_input in active_inputs.iter() {
            for i in 0..8i64 {
                let mut working_computer = computer.clone();
                let inp = active_input | i;

                working_computer.reg_a = inp;
                let out = collect_output(&mut working_computer);
                // println!("{:?}; {l}", out);
                if out.len() > l && &out[0..=l] == curr_target {
                    println!("{inp:b} is a working configuration (l = {l}). Output: {out:?}");
                    next_inputs.insert(inp << 3);
                    found_len = l + 1;
                }
            }
        }
        println!("Working Inputs: {:#?}", next_inputs);
        active_inputs = next_inputs;
    }

    assert_eq!(found_len, target_output.len());

    let min_solution = active_inputs.iter().map(|i| i >> 3).min().unwrap();
    println!();
    println!(
        "Required reg_a value: {} ({:#b})",
        min_solution.to_string().green().bold(),
        min_solution
    );
}

fn collect_output(computer: &mut Computer) -> Vec<u8> {
    let mut outputs: Vec<u8> = Vec::new();
    loop {
        // println!("{:?}", computer);
        let (exited, out) = computer.process_one_instruction();
        if out.is_some() {
            outputs.push(out.unwrap());
        }
        if exited {
            break;
        }
    }
    return outputs;
}

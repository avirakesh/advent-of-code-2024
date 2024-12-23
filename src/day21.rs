use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    ops::Sub,
    path::PathBuf,
};

use colored::Colorize;
use regex::Regex;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day21.txt"));

    if part_opt.is_none() {
        println!("Running Day 21, Part 1");
        part1(&input);
        println!();
        println!("Running Day 21, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 21, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 21, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        return Self { x, y };
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        return Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Sub<&Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: &Coord) -> Self::Output {
        return Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Sub<&Coord> for &Coord {
    type Output = Coord;

    fn sub(self, rhs: &Coord) -> Self::Output {
        return Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Sub<Coord> for &Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        return Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum NumberPadKey {
    KeyA,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
}

impl NumberPadKey {
    fn from_char(c: char) -> NumberPadKey {
        return match c {
            'A' => NumberPadKey::KeyA,
            '0' => NumberPadKey::Key0,
            '1' => NumberPadKey::Key1,
            '2' => NumberPadKey::Key2,
            '3' => NumberPadKey::Key3,
            '4' => NumberPadKey::Key4,
            '5' => NumberPadKey::Key5,
            '6' => NumberPadKey::Key6,
            '7' => NumberPadKey::Key7,
            '8' => NumberPadKey::Key8,
            '9' => NumberPadKey::Key9,
            _ => panic!("Invalid char: {}", c),
        };
    }

    fn pretty_print(code: &Vec<NumberPadKey>) {
        for key in code.iter() {
            print!(
                "{:<2}",
                match key {
                    NumberPadKey::KeyA => "A",
                    NumberPadKey::Key0 => "0",
                    NumberPadKey::Key1 => "1",
                    NumberPadKey::Key2 => "2",
                    NumberPadKey::Key3 => "3",
                    NumberPadKey::Key4 => "4",
                    NumberPadKey::Key5 => "5",
                    NumberPadKey::Key6 => "6",
                    NumberPadKey::Key7 => "7",
                    NumberPadKey::Key8 => "8",
                    NumberPadKey::Key9 => "9",
                }
            );
        }
    }
}

#[derive(Debug, Clone)]
struct NumberPad {
    hand_position: NumberPadKey,
    key_positions: HashMap<NumberPadKey, Coord>,
}

impl NumberPad {
    fn new() -> Self {
        let key_positions = HashMap::from([
            (NumberPadKey::KeyA, Coord::new(2, 3)),
            (NumberPadKey::Key0, Coord::new(1, 3)),
            (NumberPadKey::Key1, Coord::new(0, 2)),
            (NumberPadKey::Key2, Coord::new(1, 2)),
            (NumberPadKey::Key3, Coord::new(2, 2)),
            (NumberPadKey::Key4, Coord::new(0, 1)),
            (NumberPadKey::Key5, Coord::new(1, 1)),
            (NumberPadKey::Key6, Coord::new(2, 1)),
            (NumberPadKey::Key7, Coord::new(0, 0)),
            (NumberPadKey::Key8, Coord::new(1, 0)),
            (NumberPadKey::Key9, Coord::new(2, 0)),
        ]);

        return NumberPad {
            hand_position: NumberPadKey::KeyA,
            key_positions,
        };
    }

    fn move_hand_to(&mut self, key: &NumberPadKey) -> Vec<Vec<DirectionPadKey>> {
        if self.hand_position == *key {
            return vec![vec![DirectionPadKey::A]];
        }

        let curr_pos = self.key_positions.get(&self.hand_position).unwrap();
        let target_pos = self.key_positions.get(key).unwrap();

        let diff = target_pos - curr_pos;
        let horz_key: Option<DirectionPadKey> = match diff.x {
            0 => None,
            x if x > 0 => Some(DirectionPadKey::Right),
            _ => Some(DirectionPadKey::Left),
        };

        let vert_key = match diff.y {
            0 => None,
            y if y > 0 => Some(DirectionPadKey::Down),
            _ => Some(DirectionPadKey::Up),
        };

        let mut possible_directions: Vec<Vec<DirectionPadKey>> = Vec::new();

        if horz_key.is_none() {
            possible_directions.push(
                vec![vert_key.unwrap(); diff.y.abs() as usize]
                    .into_iter()
                    .chain(iter::once(DirectionPadKey::A))
                    .collect(),
            );
            self.hand_position = *key;
            return possible_directions;
        }

        if vert_key.is_none() {
            possible_directions.push(
                vec![horz_key.unwrap(); diff.x.abs() as usize]
                    .into_iter()
                    .chain(iter::once(DirectionPadKey::A))
                    .collect(),
            );
            self.hand_position = *key;
            return possible_directions;
        }

        let vert_vec = vec![vert_key.unwrap(); diff.y.abs() as usize];
        let horz_vec = vec![horz_key.unwrap(); diff.x.abs() as usize];

        // Check to ensure that going across first won't hit the invalid space
        if !(curr_pos.y == 3 && target_pos.x == 0) {
            // Go across, then go up/down
            possible_directions.push(
                horz_vec
                    .iter()
                    .chain(vert_vec.iter())
                    .chain(iter::once(&DirectionPadKey::A))
                    .map(|d| *d)
                    .collect(),
            );
        }

        // Check to ensure that going up/down first won't hit the invalid space
        if !(curr_pos.x == 0 && target_pos.y == 3) {
            // Go up/down, then go across
            possible_directions.push(
                vert_vec
                    .iter()
                    .chain(horz_vec.iter())
                    .chain(iter::once(&DirectionPadKey::A))
                    .map(|d| *d)
                    .collect(),
            );
        }

        self.hand_position = *key;
        return possible_directions;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum DirectionPadKey {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl DirectionPadKey {
    fn pretty_print(code: &Vec<DirectionPadKey>) {
        for key in code {
            print!(
                "{:<2}",
                match key {
                    DirectionPadKey::Up => "↑",
                    DirectionPadKey::Down => "↓",
                    DirectionPadKey::Left => "←",
                    DirectionPadKey::Right => "→",
                    DirectionPadKey::A => "A",
                }
            )
        }
    }
}

struct DirectionPad {
    hand_position: DirectionPadKey,
    key_positions: HashMap<DirectionPadKey, Coord>,
}

impl DirectionPad {
    fn new() -> Self {
        let key_positions = HashMap::from([
            (DirectionPadKey::A, Coord::new(2, 0)),
            (DirectionPadKey::Up, Coord::new(1, 0)),
            (DirectionPadKey::Down, Coord::new(1, 1)),
            (DirectionPadKey::Left, Coord::new(0, 1)),
            (DirectionPadKey::Right, Coord::new(2, 1)),
        ]);

        return DirectionPad {
            hand_position: DirectionPadKey::A,
            key_positions,
        };
    }

    fn move_hand_to(&mut self, key: &DirectionPadKey) -> Vec<Vec<DirectionPadKey>> {
        if self.hand_position == *key {
            return vec![vec![DirectionPadKey::A]];
        }

        let curr_pos = self.key_positions.get(&self.hand_position).unwrap();
        let target_pos = self.key_positions.get(key).unwrap();

        let diff = target_pos - curr_pos;
        let horz_key: Option<DirectionPadKey> = match diff.x {
            0 => None,
            x if x > 0 => Some(DirectionPadKey::Right),
            _ => Some(DirectionPadKey::Left),
        };

        let vert_key = match diff.y {
            0 => None,
            y if y > 0 => Some(DirectionPadKey::Down),
            _ => Some(DirectionPadKey::Up),
        };

        let mut possible_directions: Vec<Vec<DirectionPadKey>> = Vec::new();

        if horz_key.is_none() {
            possible_directions.push(
                vec![vert_key.unwrap(); diff.y.abs() as usize]
                    .into_iter()
                    .chain(iter::once(DirectionPadKey::A))
                    .collect(),
            );
            self.hand_position = *key;
            return possible_directions;
        }

        if vert_key.is_none() {
            possible_directions.push(
                vec![horz_key.unwrap(); diff.x.abs() as usize]
                    .into_iter()
                    .chain(iter::once(DirectionPadKey::A))
                    .collect(),
            );
            self.hand_position = *key;
            return possible_directions;
        }

        let vert_vec = vec![vert_key.unwrap(); diff.y.abs() as usize];
        let horz_vec = vec![horz_key.unwrap(); diff.x.abs() as usize];

        // Check to ensure that going across first won't hit the invalid space
        if !(curr_pos.y == 0 && target_pos.x == 0) {
            // Go across, then go up/down
            possible_directions.push(
                horz_vec
                    .iter()
                    .chain(vert_vec.iter())
                    .chain(iter::once(&DirectionPadKey::A))
                    .map(|d| *d)
                    .collect(),
            );
        }

        // Check to ensure that going up/down first won't hit the invalid space
        if !(curr_pos.x == 0 && target_pos.y == 0) {
            // Go up/down, then go across
            possible_directions.push(
                vert_vec
                    .iter()
                    .chain(horz_vec.iter())
                    .chain(iter::once(&DirectionPadKey::A))
                    .map(|d| *d)
                    .collect(),
            );
        }

        self.hand_position = *key;
        return possible_directions;
    }
}

fn read_code_from_file(input_file: &PathBuf) -> Vec<(Vec<NumberPadKey>, usize)> {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );
    let lines = BufReader::new(input_file).lines();
    let mut codes: Vec<(Vec<NumberPadKey>, usize)> = Vec::new();

    let non_digits = Regex::new(r"[^0-9]").unwrap();
    for line in lines {
        let line = line.expect("Could not read line");
        let numeric_part = non_digits.replace_all(line.as_str(), "").to_string();
        let numeric_part: usize = numeric_part.parse().unwrap();
        codes.push((
            line.chars().map(|c| NumberPadKey::from_char(c)).collect(),
            numeric_part,
        ));
    }

    return codes;
}

fn part1(input_file: &PathBuf) {
    let codes = read_code_from_file(input_file);
    // println!("Codes: {:?}", codes);

    let mut total_complexity: usize = 0;
    for (code, value) in codes {
        print!("Code: ");
        NumberPadKey::pretty_print(&code);
        println!();

        let directions = get_shortest_directions_for_code(&code);
        print!("Dirs: ");
        DirectionPadKey::pretty_print(&directions);
        println!();

        let complexity = value * directions.len();
        println!(
            "Complexity: {} x {} = {}",
            value,
            directions.len(),
            complexity
        );

        total_complexity += complexity;
        println!();
    }

    println!(
        "Total Complexity: {}",
        total_complexity.to_string().green().bold()
    );
}

fn get_shortest_directions_for_code(code: &[NumberPadKey]) -> Vec<DirectionPadKey> {
    let mut number_pad = NumberPad::new();
    let mut shortest_input: Vec<DirectionPadKey> = Vec::new();

    for key in code {
        let directions = number_pad.move_hand_to(key);
        if directions.is_empty() {
            // No movement needed, just push A!
            shortest_input.push(DirectionPadKey::A);
        }

        let shortest_d_pad_input = directions
            .iter()
            .map(|d| get_shortest_d_pad_directions(d, 2))
            .min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap();

        shortest_input.extend(shortest_d_pad_input.into_iter());
    }

    return shortest_input;
}

fn get_shortest_d_pad_directions(code: &[DirectionPadKey], level: usize) -> Vec<DirectionPadKey> {
    if level == 0 {
        return code.iter().map(|d| *d).collect();
    }

    let mut d_pad = DirectionPad::new();
    let mut curr_best_min: Vec<DirectionPadKey> = Vec::new();

    for key in code {
        let options = d_pad.move_hand_to(key);

        let min_option = options
            .iter()
            .map(|o| get_shortest_d_pad_directions(&o, level - 1))
            .min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap();

        curr_best_min.extend(min_option.into_iter());
    }

    return curr_best_min;
}

fn part2(input_file: &PathBuf) {
    todo!("Implement Part2")
}

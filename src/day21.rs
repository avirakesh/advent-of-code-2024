use std::{collections::HashMap, path::PathBuf};

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

#[derive(Debug)]
struct NumberPad {
    hand_position: Coord,
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
            (NumberPadKey::Key4, Coord::new(1, 1)),
            (NumberPadKey::Key5, Coord::new(2, 1)),
            (NumberPadKey::Key6, Coord::new(3, 1)),
            (NumberPadKey::Key7, Coord::new(1, 0)),
            (NumberPadKey::Key8, Coord::new(2, 0)),
            (NumberPadKey::Key9, Coord::new(3, 0)),
        ]);

        return NumberPad {
            hand_position: *key_positions.get(&NumberPadKey::KeyA).unwrap(),
            key_positions,
        };
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

struct DirectionPad {
    hand_position: Coord,
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
            hand_position: *key_positions.get(&DirectionPadKey::A).unwrap(),
            key_positions,
        };
    }
}

fn part1(input_file: &PathBuf) {
    todo!("Implement Part1")
}

fn part2(input_file: &PathBuf) {
    todo!("Implement Part2")
}

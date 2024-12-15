use std::path::PathBuf;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day15.txt"));

    if part_opt.is_none() {
        println!("Running Day 15, Part 1");
        part1(&input);
        println!();
        println!("Running Day 15, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 15, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 15, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Coord { x, y }
    }
}

enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn get_d_coord(&self) -> Coord {
        match self {
            Facing::Up => Coord::new(0, -1),
            Facing::Down => Coord::new(0, 1),
            Facing::Left => Coord::new(-1, 0),
            Facing::Right => Coord::new(1, 0),
        }
    }
}

enum Entity {
    Wall,
    Box,
    Robot,
    None,
}

fn part1(input_file: &PathBuf) {
    todo!("Implement Part1");
}

fn part2(input_file: &PathBuf) {
    todo!("Implement Part2");
}

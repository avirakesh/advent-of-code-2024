use std::{
    collections::HashMap,
    fs::File,
    io::{stdin, BufRead, BufReader},
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Rem},
    path::PathBuf,
};

use colored::Colorize;
use regex::Regex;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day14.txt"));

    if part_opt.is_none() {
        println!("Running Day 14, Part 1");
        part1(&input);
        println!();
        println!("Running Day 14, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 14, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 14, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    fn from_pair(xy: (i32, i32)) -> Self {
        Coord::new(xy.0, xy.1)
    }

    fn wrap_to_size(&self, size: &(i32, i32)) -> Coord {
        return Coord::new(self.x.rem_euclid(size.0), self.y.rem_euclid(size.1));
    }
}

impl Mul<i32> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i32) -> Self::Output {
        return Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl MulAssign<i32> for Coord {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        return Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Index<Coord> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        return &self[index.y as usize][index.x as usize];
    }
}

impl<T> IndexMut<Coord> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        return &mut self[index.y as usize][index.x as usize];
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Robot {
    position: Coord,
    velocity: Coord,
}

impl Robot {
    fn from_string(line: &String) -> Self {
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

        let captures = re
            .captures(line)
            .expect(format!("Line '{}' does not match expected format!", line).as_str());

        let position = Coord::new(captures[1].parse().unwrap(), captures[2].parse().unwrap());
        let velocity = Coord::new(captures[3].parse().unwrap(), captures[4].parse().unwrap());

        return Self { position, velocity };
    }

    fn move_for_n_secs(&mut self, n_secs: i32, board_size: &(i32, i32)) {
        let new_pos = self.position + (self.velocity * n_secs);
        self.position = new_pos.wrap_to_size(board_size);
    }
}

fn part1(input_file: &PathBuf) {
    let mut robots = create_robots_from_file(input_file);
    println!("# Robots: {:#?}", robots.len());
    let board_size = (101, 103);

    println!("Starting State:");
    pretty_print_robot_count(&robots, &board_size);
    println!();

    let num_secs = 100;
    robots
        .iter_mut()
        .for_each(|r| r.move_for_n_secs(num_secs, &board_size));
    // println!("{:#?}", robots);

    println!("State after {}s:", num_secs);
    pretty_print_robot_count(&robots, &board_size);
    println!();

    let num_robots = count_robots_in_quadrants(&robots, &board_size);
    println!("Counts per quadrant: {:?}", num_robots);

    let safety_factor = num_robots.0 * num_robots.1 * num_robots.2 * num_robots.3;
    println!(
        "Safety Factor: {}",
        safety_factor.to_string().as_str().green().bold()
    );
}

fn create_robots_from_file(input_file: &PathBuf) -> Vec<Robot> {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );

    let lines = BufReader::new(input_file).lines();

    return lines
        .into_iter()
        .map(|l| l.expect("Could not read line"))
        .map(|l| Robot::from_string(&l))
        .collect();
}

fn count_robots_in_quadrants(robots: &Vec<Robot>, board_size: &(i32, i32)) -> (i32, i32, i32, i32) {
    let (width, height) = *board_size;

    let middle_x = width / 2;
    let middle_y = height / 2;

    let count_middle_x = width % 2 == 0;
    let count_middle_y = height % 2 == 0;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for robot in robots.iter() {
        let robot_x = robot.position.x;
        let robot_y = robot.position.y;

        if (!count_middle_x && robot_x == middle_x) || (!count_middle_y && robot_y == middle_y) {
            continue;
        }

        if robot_x < middle_x && robot_y < middle_y {
            top_left += 1;
        } else if robot_x < middle_x && robot_y >= middle_y {
            bottom_left += 1;
        } else if robot_x >= middle_x && robot_y < middle_y {
            top_right += 1;
        } else {
            bottom_right += 1;
        }
    }

    return (top_left, top_right, bottom_left, bottom_right);
}

fn get_average_count_per_cell(robots: &Vec<Robot>) -> f64 {
    let counts: HashMap<Coord, i32> =
        robots
            .iter()
            .map(|r| r.position)
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });
    let num_robots = robots.len();
    let num_positions = counts.len();
    return num_robots as f64 / num_positions as f64;
}

fn pretty_print_robot_count(robots: &Vec<Robot>, board_size: &(i32, i32)) {
    let (width, height) = *board_size;
    let mut board = vec![vec![0; width as usize]; height as usize];

    robots
        .iter()
        .map(|r| r.position)
        .for_each(|c| board[c] += 1);

    for board_row in board.iter() {
        for count in board_row.iter() {
            print!(
                "{:<2}",
                match (count) {
                    0 => ".".black(),
                    _ => format!("{}", count).as_str().green(),
                }
            );
        }
        println!();
    }
}

fn part2(input_file: &PathBuf) {
    let mut robots = create_robots_from_file(input_file);
    println!("# Robots: {:#?}", robots.len());
    let board_size = (101, 103);

    let mut user_input = String::new();
    let mut num_seconds = 0;

    println!("Board After {}s", num_seconds);
    pretty_print_robot_count(&robots, &board_size);
    println!();

    let avg_threshold = 1.001;
    loop {
        let avg_robots = get_average_count_per_cell(&robots);
        if avg_robots <= avg_threshold {
            println!("Board After {}s", num_seconds);
            pretty_print_robot_count(&robots, &board_size);
            println!();
            println!("Board has avergage robots count: {}", avg_robots);
            println!("Press Enter to continue (or exit to exit): ");
            stdin().read_line(&mut user_input).unwrap();
            if user_input.to_lowercase() == "exit" {
                break;
            }
        }

        num_seconds += 1;
        robots
            .iter_mut()
            .for_each(|r| r.move_for_n_secs(1, &board_size));
    }
}

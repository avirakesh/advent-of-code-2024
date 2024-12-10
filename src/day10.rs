use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, IndexMut},
    path::PathBuf,
};

use colored::Colorize;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day10.txt"));

    if part_opt.is_none() {
        println!("Running Day 10, Part 1");
        part1(&input);
        println!();
        println!("Running Day 10, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 10, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 10, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Index<Point> for Vec<Vec<u8>> {
    type Output = u8;

    fn index(&self, index: Point) -> &Self::Output {
        &self[index.y][index.x]
    }
}

impl IndexMut<Point> for Vec<Vec<u8>> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self[index.y][index.x]
    }
}

impl Index<Point> for Vec<Vec<u32>> {
    type Output = u32;

    fn index(&self, index: Point) -> &Self::Output {
        &self[index.y][index.x]
    }
}

impl IndexMut<Point> for Vec<Vec<u32>> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self[index.y][index.x]
    }
}

#[derive(Debug)]
struct State {
    board: Vec<Vec<u8>>,
    trail_score: Vec<Vec<u32>>,
}

impl State {
    fn from_file(input_file: &PathBuf) -> Self {
        let input_file = File::open(input_file).expect(
            format!(
                "Could not open input file: {}",
                input_file.to_string_lossy()
            )
            .as_str(),
        );

        let lines = BufReader::new(input_file).lines();

        let board: Vec<Vec<u8>> = lines
            .map(|line| {
                line.expect("Could not read line")
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        let trail_score: Vec<Vec<u32>> = vec![vec![0; board[0].len()]; board.len()];

        return State { board, trail_score };
    }

    fn update_all_trail_scores_part1(&mut self) {
        let nines = self.get_all_9_positions();

        for pos in nines {
            self.update_part1_trail_scores_for_nine(pos);
        }
    }

    fn update_part1_trail_scores_for_nine(&mut self, pos: Point) {
        let mut visited: HashSet<Point> = HashSet::new();

        let mut active_nodes: Vec<Point> = vec![pos];

        while !active_nodes.is_empty() {
            let node = active_nodes.pop().unwrap();
            self.trail_score[node] += 1;
            visited.insert(node);

            let curr_level = self.board[node];

            let neighbors = self.get_neighbor_pos(node);
            for neighbor in neighbors {
                let neighbor_level = self.board[neighbor];
                if neighbor_level + 1 != curr_level {
                    continue;
                }

                if visited.contains(&neighbor) {
                    continue;
                }

                active_nodes.push(neighbor);
            }
        }
    }

    fn get_neighbor_pos(&self, pos: Point) -> Vec<Point> {
        let mut ret = Vec::new();
        if pos.x > 0 {
            ret.push(Point::new(pos.x - 1, pos.y));
        }

        if pos.y > 0 {
            ret.push(Point::new(pos.x, pos.y - 1));
        }

        if pos.x < self.board[0].len() - 1 {
            ret.push(Point::new(pos.x + 1, pos.y));
        }

        if pos.y < self.board.len() - 1 {
            ret.push(Point::new(pos.x, pos.y + 1));
        }

        return ret;
    }

    fn get_all_9_positions(&self) -> Vec<Point> {
        return self
            .board
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, v)| **v == 9)
                    .map(move |(x, _)| Point::new(x, y))
            })
            .collect();
    }

    fn calculate_trailhead_scores(&self) -> u32 {
        return self
            .board
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, v)| **v == 0)
                    .map(move |(x, _)| (x, y))
            })
            .map(|(x, y)| self.trail_score[y][x])
            .sum();
    }

    fn update_all_trail_scores_part2(&mut self) {
        let nines = self.get_all_9_positions();
        for pos in nines.iter() {
            self.trail_score[*pos] += 1;
        }

        let mut active_nodes: HashMap<Point, u32> = HashMap::new();
        for pos in nines.iter() {
            active_nodes.insert(*pos, 1);
        }

        while !active_nodes.is_empty() {
            let mut next_active_nodes: HashMap<Point, u32> = HashMap::new();
            for (pos, score) in active_nodes.iter() {
                let curr_level = self.board[*pos];
                self.trail_score[*pos] = *score;
                let neighbors = self.get_neighbor_pos(*pos);
                for n in neighbors {
                    let neighbor_level = self.board[n];
                    if neighbor_level + 1 != curr_level {
                        continue;
                    }
                    *next_active_nodes.entry(n).or_insert(0) += score;
                }
            }
            active_nodes = next_active_nodes;
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Board:")?;
        for (y, row) in self.board.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                match self.board[y][x] {
                    0 => write!(f, "{:<3}", format!("{}", val).as_str().green())?,
                    9 => write!(f, "{:<3}", format!("{}", val).as_str().red())?,
                    _ => write!(f, "{:<3}", val)?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        writeln!(f, "Trail Scores:")?;
        for (y, row) in self.trail_score.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                match self.board[y][x] {
                    0 => write!(f, "{:<3}", format!("{}", val).as_str().green())?,
                    9 => write!(f, "{:<3}", format!("{}", val).as_str().red())?,
                    _ => write!(f, "{:<3}", val)?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        return Ok(());
    }
}

fn part1(input_file: &PathBuf) {
    let mut state = State::from_file(input_file);
    println!("{}", state);

    state.update_all_trail_scores_part1();
    println!("{}", state);

    println!(
        "Trailhead Score: {}",
        format!("{}", state.calculate_trailhead_scores())
            .as_str()
            .green()
            .bold()
    );
}

fn part2(input_file: &PathBuf) {
    let mut state = State::from_file(input_file);
    println!("{}", state);

    state.update_all_trail_scores_part2();
    println!("{}", state);

    println!(
        "Trailhead Score: {}",
        format!("{}", state.calculate_trailhead_scores())
            .as_str()
            .green()
            .bold()
    );
}

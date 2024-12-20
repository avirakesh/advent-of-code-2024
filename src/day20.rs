use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Index, IndexMut, Mul},
    path::PathBuf,
    vec,
};

use colored::Colorize;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day20.txt"));

    if part_opt.is_none() {
        println!("Running Day 20, Part 1");
        part1(&input);
        println!();
        println!("Running Day 20, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 20, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 20, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn is_coord_valid<T>(&self, matrix: &Vec<Vec<T>>) -> bool {
        return self.x >= 0
            && self.y >= 0
            && self.x < matrix[0].len() as isize
            && self.y < matrix.len() as isize;
    }
}

impl<T> Index<Coord> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        return &self[index.y as usize][index.x as usize];
    }
}

impl<T> Index<&Coord> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: &Coord) -> &Self::Output {
        return &self[index.y as usize][index.x as usize];
    }
}

impl<T> IndexMut<Coord> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        return &mut self[index.y as usize][index.x as usize];
    }
}

impl<T> IndexMut<&Coord> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: &Coord) -> &mut Self::Output {
        return &mut self[index.y as usize][index.x as usize];
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<&Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<&Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<usize> for Coord {
    type Output = Coord;

    fn mul(self, rhs: usize) -> Self::Output {
        return Coord::new(self.x * rhs as isize, self.y * rhs as isize);
    }
}

impl Mul<usize> for &Coord {
    type Output = Coord;

    fn mul(self, rhs: usize) -> Self::Output {
        return Coord::new(self.x * rhs as isize, self.y * rhs as isize);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Entity {
    Wall,
    None,
}

impl Entity {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Entity::Wall,
            '.' => Entity::None,
            _ => panic!("Invalid character for entity"),
        }
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Entity::Wall => write!(f, "#"),
            Entity::None => write!(f, "."),
        }
    }
}

struct State {
    race_track: Vec<Vec<Entity>>,
    start_pos: Coord,
    end_pos: Coord,
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

        let mut start_pos: Option<Coord> = None;
        let mut end_pos: Option<Coord> = None;

        let mut race_track: Vec<Vec<Entity>> = Vec::new();
        for (y, line) in lines.enumerate() {
            let line = line.expect("Could not read line");
            let mut row: Vec<Entity> = Vec::new();

            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start_pos = Some(Coord::new(x as isize, y as isize));
                    row.push(Entity::None);
                    continue;
                }

                if c == 'E' {
                    end_pos = Some(Coord::new(x as isize, y as isize));
                    row.push(Entity::None);
                    continue;
                }

                row.push(Entity::from_char(c));
            }
            race_track.push(row);
        }

        return State {
            race_track,
            start_pos: start_pos.expect("Starting position not found!"),
            end_pos: end_pos.expect("Ending Position for found!"),
        };
    }

    fn get_path(&self) -> Vec<Coord> {
        let mut path: Vec<Coord> = Vec::new();

        let mut prev: Option<Coord> = None;
        let mut curr = self.start_pos;

        while curr != self.end_pos {
            // println!("Curr: {:?}", curr);
            path.push(curr);
            let next = self.get_next_in_path(&curr, prev);
            prev = Some(curr);
            curr = next;
        }

        path.push(curr);
        return path;
    }

    fn get_next_in_path(&self, curr: &Coord, prev: Option<Coord>) -> Coord {
        let dirs = vec![
            Coord::new(-1, 0),
            Coord::new(0, -1),
            Coord::new(1, 0),
            Coord::new(0, 1),
        ];

        return dirs
            .iter()
            .map(|d| curr + d)
            .filter(|c| c.is_coord_valid(&self.race_track))
            .filter(|c| prev.is_none() || *c != prev.unwrap())
            .filter(|c| self.race_track[c] == Entity::None)
            .next()
            .expect(
                format!(
                    "Could not find next in path. Curr: {:?}, Prev: {:?}",
                    curr, prev,
                )
                .as_str(),
            );
    }

    fn get_cheatsy_neighbors(&self, curr_pos: &Coord, cheat_step: usize) -> Vec<Coord> {
        let dirs = vec![
            Coord::new(-1, 0),
            Coord::new(0, -1),
            Coord::new(1, 0),
            Coord::new(0, 1),
        ];

        return dirs
            .iter()
            .map(|c| c * cheat_step)
            .map(|c| c + curr_pos)
            .filter(|c| c.is_coord_valid(&self.race_track))
            .filter(|c| self.race_track[c] == Entity::None)
            .collect();
    }

    fn get_cheat_counts(&self, path: &[Coord], cheat_step: usize) -> HashMap<usize, usize> {
        // Returns savings -> count
        let mut savings_count: HashMap<usize, usize> = HashMap::new();

        let coord_to_time: HashMap<Coord, usize> =
            path.iter().enumerate().map(|(i, c)| (*c, i)).collect();

        for (curr_time, coord) in path.iter().enumerate() {
            self.get_cheatsy_neighbors(coord, cheat_step)
                .into_iter()
                .filter(|c| coord_to_time.contains_key(c))
                .map(|c| coord_to_time[&c])
                .filter(|t| *t > curr_time + cheat_step)
                .map(|t| t - curr_time - cheat_step)
                .for_each(|t| *savings_count.entry(t).or_insert(0) += 1);
        }

        return savings_count;
    }

    fn pretty_print(&self) {
        for (y, row) in self.race_track.iter().enumerate() {
            for (x, entity) in row.iter().enumerate() {
                let curr_coord = Coord::new(x as isize, y as isize);
                if curr_coord == self.start_pos {
                    print!("{:<2}", "◯".bright_white().bold());
                    continue;
                }

                if curr_coord == self.end_pos {
                    print!("{:<2}", "◉".green().bold());
                    continue;
                }

                print!(
                    "{:<2}",
                    match entity {
                        Entity::Wall => entity.to_string().red(),
                        Entity::None => entity.to_string().bright_black(),
                    }
                );
            }
            println!();
        }
        println!();
    }
}

fn part1(input_file: &PathBuf) {
    let state = State::from_file(input_file);
    state.pretty_print();

    let path = state.get_path();
    println!("Path length: {}", path.len());

    let savings_count = state.get_cheat_counts(&path, 2);
    let mut savings: Vec<(usize, usize)> = savings_count.iter().map(|(k, v)| (*k, *v)).collect();
    savings.sort_by(|(a, _), (b, _)| a.cmp(b));

    // println!();
    // println!("Possible Savings: ");
    // savings.iter().for_each(|(k, v)| {
    //     println!("    {:>3} -> {}", k, v);
    // });

    let optimal_saving_threshold: usize = 100;

    let optimal_savings_count: usize = savings_count
        .iter()
        .filter(|(t, _)| **t >= optimal_saving_threshold)
        .map(|(_, count)| count)
        .sum();

    println!();
    println!(
        "Number of savings >= {}ps : {}",
        optimal_saving_threshold.to_string().yellow(),
        optimal_savings_count.to_string().green().bold()
    );
}

fn part2(input_file: &PathBuf) {
    todo!("Implement part 2");
}

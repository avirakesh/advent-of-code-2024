use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Index, IndexMut},
    path::PathBuf,
};

use colored::Colorize;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day18.txt"));

    if part_opt.is_none() {
        println!("Running Day 18, Part 1");
        part1(&input);
        println!();
        println!("Running Day 18, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 18, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 18, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        return Self { x, y };
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Add<&Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
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

#[derive(Debug, Clone)]
struct Memory {
    corruption_map: Vec<Vec<bool>>,
    start_pos: Coord,
    end_pos: Coord,
    size: (usize, usize),
}

impl Memory {
    fn from_file(
        input_file: &PathBuf,
        board_size: (usize, usize),
        bytes_to_take: Option<usize>,
    ) -> Self {
        let input_file = File::open(input_file).expect(
            format!(
                "Could not open input file: {}",
                input_file.to_string_lossy()
            )
            .as_str(),
        );
        let lines = BufReader::new(input_file).lines();

        let mut corruption_map = vec![vec![false; board_size.1]; board_size.0];
        let mut bytes_processed: usize = 0;
        let byted_to_process = bytes_to_take.unwrap_or(usize::MAX);
        for line in lines {
            if bytes_processed >= byted_to_process {
                break;
            }

            let line = line.expect("Could not read line.");

            let parts = line.split(",").collect::<Vec<&str>>();
            let x = parts[0].parse::<isize>().unwrap();
            let y = parts[1].parse::<isize>().unwrap();

            corruption_map[Coord::new(x, y)] = true;
            bytes_processed += 1;
        }

        let start_pos = Coord::new(0, 0);
        let end_pos = Coord::new(board_size.0 as isize - 1, board_size.1 as isize - 1);
        let size = board_size;

        return Self {
            corruption_map,
            start_pos,
            end_pos,
            size,
        };
    }

    fn from_raw_parts(board_size: (usize, usize), corrupted_coords: &[Coord]) -> Self {
        let mut corruption_map = vec![vec![false; board_size.1]; board_size.0];
        for coord in corrupted_coords {
            corruption_map[*coord] = true;
        }

        return Self {
            corruption_map,
            start_pos: Coord::new(0, 0),
            end_pos: Coord::new(board_size.0 as isize - 1, board_size.1 as isize - 1),
            size: board_size,
        };
    }

    fn get_shortest_path_to_exit(&self) -> Option<Vec<Coord>> {
        // Use a breadth-first search (BFS) to find the shortest path
        let mut prev_nodes: HashMap<Coord, Option<Coord>> = HashMap::new();
        let mut frontier: VecDeque<Coord> = VecDeque::new();

        frontier.push_back(self.start_pos);
        prev_nodes.insert(self.start_pos, None);

        while !frontier.is_empty() {
            let current_coord = frontier.pop_front().unwrap();
            // println!("{:?}", current_coord);

            if current_coord == self.end_pos {
                break;
            }

            let neighbors = self.get_uncorrupted_neighbors(current_coord);
            for neighbor in neighbors {
                if prev_nodes.contains_key(&neighbor) {
                    continue;
                }
                prev_nodes.insert(neighbor, Some(current_coord));
                frontier.push_back(neighbor);
            }
        }

        if !prev_nodes.contains_key(&self.end_pos) {
            return None;
        }

        let mut path: Vec<Coord> = Vec::new();
        let mut current_coord = Some(self.end_pos);
        while let Some(coord) = current_coord {
            path.push(coord);
            current_coord = *prev_nodes.get(&coord).unwrap();
        }

        path.reverse();
        return Some(path);
    }

    fn get_uncorrupted_neighbors(&self, current_coord: Coord) -> Vec<Coord> {
        let neighbor_offsets = vec![
            Coord::new(0, -1),
            Coord::new(0, 1),
            Coord::new(-1, 0),
            Coord::new(1, 0),
        ];

        let mut neighbors: Vec<Coord> = Vec::new();

        for offset in neighbor_offsets.iter() {
            let neighbor_coord = current_coord + offset;
            if neighbor_coord.x < 0
                || neighbor_coord.y < 0
                || neighbor_coord.x >= self.size.0 as isize
                || neighbor_coord.y >= self.size.1 as isize
            {
                continue;
            }

            if !self.corruption_map[neighbor_coord] {
                neighbors.push(neighbor_coord);
            }
        }

        return neighbors;
    }

    fn pretty_print(&self, path: Option<&Vec<Coord>>) {
        let mut path_set: HashSet<Coord> = HashSet::new();
        if let Some(path) = path {
            path_set.extend(path);
        }

        for (y, row) in self.corruption_map.iter().enumerate() {
            for (x, &is_corrupt) in row.iter().enumerate() {
                let curr_coord = Coord::new(x as isize, y as isize);
                if self.start_pos == curr_coord {
                    print!("{:<2}", "◯".bright_white().bold());
                    continue;
                }

                if self.end_pos == curr_coord {
                    print!("{:<2}", "◉".green().bold());
                    continue;
                }

                if path_set.contains(&curr_coord) {
                    print!("{:<2}", "●".bright_cyan());
                    continue;
                }

                if is_corrupt {
                    print!("{:<2}", "#".red());
                } else {
                    print!("{:<2}", ".".bright_black());
                }
            }
            println!();
        }
        println!();
    }
}

fn part1(input_file: &PathBuf) {
    // let board_size = (7 as usize, 7 as usize);
    let board_size = (71 as usize, 71 as usize);
    // let bytes_to_take = 12 as usize;
    let bytes_to_take = 1024 as usize;

    let memory = Memory::from_file(input_file, board_size, Some(bytes_to_take));
    memory.pretty_print(None);

    let path = memory.get_shortest_path_to_exit();
    if path.is_none() {
        panic!("Could not find a path to exit. :(");
    }

    let path = path.unwrap();
    memory.pretty_print(Some(&path));

    println!(
        "Shortest Path Length: {} steps",
        (path.len() - 1).to_string().green().bold()
    );
}

fn part2(input_file: &PathBuf) {
    let corruption_list = coord_list_from_file(input_file);
    // let board_size = (7 as usize, 7 as usize);
    let board_size = (71 as usize, 71 as usize);

    let mut left_limit = 0;
    let mut right_limit = corruption_list.len() - 1;

    while left_limit < right_limit {
        let mid_point = (left_limit + right_limit) / 2;
        // println!(
        //     "left_limit: {}; mid_point: {}; right_limit: {}",
        //     left_limit, mid_point, right_limit
        // );
        let curr_coords = &corruption_list[0..=mid_point];
        let memory = Memory::from_raw_parts(board_size, curr_coords);
        let path = memory.get_shortest_path_to_exit();
        match path {
            Some(_) => {
                left_limit = mid_point + 1;
            }
            None => {
                right_limit = mid_point;
            }
        }
    }

    let last_working_solution = left_limit - 1;
    let coord_list = &corruption_list[0..=last_working_solution];

    let memory = Memory::from_raw_parts(board_size, coord_list);

    let shortest_path = memory.get_shortest_path_to_exit();
    if shortest_path.is_none() {
        panic!("Could not find path in any configuration? WTF?");
    }
    println!("Last feasible path:");
    memory.pretty_print(Some(&shortest_path.unwrap()));

    let first_blocked_solution = left_limit;
    let coord_list = &corruption_list[0..=first_blocked_solution];

    let memory = Memory::from_raw_parts(board_size, coord_list);

    let shortest_path = memory.get_shortest_path_to_exit();
    if shortest_path.is_some() {
        panic!("Found a path through an impossible memory? WTF?");
    }
    println!("First infeasible memory:");
    memory.pretty_print(None);

    let first_blocked_coord = corruption_list[first_blocked_solution];
    println!(
        "First infeasible memory: {}",
        format!("{},{}", first_blocked_coord.x, first_blocked_coord.y)
            .as_str()
            .green()
            .bold()
    );
}

fn coord_list_from_file(input_file: &PathBuf) -> Vec<Coord> {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );
    let lines = BufReader::new(input_file).lines();

    let mut ret: Vec<Coord> = Vec::new();
    for line in lines {
        let line = line.expect("Could not read line.");

        let parts = line.split(",").collect::<Vec<&str>>();
        let x = parts[0].parse::<isize>().unwrap();
        let y = parts[1].parse::<isize>().unwrap();

        ret.push(Coord::new(x, y));
    }

    return ret;
}

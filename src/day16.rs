use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Index},
    path::PathBuf,
};

use colored::Colorize;
use priority_queue::PriorityQueue;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day16.txt"));

    if part_opt.is_none() {
        println!("Running Day 16, Part 1");
        part1(&input);
        println!();
        println!("Running Day 16, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 16, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 16, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
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
        return Coord {
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn d_coord(&self) -> Coord {
        return match self {
            Direction::North => Coord::new(0, -1),
            Direction::South => Coord::new(0, 1),
            Direction::West => Coord::new(-1, 0),
            Direction::East => Coord::new(1, 0),
        };
    }

    fn turn_clockwise(&self) -> Direction {
        return match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        };
    }

    fn turn_counter_clockwise(&self) -> Direction {
        return match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        };
    }

    fn turn_around(&self) -> Direction {
        return match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        };
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Direction::North => write!(f, "{}", "↑"),
            Direction::South => write!(f, "{}", "↓"),
            Direction::West => write!(f, "{}", "←"),
            Direction::East => write!(f, "{}", "→"),
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Entity {
    Wall,
    Empty,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Reindeer {
    pos: Coord,
    facing: Direction,
}

impl Reindeer {
    fn new(pos: Coord, facing: Direction) -> Self {
        return Self { pos, facing };
    }

    fn turn_around(&self) -> Self {
        return Self {
            pos: self.pos,
            facing: self.facing.turn_around(),
        };
    }

    fn possible_reindeer_at_pos(pos: &Coord) -> Vec<Reindeer> {
        return vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .iter()
        .map(|d| Reindeer::new(*pos, *d))
        .collect();
    }
}

impl Display for Reindeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.facing)?;
        Ok(())
    }
}

struct Maze {
    maze: Vec<Vec<Entity>>,
    reindeer: Reindeer,
    end_pos: Coord,
}

impl Maze {
    fn from_file(input_file: &PathBuf) -> Self {
        let input_file = File::open(input_file).expect(
            format!(
                "Could not open input file: {}",
                input_file.to_string_lossy()
            )
            .as_str(),
        );
        let lines = BufReader::new(input_file).lines();

        let mut maze: Vec<Vec<Entity>> = Vec::new();
        let mut end_pos: Option<Coord> = None;
        let mut reindeer: Option<Reindeer> = None;
        for (y, line) in lines.enumerate() {
            let line = line.expect("Could not read line.");

            let mut row: Vec<Entity> = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(if c == '#' {
                    Entity::Wall
                } else {
                    Entity::Empty
                });

                match c {
                    'E' => end_pos = Some(Coord::new(x as isize, y as isize)),
                    'S' => {
                        reindeer = Some(Reindeer {
                            pos: Coord::new(x as isize, y as isize),
                            facing: Direction::East,
                        })
                    }
                    _ => (),
                };
            }
            maze.push(row);
        }

        return Self {
            maze,
            reindeer: reindeer
                .expect("No starting position found. Make sure the input contains 'S'."),
            end_pos: end_pos.expect("No end position found. Make sure the input contains 'E'."),
        };
    }

    fn find_path_of_least_cost(&self) -> (Vec<Reindeer>, isize) {
        let mut active_reindeers: PriorityQueue<Reindeer, Reverse<isize>> = PriorityQueue::new();
        let mut explored_reindeers: HashSet<Reindeer> = HashSet::new();
        // reindeer -> prev reindeer for backtracking
        let mut prev_reindeers: HashMap<Reindeer, Reindeer> = HashMap::new();
        let mut path_cost: Option<isize> = None;

        active_reindeers.push(self.reindeer, Reverse(0 as isize));

        while !active_reindeers.is_empty() {
            let (curr_reindeer, cost) = active_reindeers.pop().unwrap();
            let cost = cost.0;
            explored_reindeers.insert(curr_reindeer);

            if curr_reindeer.pos == self.end_pos {
                path_cost = Some(cost);
                break;
            }

            let next_reindeers = self.get_next_possible_reindeers(&curr_reindeer, cost);
            for (next_reindeer, next_cost) in next_reindeers {
                if explored_reindeers.contains(&next_reindeer)
                    || explored_reindeers.contains(&next_reindeer.turn_around())
                {
                    continue;
                }

                let old_cost = active_reindeers.push_increase(next_reindeer, Reverse(next_cost));
                if old_cost.is_none() || old_cost.unwrap().0 > next_cost {
                    prev_reindeers.insert(next_reindeer, curr_reindeer);
                }
            }
        }

        let final_reindeer = Reindeer::possible_reindeer_at_pos(&self.end_pos)
            .into_iter()
            .filter(|r| explored_reindeers.contains(r))
            .next()
            .expect("Could not find a path to the exit :(");

        let mut path: Vec<Reindeer> = Vec::new();
        let mut active_reindeer = final_reindeer;
        while active_reindeer != self.reindeer {
            path.push(active_reindeer);
            active_reindeer = *prev_reindeers
                .get(&active_reindeer)
                .expect(format!("Could not find parent of {:?}", active_reindeer).as_str());
        }
        path.push(active_reindeer);

        path.reverse();
        return (path, path_cost.unwrap());
    }

    fn find_all_possible_paths_of_least_cost(&self) -> HashMap<Reindeer, Vec<Reindeer>> {
        let mut active_reindeers: PriorityQueue<Reindeer, Reverse<isize>> = PriorityQueue::new();
        let mut explored_reindeers: HashSet<Reindeer> = HashSet::new();
        // reindeer -> prev_reindeers for backtracking
        let mut prev_reindeers: HashMap<Reindeer, Vec<Reindeer>> = HashMap::new();
        let mut least_cost: Option<isize> = None;

        active_reindeers.push(self.reindeer, Reverse(0 as isize));

        while !active_reindeers.is_empty() {
            let (curr_reindeer, cost) = active_reindeers.pop().unwrap();
            let cost = cost.0;
            if least_cost.is_some() && least_cost.unwrap() < cost {
                // exit early if the cost is greater than the path of least cost
                break;
            }

            if curr_reindeer.pos == self.end_pos {
                println!("Found path of cost: {}", cost);
                least_cost = Some(cost);
            }

            explored_reindeers.insert(curr_reindeer);

            let next_reindeers = self.get_next_possible_reindeers(&curr_reindeer, cost);
            for (next_reindeer, next_cost) in next_reindeers {
                if explored_reindeers.contains(&next_reindeer)
                    || explored_reindeers.contains(&next_reindeer.turn_around())
                {
                    continue;
                }

                let old_cost = active_reindeers.push_increase(next_reindeer, Reverse(next_cost));
                if old_cost.is_none() || old_cost.unwrap().0 > next_cost {
                    prev_reindeers.insert(next_reindeer, vec![curr_reindeer]);
                } else if old_cost.unwrap().0 == next_cost {
                    prev_reindeers
                        .get_mut(&next_reindeer)
                        .expect(
                            format!(
                                "{} is in active_reindeers but not in prev_reindeers?",
                                next_reindeer
                            )
                            .as_str(),
                        )
                        .push(curr_reindeer);
                }
            }
        }

        return prev_reindeers;
    }

    fn get_next_possible_reindeers(
        &self,
        curr_reindeer: &Reindeer,
        curr_cost: isize,
    ) -> Vec<(Reindeer, isize)> {
        let mut next_states: Vec<(Reindeer, isize)> = Vec::new();

        // Check if reindeer can move forward
        // No need for bounds checking because there are always walls at the edges
        let front_coord = curr_reindeer.pos + curr_reindeer.facing.d_coord();
        if self.maze[front_coord] != Entity::Wall {
            let next_cost = curr_cost + 1;
            next_states.push((Reindeer::new(front_coord, curr_reindeer.facing), next_cost));
        }

        // Rotate clockwise if there is no wall
        let new_dir = curr_reindeer.facing.turn_clockwise();
        let next_coord = curr_reindeer.pos + new_dir.d_coord();
        if self.maze[next_coord] != Entity::Wall {
            let next_cost = curr_cost + 1000;
            next_states.push((Reindeer::new(curr_reindeer.pos, new_dir), next_cost));
        }

        // Rotate counter-clockwise if there is no wall
        let new_dir = curr_reindeer.facing.turn_counter_clockwise();
        let next_coord = curr_reindeer.pos + new_dir.d_coord();
        if self.maze[next_coord] != Entity::Wall {
            let next_cost = curr_cost + 1000;
            next_states.push((Reindeer::new(curr_reindeer.pos, new_dir), next_cost));
        }

        return next_states;
    }

    fn pretty_print_path(&self, path: Vec<Reindeer>) {
        let mut path_map: HashMap<Coord, Reindeer> = HashMap::new();
        path.iter().for_each(|r| {
            path_map.insert(r.pos, r.clone());
        });

        for (y, row) in self.maze.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let curr_coord = Coord::new(x as isize, y as isize);

                let is_exit = curr_coord == self.end_pos;
                let is_entry = curr_coord == self.reindeer.pos;
                if path_map.contains_key(&curr_coord) {
                    let str = format!("{}", path_map[&curr_coord]);
                    let str = if is_exit {
                        "⌾".green().bold()
                    } else if is_entry {
                        str.green().bold()
                    } else {
                        str.cyan()
                    };
                    print!("{:<2}", str);
                    continue;
                }

                if self.end_pos == curr_coord {
                    print!("{:<2}", "⌾".white().bold());
                    continue;
                }

                match cell {
                    Entity::Wall => print!("{:<2}", "#".red()),
                    Entity::Empty => print!("{:<2}", ".".bright_black()),
                };
            }
            println!();
        }
    }

    fn count_and_pretty_print_best_seats(
        &self,
        all_paths: &HashMap<Reindeer, Vec<Reindeer>>,
    ) -> usize {
        let mut active_reindeers: VecDeque<Reindeer> =
            Reindeer::possible_reindeer_at_pos(&self.end_pos)
                .into_iter()
                .filter(|r| all_paths.contains_key(r))
                .collect();

        let mut explored_reindeers: HashSet<Reindeer> = HashSet::new();
        explored_reindeers.extend(active_reindeers.iter());

        while !active_reindeers.is_empty() {
            let curr_reindeer = active_reindeers.pop_front().unwrap();

            if !all_paths.contains_key(&curr_reindeer) {
                continue;
            }

            for next_reindeer in all_paths[&curr_reindeer].iter() {
                if explored_reindeers.contains(next_reindeer) {
                    continue;
                }

                active_reindeers.push_back(*next_reindeer);
                explored_reindeers.insert(*next_reindeer);
            }
        }

        let seats_on_path: HashSet<Coord> = explored_reindeers.iter().map(|r| r.pos).collect();

        for (y, row) in self.maze.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let curr_coord = Coord::new(x as isize, y as isize);
                if seats_on_path.contains(&curr_coord) {
                    print!("{:<2}", "•".green().bold());
                    continue;
                }

                match cell {
                    Entity::Wall => print!("{:<2}", "#".red()),
                    Entity::Empty => print!("{:<2}", ".".bright_black()),
                };
            }
            println!();
        }

        return seats_on_path.len();
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_at_exit = self.reindeer.pos == self.end_pos;

        for (y, row) in self.maze.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let curr_coord = Coord::new(x as isize, y as isize);

                if self.reindeer.pos == curr_coord {
                    let str = format!("{:<2}", self.reindeer);
                    write!(
                        f,
                        "{:<2}",
                        if is_at_exit {
                            "⌾".green().bold()
                        } else {
                            str.green().bold()
                        }
                    )?;
                    continue;
                }

                if self.end_pos == curr_coord {
                    write!(f, "{:<2}", "⌾".white().bold())?;
                    continue;
                }

                match cell {
                    Entity::Wall => write!(f, "{:<2}", "#".red())?,
                    Entity::Empty => write!(f, "{:<2}", ".".bright_black())?,
                };
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn part1(input_file: &PathBuf) {
    let maze = Maze::from_file(input_file);
    println!("{}", maze);

    let (path, cost) = maze.find_path_of_least_cost();

    maze.pretty_print_path(path);
    println!("Cost of path: {}", cost.to_string().as_str().green().bold());
}

fn part2(input_file: &PathBuf) {
    let maze = Maze::from_file(input_file);
    println!("{}", maze);

    let all_paths = maze.find_all_possible_paths_of_least_cost();
    let num_seats = maze.count_and_pretty_print_best_seats(&all_paths);
    println!(
        "Number of seats on path: {}",
        num_seats.to_string().as_str().green().bold()
    );
}

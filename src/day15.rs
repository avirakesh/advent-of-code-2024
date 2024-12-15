use std::{
    collections::VecDeque,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign},
    path::PathBuf,
};

use colored::Colorize;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Coord { x, y }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        return Coord::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        return Coord::new(self.x - rhs.x, self.y - rhs.y);
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Coord> for Coord {
    fn sub_assign(&mut self, rhs: Coord) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: &char) -> Self {
        return match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Direction must be one of [^, v, <, >]. Found: {}", c),
        };
    }

    fn get_d_coord(&self) -> Coord {
        match self {
            Direction::Up => Coord::new(0, -1),
            Direction::Down => Coord::new(0, 1),
            Direction::Left => Coord::new(-1, 0),
            Direction::Right => Coord::new(1, 0),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "↑",)?,
            Direction::Down => write!(f, "↓",)?,
            Direction::Left => write!(f, "←",)?,
            Direction::Right => write!(f, "→",)?,
        };
        return Ok(());
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Entity {
    Wall,
    Box,
    Robot,
    None,
}

impl Entity {
    fn from_char(c: &char) -> Self {
        return match *c {
            '@' => Entity::Robot,
            'O' => Entity::Box,
            '#' => Entity::Wall,
            '.' => Entity::None,
            _ => panic!("Entity character must be one of [@, O, #, .]. Found {}", c),
        };
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Wall => write!(f, "{:<2}", "░".red())?,
            Entity::Box => write!(f, "{:<2}", "■".white())?,
            Entity::Robot => write!(f, "{:<2}", "@".bright_green().bold())?,
            Entity::None => write!(f, "{:<2}", ".".bright_black())?,
        }
        return Ok(());
    }
}

#[derive(Debug)]
struct State {
    board: Vec<Vec<Entity>>,
    robot_position: Coord,
    pending_instructions: VecDeque<Direction>,
    processed_instructions: Vec<Direction>,
}

impl State {
    fn from_file(input_file: &PathBuf) -> Self {
        let mut board: Vec<Vec<Entity>> = Vec::new();
        let mut instructions: VecDeque<Direction> = VecDeque::new();

        let input_file = File::open(input_file).expect(
            format!(
                "Could not open input file: {}",
                input_file.to_string_lossy()
            )
            .as_str(),
        );

        let mut is_instructions = false;
        let lines = BufReader::new(input_file).lines();
        for line in lines {
            let line = line.expect("Could not read line.");
            if line.is_empty() {
                is_instructions = true;
                continue;
            }

            if is_instructions {
                instructions.extend(line.chars().map(|c| Direction::from_char(&c)));
            } else {
                board.push(line.chars().map(|c| Entity::from_char(&c)).collect());
            }
        }

        let robot_position = board
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, e)| **e == Entity::Robot)
                    .map(move |(x, _)| Coord::new(x as isize, y as isize))
            })
            .next()
            .expect("Robot not present in input board.");

        return State {
            board,
            robot_position,
            pending_instructions: instructions,
            processed_instructions: Vec::new(),
        };
    }

    fn process_one_instruction(&mut self) -> bool {
        // Returns true if an instruction was processed. False otherwise
        if let Some(dir) = self.pending_instructions.pop_front() {
            self.move_robot_in_dir(&dir);
            self.processed_instructions.push(dir);
            return true;
        }

        return false;
    }

    fn move_robot_in_dir(&mut self, dir: &Direction) {
        let next_feasible_coord = self.get_first_non_box_entity_in_dir(&self.robot_position, dir);

        if self.board[next_feasible_coord] == Entity::Wall {
            return;
        }

        let d_coord = dir.get_d_coord();
        let mut curr_coord = next_feasible_coord;

        while curr_coord != self.robot_position {
            let next_coord = curr_coord - d_coord;
            self.board[curr_coord] = self.board[next_coord];
            curr_coord = next_coord;
        }

        self.board[curr_coord] = Entity::None;
        self.robot_position += d_coord;
    }

    fn get_first_non_box_entity_in_dir(&self, from: &Coord, dir: &Direction) -> Coord {
        let d_coord = dir.get_d_coord();
        let mut next_coord = *from + d_coord;

        // No need for bounds checking as there is always a wall at the edges.
        while self.board[next_coord] == Entity::Box {
            next_coord += d_coord;
        }

        return next_coord;
    }

    fn sum_of_all_box_gps(&self) -> usize {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, e)| e == &&Entity::Box)
                    .map(move |(x, _)| (x, y))
            })
            .map(|(x, y)| (100 * y) + x)
            .sum()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            for entity in row {
                write!(f, "{}", entity)?;
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "Pending Instructions:")?;
        for dir in self.pending_instructions.iter() {
            write!(f, "{}", dir)?;
        }
        writeln!(f)?;

        writeln!(f)?;
        writeln!(f, "Processed Instructions:")?;
        for dir in self.processed_instructions.iter() {
            write!(f, "{}", dir)?;
        }
        writeln!(f)?;

        return Ok(());
    }
}

fn part1(input_file: &PathBuf) {
    let mut state = State::from_file(input_file);
    println!("Initial State:");
    println!("{}", state);

    // let mut user_input: String = String::new();
    // loop {
    //     println!("Press Enter to continue (or exit to exit): ");
    //     stdin().read_line(&mut user_input).unwrap();
    //     if user_input.trim().to_lowercase() == "exit" {
    //         break;
    //     }

    //     if !state.process_one_instruction() {
    //         break;
    //     }
    //     println!("{}", state);
    // }

    while state.process_one_instruction() {}

    println!("Final State:");
    println!("{}", state);

    let gps_sum = state.sum_of_all_box_gps();
    println!(
        "Sum of GPS of all boxes: {}",
        gps_sum.to_string().as_str().green().bold()
    );
}

fn part2(input_file: &PathBuf) {
    todo!("Implement Part2");
}

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day6.txt"));

    if part_opt.is_none() {
        println!("Running Day 6, Part 1");
        part1(&input);
        println!();
        println!("Running Day 6, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 6, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 6, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Unvisited,
    Visited,
    Guard,
    Obstacle,
    AddedObstacle, // Used for pretty printing only, not for logic. Use Obstacle for logic instead.
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Completion {
    OutOfBounds,
    Loop,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    board: Vec<Vec<Cell>>,
    board_size: (usize, usize),                       // (width, height)
    guard_pos: (i32, i32),                            // (x, y): (0, 0) is top left corner of board
    guard_facing: (i32, i32),                         // (dx, dy)
    guard_pos_history: HashSet<(i32, i32)>,           // (pos_x, pos_y)
    guard_history: HashSet<((i32, i32), (i32, i32))>, // ((pos_x, pos_y), (facing_x, facing_y))
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

        let mut board: Vec<Vec<Cell>> = Vec::new();
        let mut guard_pos = (0, 0);
        let guard_facing = (0, -1); // Facing up at start

        for line in lines {
            let line = line.expect("Could not read line");
            let mut row: Vec<Cell> = Vec::new();
            for c in line.chars() {
                match c {
                    '.' => row.push(Cell::Unvisited),
                    '#' => row.push(Cell::Obstacle),
                    '^' => {
                        guard_pos = (row.len() as i32, board.len() as i32);
                        row.push(Cell::Guard);
                    }
                    _ => panic!("Invalid character in input file"),
                }
            }
            board.push(row);
        }

        let board_size = (board[0].len(), board.len());
        let guard_pos_history: HashSet<(i32, i32)> = HashSet::new();
        let guard_history: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

        // Initialize the board with the guard's starting position and facing direction
        return Self {
            board,
            board_size,
            guard_pos,
            guard_facing,
            guard_pos_history,
            guard_history,
        };
    }

    fn next_state(&mut self) {
        let curr_pos = self.guard_pos;
        let (dx, dy) = self.guard_facing;
        let next_pos = (curr_pos.0 + dx, curr_pos.1 + dy);

        if !self._is_within_bounds(&next_pos) {
            return;
        }

        // Log history as the next steps will either change the position or the
        // facing of the guard.
        self.log_history();

        if self.get_cell_at_pos(&next_pos) == Cell::Obstacle {
            self.turn_right();
            return;
        }

        // No obstable in front. Move guard by one position
        self.set_cell_at_pos(&curr_pos, Cell::Visited);
        self.set_cell_at_pos(&next_pos, Cell::Guard);
        self.guard_pos = next_pos;
    }

    fn get_board_completion_state(&mut self) -> Completion {
        while self.has_next_state() {
            if self.is_guard_looping() {
                return Completion::Loop;
            }
            self.next_state();
        }
        return Completion::OutOfBounds;
    }

    fn get_obstacles_to_force_loop(&mut self) -> Vec<(i32, i32)> {
        // Validate that self always ends in out of bounds
        if self.clone().get_board_completion_state() != Completion::OutOfBounds {
            panic!("Initial problem already loops :(");
        }

        let mut obstacles: Vec<(i32, i32)> = Vec::new();
        while self.has_next_state() {
            let curr_pos = self.guard_pos;
            let (dx, dy) = self.guard_facing;
            let next_pos = (curr_pos.0 + dx, curr_pos.1 + dy);

            // If there is no obstacle in front and the guard has not been at this position before,
            // try putting an obstacle there to see if what happens to the board
            if self.get_cell_at_pos(&next_pos) != Cell::Obstacle
                && !self.was_guard_at_pos(&next_pos)
            {
                let mut test_board = self.clone();
                test_board.set_cell_at_pos(&next_pos, Cell::Obstacle);
                if test_board.get_board_completion_state() == Completion::Loop {
                    obstacles.push(next_pos);
                }
            }

            self.next_state();
        }

        return obstacles;
    }

    fn turn_right(&mut self) {
        self.guard_facing = match self.guard_facing {
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            _ => panic!("Invalid guard facing direction"),
        };
    }

    fn has_next_state(&self) -> bool {
        // If the guard's next step is within bounds, regardless of if an obstable exists or not
        // there is a next possible state, either by movinf the guard or by turning to the right.
        let curr_pos = self.guard_pos;
        let (dx, dy) = self.guard_facing;

        let next_pos = (curr_pos.0 + dx, curr_pos.1 + dy);
        return self._is_within_bounds(&next_pos);
    }

    fn is_guard_looping(&self) -> bool {
        let history_entry = (self.guard_pos, self.guard_facing);
        return self.guard_history.contains(&history_entry);
    }

    fn was_guard_at_pos(&self, pos: &(i32, i32)) -> bool {
        return self.guard_pos_history.contains(pos);
    }

    fn get_cell_at_pos(&self, pos: &(i32, i32)) -> Cell {
        return self.board[pos.1 as usize][pos.0 as usize];
    }

    fn set_cell_at_pos(&mut self, pos: &(i32, i32), new_cell: Cell) {
        self.board[pos.1 as usize][pos.0 as usize] = new_cell;
    }

    fn log_history(&mut self) {
        self.guard_pos_history.insert(self.guard_pos);
        self.guard_history
            .insert((self.guard_pos, self.guard_facing));
    }

    fn _is_within_bounds(&self, pos: &(i32, i32)) -> bool {
        return pos.0 >= 0
            && (pos.0 as usize) < self.board_size.0
            && pos.1 >= 0
            && (pos.1 as usize) < self.board_size.1;
    }

    fn pretty_print_board(&self) {
        for row in self.board.iter() {
            for cell_state in row.iter() {
                match cell_state {
                    Cell::Unvisited => print!("."),
                    Cell::Visited => print!("*"),
                    Cell::Guard => match self.guard_facing {
                        (0, -1) => print!("↑"),
                        (0, 1) => print!("↓"),
                        (-1, 0) => print!("←"),
                        (1, 0) => print!("→"),
                        _ => panic!("Invalid guard facing direction"),
                    },
                    Cell::Obstacle => print!("#"),
                    Cell::AddedObstacle => print!("O"),
                }
            }
            println!();
        }
        println!();
    }

    fn count_visited_cells(&self) -> usize {
        return self
            .board
            .iter()
            .flatten()
            .filter(|c| **c == Cell::Visited || **c == Cell::Guard)
            .count();
    }
}

fn part1(input_file: &PathBuf) {
    let mut board_state = State::from_file(input_file);
    board_state.pretty_print_board();

    while board_state.has_next_state() {
        board_state.next_state();
        board_state.pretty_print_board();
    }

    println!("Visited cells: {}", board_state.count_visited_cells());
}

fn part2(input_file: &PathBuf) {
    let vanilla_board = State::from_file(input_file);

    let mut working_board = vanilla_board.clone();
    let obstacles = working_board.get_obstacles_to_force_loop();

    for obstacle in obstacles.iter() {
        let mut print_board = vanilla_board.clone();
        print_board.set_cell_at_pos(obstacle, Cell::AddedObstacle);
        println!("Obstacle at {:?}", obstacle);
        print_board.pretty_print_board();
    }

    println!("Possible obstacles for loop: {}", obstacles.len());
}

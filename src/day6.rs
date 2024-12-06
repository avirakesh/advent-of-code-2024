use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day6.txt"));

    if part_opt.is_none() {
        println!("Running Day 5, Part 1");
        part1(&input);
        println!();
        println!("Running Day 5, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 5, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 5, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    UNVISITED,
    VISITED,
    GUARD,
    OBSTACLE,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    board: Vec<Vec<Cell>>,
    board_size: (usize, usize), // (width, height)
    guard_pos: (i32, i32),      // (x, y): (0, 0) is top left corner of board
    guard_facing: (i32, i32),   // (dx, dy)
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
                    '.' => row.push(Cell::UNVISITED),
                    '#' => row.push(Cell::OBSTACLE),
                    '^' => {
                        guard_pos = (row.len() as i32, board.len() as i32);
                        row.push(Cell::GUARD);
                    }
                    _ => panic!("Invalid character in input file"),
                }
            }
            board.push(row);
        }

        let board_size = (board[0].len(), board.len());
        return Self {
            board,
            board_size,
            guard_pos,
            guard_facing,
        };
    }

    fn next_state(&mut self) {
        let mut curr_pos = self.guard_pos;
        let (dx, dy) = self.guard_facing;
        let mut next_pos = (curr_pos.0 + dx, curr_pos.1 + dy);

        if self._is_within_bounds(&next_pos) && self.get_cell_at_pos(&next_pos) == Cell::OBSTACLE {
            self.turn_right();
            return;
        }

        // No obstable in front. Move guard until either an obstable is encountered, or
        // we reach the edge of the board.
        while self._is_within_bounds(&next_pos) && self.get_cell_at_pos(&next_pos) != Cell::OBSTACLE
        {
            self.set_cell_at_pos(&curr_pos, Cell::VISITED);
            curr_pos = next_pos;
            next_pos = (curr_pos.0 + dx, curr_pos.1 + dy);
        }

        self.set_cell_at_pos(&curr_pos, Cell::GUARD);
        self.guard_pos = curr_pos;
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

    fn get_cell_at_pos(&self, pos: &(i32, i32)) -> Cell {
        return self.board[pos.1 as usize][pos.0 as usize];
    }

    fn set_cell_at_pos(&mut self, pos: &(i32, i32), new_cell: Cell) {
        self.board[pos.1 as usize][pos.0 as usize] = new_cell;
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
                    Cell::UNVISITED => print!("."),
                    Cell::VISITED => print!("*"),
                    Cell::GUARD => match self.guard_facing {
                        (0, -1) => print!("↑"),
                        (0, 1) => print!("↓"),
                        (-1, 0) => print!("←"),
                        (1, 0) => print!("→"),
                        _ => panic!("Invalid guard facing direction"),
                    },
                    Cell::OBSTACLE => print!("#"),
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
            .filter(|c| **c == Cell::VISITED || **c == Cell::GUARD)
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
    todo!("Implement Part2");
}

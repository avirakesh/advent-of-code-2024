use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    vec,
};

static POSSIBLE_DIRS: [(i32, i32); 8] = [
    (0, -1),  // up
    (0, 1),   // down
    (-1, 0),  // left
    (1, 0),   // right
    (1, 1),   // down right
    (-1, -1), // up left
    (1, -1),  // down left
    (-1, 1),  // up right
];

static XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
static MAS: [char; 3] = ['M', 'A', 'S'];

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day4.txt"));

    if part_opt.is_none() {
        println!("Running Day 4, Part 1");
        part1(&input);
        println!();
        println!("Running Day 4, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 4, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 4, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

fn part1(input_file: &PathBuf) {
    let board = create_board(input_file);
    let mut empty_board = create_empty_board(&board);

    pretty_print_board(&board);
    println!();

    let board_size = (board[0].len(), board.len());
    let mut xmas_count = 0;
    for x in 0..board_size.0 {
        for y in 0..board_size.1 {
            if board[y][x] == 'X' {
                let xmas_dirs = check_xmas_in_all_dirs(&board, (x, y));
                xmas_count += xmas_dirs.len() as i32;
                fill_xmas_in_empty_board(&mut empty_board, &board, (x, y), xmas_dirs);
            }
        }
    }

    pretty_print_board(&empty_board);
    println!("XMAS count: {}", xmas_count);
}

fn create_board(input_file: &PathBuf) -> Vec<Vec<char>> {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );
    let lines = BufReader::new(input_file).lines();

    let mut board: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line.expect("Could not read line");
        let parts: Vec<char> = line.chars().collect();
        board.push(parts);
    }
    return board;
}

fn create_empty_board(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let empty_board: Vec<Vec<char>> = vec![vec!['.'; board[0].len()]; board.len()];
    return empty_board;
}

fn fill_xmas_in_empty_board(
    empty_board: &mut Vec<Vec<char>>,
    board: &Vec<Vec<char>>,
    start: (usize, usize),
    xmas_dirs: Vec<(i32, i32)>,
) {
    for dir in xmas_dirs {
        let (mut x, mut y) = (start.0 as i32, start.1 as i32);
        for _ in XMAS {
            let (x_idx, y_idx) = (x as usize, y as usize);
            empty_board[y_idx][x_idx] = board[y_idx][x_idx];
            x += dir.0;
            y += dir.1;
        }
    }
}

fn check_xmas_in_all_dirs(board: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<(i32, i32)> {
    if board[start.1][start.0] != 'X' {
        return Vec::new();
    }

    let mut ret: Vec<(i32, i32)> = Vec::new();
    for dir in POSSIBLE_DIRS {
        if is_xmas_in_dir(board, start, dir) {
            ret.push(dir);
        }
    }
    return ret;
}

fn is_xmas_in_dir(board: &Vec<Vec<char>>, start: (usize, usize), dir: (i32, i32)) -> bool {
    let mut x = start.0 as i32;
    let mut y = start.1 as i32;

    let board_size = (board[0].len() as i32, board.len() as i32);

    for c in XMAS {
        if x < 0 || x >= board_size.0 || y < 0 || y >= board_size.1 {
            return false;
        }

        if board[y as usize][x as usize] != c {
            return false;
        }
        x += dir.0;
        y += dir.1;
    }

    return true;
}

fn pretty_print_board(board: &Vec<Vec<char>>) {
    for row in board {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part2(input_file: &PathBuf) {
    let board = create_board(input_file);
    let mut empty_board = create_empty_board(&board);

    pretty_print_board(&board);
    println!();

    let board_size = (board[0].len(), board.len());
    let mut x_mas_count = 0;

    for x in 0..board_size.0 {
        for y in 0..board_size.1 {
            if board[y][x] == 'A' {
                if is_x_mas(&board, (x, y)) {
                    fill_x_mas_in_empty_board(&mut empty_board, &board, (x, y));
                    x_mas_count += 1;
                }
            }
        }
    }

    pretty_print_board(&empty_board);
    println!("X-MAS count: {}", x_mas_count);
}

fn is_x_mas(board: &Vec<Vec<char>>, middle: (usize, usize)) -> bool {
    let (x, y) = (middle.0 as i32, middle.1 as i32);
    if board[y as usize][x as usize] != 'A' {
        return false;
    }

    let is_left_diagonal_mas = is_mas_in_dir(board, (x - 1, y - 1), (1, 1))
        || is_mas_in_dir(board, (x + 1, y + 1), (-1, -1));

    let is_right_diagonal_mas = is_mas_in_dir(board, (x + 1, y - 1), (-1, 1))
        || is_mas_in_dir(board, (x - 1, y + 1), (1, -1));

    return is_left_diagonal_mas && is_right_diagonal_mas;
}

fn is_mas_in_dir(board: &Vec<Vec<char>>, start: (i32, i32), dir: (i32, i32)) -> bool {
    let mut x = start.0;
    let mut y = start.1;
    for c in MAS {
        if x < 0 || x >= board[0].len() as i32 || y < 0 || y >= board.len() as i32 {
            return false;
        }
        if board[y as usize][x as usize] != c {
            return false;
        }
        x += dir.0;
        y += dir.1;
    }
    return true;
}

fn fill_x_mas_in_empty_board(
    empty_board: &mut Vec<Vec<char>>,
    board: &Vec<Vec<char>>,
    middle: (usize, usize),
) {
    let (mut x, mut y) = (middle.0 - 1, middle.1 - 1);
    let dir = (1, 1);
    for _ in MAS {
        empty_board[y][x] = board[y][x];
        x += dir.0;
        y += dir.1;
    }

    let (mut x, mut y) = ((middle.0 + 1) as i32, (middle.1 - 1) as i32);
    let dir = (-1 as i32, 1 as i32);
    for _ in MAS {
        empty_board[y as usize][x as usize] = board[y as usize][x as usize];
        x += dir.0;
        y += dir.1;
    }
}

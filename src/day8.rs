use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use colored::Colorize;
use num_rational::Ratio;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day8.txt"));

    if part_opt.is_none() {
        println!("Running Day 8, Part 1");
        part1(&input);
        println!();
        println!("Running Day 8, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 8, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 8, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Distance {
    dx: i32,
    dy: i32,
}

impl Distance {
    fn between_points(from: &(i32, i32), to: &(i32, i32)) -> Self {
        Distance {
            dx: (to.0 - from.0),
            dy: (to.1 - from.1),
        }
    }

    fn point_at_distance(&self, from: &(i32, i32)) -> (i32, i32) {
        return (from.0 + self.dx, from.1 + self.dy);
    }
}

impl std::ops::Mul<i32> for Distance {
    type Output = Distance;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl std::ops::Div<i32> for Distance {
    type Output = Result<Distance, ()>;

    fn div(self, rhs: i32) -> Self::Output {
        if rhs == 0 {
            return Err(());
        }
        if self.dx % rhs != 0 || self.dy % rhs != 0 {
            Err(())
        } else {
            return Ok(Distance {
                dx: self.dx / rhs,
                dy: self.dy / rhs,
            });
        }
    }
}

impl std::ops::Add<Distance> for Distance {
    type Output = Distance;
    fn add(self, rhs: Self) -> Self::Output {
        Distance {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl std::ops::Sub<Distance> for Distance {
    type Output = Distance;

    fn sub(self, rhs: Distance) -> Self::Output {
        Distance {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    // All positions are tracked as (x, y) where (0, 0) is the top-left corner of the board
    board_size: (i32, i32),                   // (width, height)
    antennas: HashMap<char, Vec<(i32, i32)>>, // (frequency, antenna positions),
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
        let mut board_height = 0;
        let mut board_width: Option<i32> = Option::None;

        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for line in lines {
            let line = line.expect("Could not read line");
            if board_width.is_none() {
                board_width = Some(line.chars().count() as i32);
            }
            for (idx, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_insert(Vec::new())
                        .push((idx as i32, board_height));
                }
            }
            board_height += 1;
        }

        return State {
            board_size: (board_width.unwrap(), board_height),
            antennas,
        };
    }

    fn get_antinodes_for_all_antennas(&self) -> HashSet<(i32, i32)> {
        let mut antinodes: Vec<HashSet<(i32, i32)>> = Vec::new();
        for (_, positions) in self.antennas.iter() {
            if positions.len() < 2 {
                continue;
            }

            let frequency_antinodes = self.get_antinode_for_one_frequency(positions);
            antinodes.push(frequency_antinodes);
        }
        return antinodes
            .into_iter()
            .flatten()
            .filter(|p| p.0 >= 0 && p.0 < self.board_size.0 && p.1 >= 0 && p.1 < self.board_size.1)
            .collect();
    }

    fn get_antinode_for_one_frequency(&self, positions: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
        let mut antinode_positions: HashSet<(i32, i32)> = HashSet::new();

        for i in 0..positions.len() - 1 {
            let p1 = positions[i];
            for j in i + 1..positions.len() {
                let p2 = positions[j];
                let antennas_in_line = self.get_all_antennas_in_line(p1, p2, positions);
                if antennas_in_line.len() > 2 {
                    println!(
                        "{}",
                        format!("Found more than 2 antennas in line: {:?}", antennas_in_line)
                            .as_str()
                            .red()
                    );
                    continue;
                }

                let distance = Distance::between_points(&p1, &p2);

                // Antinodes between antennas
                let third_distance = distance / 3;
                if third_distance.is_ok() {
                    let third_distance = third_distance.unwrap();
                    antinode_positions.insert(third_distance.point_at_distance(&p1));
                    antinode_positions.insert((third_distance * -1).point_at_distance(&p2));
                }

                // Antinodes outside antennas
                antinode_positions.insert(distance.point_at_distance(&p2));
                antinode_positions.insert((distance * -1).point_at_distance(&p1));
            }
        }

        return antinode_positions;
    }

    fn get_all_antennas_in_line(
        &self,
        p1: (i32, i32),
        p2: (i32, i32),
        positions: &Vec<(i32, i32)>,
    ) -> Vec<(i32, i32)> {
        let distance = Distance::between_points(&p1, &p2);
        let slope = Ratio::new(distance.dy, distance.dx);

        let mut ret: Vec<(i32, i32)> = vec![p1, p2];

        for p in positions {
            if p == &p1 || p == &p2 {
                continue;
            }

            let p_distance = Distance::between_points(&p1, &p);
            if Ratio::new(p_distance.dy, p_distance.dx) == slope {
                ret.push(*p);
                continue;
            }

            let p_distance = Distance::between_points(&p, &p1);
            if Ratio::new(p_distance.dy, p_distance.dx) == slope {
                ret.push(*p);
                continue;
            }
        }
        return ret;
    }

    fn visualize_antinodes(&self, antinodes: &HashSet<(i32, i32)>) {
        let mut board = vec![vec!['.'; self.board_size.0 as usize]; self.board_size.1 as usize];
        for (frequency, pos) in self.antennas.iter() {
            for (x, y) in pos.iter() {
                board[*y as usize][*x as usize] = *frequency;
            }
        }

        for (x, y) in antinodes.iter() {
            if board[*y as usize][*x as usize] == '.' {
                board[*y as usize][*x as usize] = '#';
            }
        }

        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if antinodes.contains(&(x as i32, y as i32)) {
                    print!("{}", format!("{}", cell).as_str().green());
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }

        println!();
    }
}

fn part1(input_file: &PathBuf) {
    let state = State::from_file(input_file);
    let antinodes = state.get_antinodes_for_all_antennas();
    println!("{:?}", state);

    state.visualize_antinodes(&antinodes);
    println!(
        "Number of antinodes: {}",
        format!("{}", antinodes.len()).as_str().green()
    );
}

fn part2(input_file: &PathBuf) {
    todo!("Implement day8 part2");
}

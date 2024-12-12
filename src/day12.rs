use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Index, IndexMut},
    path::PathBuf,
};

use colored::{Colorize, CustomColor};

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day12.txt"));

    if part_opt.is_none() {
        println!("Running Day 12, Part 1");
        part1(&input);
        println!();
        println!("Running Day 12, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 12, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 12, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn new(x: usize, y: usize) -> Coords {
        Coords { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Boundary {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Boundary {
    fn new(top: bool, bottom: bool, left: bool, right: bool) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone)]
struct Garden {
    plot: Vec<Vec<char>>,
    id_map: Vec<Vec<i32>>,
    id_to_plant: HashMap<i32, char>,
    id_to_perimeter_area: HashMap<i32, (usize, usize)>,
    id_to_num_sides: HashMap<i32, usize>,
}

impl<T> Index<Coords> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Coords) -> &Self::Output {
        return &self[index.y][index.x];
    }
}

impl<T> Index<&Coords> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: &Coords) -> &Self::Output {
        return &self[index.y][index.x];
    }
}

impl<T> IndexMut<Coords> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Coords) -> &mut Self::Output {
        return &mut self[index.y][index.x];
    }
}

impl<T> IndexMut<&Coords> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: &Coords) -> &mut Self::Output {
        return &mut self[index.y][index.x];
    }
}

impl Add<(i32, i32)> for &Coords {
    type Output = Result<Coords, ()>;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        let x = self.x as i32 + rhs.0;
        let y = self.y as i32 + rhs.1;
        if x < 0 || y < 0 {
            return Err(());
        }
        return Ok(Coords::new(x as usize, y as usize));
    }
}

impl Add<(i32, i32)> for Coords {
    type Output = Result<Coords, ()>;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        let x = self.x as i32 + rhs.0;
        let y = self.y as i32 + rhs.1;
        if x < 0 || x < 0 {
            return Err(());
        }
        return Ok(Coords::new(x as usize, y as usize));
    }
}

impl Garden {
    fn from_file(input_file: &PathBuf) -> Self {
        let input_file = File::open(input_file).expect(
            format!(
                "Could not open input file: {}",
                input_file.to_string_lossy()
            )
            .as_str(),
        );

        let lines = BufReader::new(input_file).lines();
        let plot: Vec<Vec<char>> = lines
            .into_iter()
            .map(|l| l.expect("Could not read line"))
            .map(|l| l.chars().into_iter().collect())
            .collect();

        let id_map: Vec<Vec<i32>> = vec![vec![-1; plot[0].len()]; plot.len()];

        return Self {
            plot,
            id_map,
            id_to_plant: HashMap::new(),
            id_to_perimeter_area: HashMap::new(),
            id_to_num_sides: HashMap::new(),
        };
    }

    fn label_plants(&mut self) {
        let mut next_id = 0;
        while let Some(unlabelled_coord) = self.get_one_unlabelled_coord() {
            self.label_all_plants_in_group(&unlabelled_coord, next_id);
            next_id += 1;
        }
    }

    fn label_all_plants_in_group(&mut self, unlabelled_coord: &Coords, id: i32) {
        let plant = self.plot[unlabelled_coord];
        let mut perimeter: usize = 0;
        let mut area: usize = 0;

        // coord -> boundary at (top, bottom, left, right)
        let mut coords_to_boundary: HashMap<Coords, Boundary> = HashMap::new();

        self.id_to_plant.insert(id, plant);

        let mut active_coords: Vec<Coords> = vec![*unlabelled_coord];
        let mut visited_coords: HashSet<Coords> = HashSet::new();

        while !active_coords.is_empty() {
            let coord = active_coords.pop().unwrap();
            if visited_coords.contains(&coord) {
                continue;
            }

            visited_coords.insert(coord);

            self.id_map[coord] = id;
            area += 1;

            let group_neighbors = self.get_group_neighbors(&coord);
            perimeter += 4 - group_neighbors.len();
            coords_to_boundary.insert(coord, self.get_boundary_of_coord(&coord, &group_neighbors));

            active_coords.extend(
                group_neighbors
                    .iter()
                    .filter(|c| !visited_coords.contains(*c))
                    .filter(|c| self.id_map[*c] == -1),
            );
        }

        self.id_to_perimeter_area.insert(id, (perimeter, area));
        self.id_to_num_sides
            .insert(id, self.count_sides_of_group(&coords_to_boundary));
    }

    fn count_sides_of_group(&self, coords_to_boundary: &HashMap<Coords, Boundary>) -> usize {
        let mut num_sides: usize = 0;
        for (coord, boundary) in coords_to_boundary.iter() {
            let left = coord + (-1, 0);
            let left = if left.is_ok() && coords_to_boundary.contains_key(&left.unwrap()) {
                coords_to_boundary[&left.unwrap()]
            } else {
                Boundary::new(false, false, true, false)
            };

            let top = coord + (0, -1);
            let top = if top.is_ok() && coords_to_boundary.contains_key(&top.unwrap()) {
                coords_to_boundary[&top.unwrap()]
            } else {
                Boundary::new(true, false, false, false)
            };

            if boundary.top && !left.top {
                num_sides += 1;
            }

            if boundary.bottom && !left.bottom {
                num_sides += 1;
            }

            if boundary.left && !top.left {
                num_sides += 1;
            }

            if boundary.right && !top.right {
                num_sides += 1;
            }
        }

        return num_sides;
    }

    fn get_boundary_of_coord(&self, coord: &Coords, group_neighbor: &Vec<Coords>) -> Boundary {
        let top = coord + (0, -1);
        let top = top.is_err() || !group_neighbor.contains(&top.unwrap());

        let bottom = coord + (0, 1);
        let bottom = bottom.is_err() || !group_neighbor.contains(&bottom.unwrap());

        let left = coord + (-1, 0);
        let left = left.is_err() || !group_neighbor.contains(&left.unwrap());

        let right = coord + (1, 0);
        let right = right.is_err() || !group_neighbor.contains(&right.unwrap());

        return Boundary::new(top, bottom, left, right);
    }

    fn get_group_neighbors(&self, coord: &Coords) -> Vec<Coords> {
        let neighbor_direction = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

        let plant = self.plot[coord];
        return neighbor_direction
            .iter()
            .map(|dir| coord + *dir)
            .filter(|coord| coord.is_ok())
            .map(|c| c.unwrap())
            .filter(|c| c.x < self.plot[0].len() && c.y < self.plot.len())
            .filter(|c| self.plot[c] == plant)
            .collect();
    }

    fn get_one_unlabelled_coord(&self) -> Option<Coords> {
        self.id_map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, &id)| {
                    if id == -1 {
                        Some(Coords::new(x as usize, y as usize))
                    } else {
                        None
                    }
                })
            })
            .next()
    }

    fn total_perimeter_times_area(&self) -> usize {
        return self
            .id_to_perimeter_area
            .iter()
            .map(|(_, (p, a))| p * a)
            .sum();
    }

    fn total_num_sides_times_area(&self) -> usize {
        return self
            .id_to_num_sides
            .iter()
            .map(|(id, num_sides)| self.id_to_perimeter_area[id].1 * num_sides)
            .sum();
    }

    fn generate_color_mapping(&self) -> HashMap<i32, CustomColor> {
        let mut color_mapping = HashMap::new();
        color_mapping.insert(-1, CustomColor::new(10, 10, 10));

        let mut colors: HashSet<(u8, u8, u8)> = HashSet::new();
        while colors.len() < self.id_to_plant.len() {
            let r: f32 = rand::random();
            let r = 255 - ((r * (255 - 128) as f32) as u8);

            let g: f32 = rand::random();
            let g = 255 - ((g * (255 - 128) as f32) as u8);

            let b: f32 = rand::random();
            let b = 255 - ((b * (255 - 128) as f32) as u8);

            colors.insert((r, g, b));
        }

        for (id, color) in self.id_to_plant.keys().zip(colors.iter()) {
            color_mapping.insert(*id, CustomColor::new(color.0, color.1, color.2));
        }

        return color_mapping;
    }
}

impl Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_mapping = self.generate_color_mapping();

        writeln!(f, "Garden:")?;
        for (y, row) in self.plot.iter().enumerate() {
            for (x, plant) in row.iter().enumerate() {
                let id = self.id_map[y][x];
                let color = color_mapping[&id];

                write!(
                    f,
                    "{:<3}",
                    (*plant).to_string().as_str().custom_color(color)
                )?;
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "Grouping:")?;
        for row in self.id_map.iter() {
            for id in row.iter() {
                let color = color_mapping[id];
                write!(f, "{:<4}", (*id).to_string().as_str().custom_color(color))?;
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "ID Map:")?;
        writeln!(
            f,
            " |------------|------------|------------|------------|------------|------------|------------|"
        )?;
        writeln!(
            f,
            " | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} |",
            "ID", "PLANT", "PERIMETER", "AREA", "PERIMxAREA", "NUM SIDES", "SIDESxAREA"
        )?;
        writeln!(
            f,
            " |------------|------------|------------|------------|------------|------------|------------|"
        )?;

        for id in 0..self.id_to_plant.len() {
            let id = &(id as i32);
            let plant = self.id_to_plant[id];
            let (perimeter, area) = self.id_to_perimeter_area.get(id).unwrap();
            let num_sides = self.id_to_num_sides.get(id).unwrap();
            let color = color_mapping[id];
            writeln!(
                f,
                "{}",
                format!(
                    " | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} |",
                    id,
                    plant,
                    perimeter,
                    area,
                    perimeter * area,
                    num_sides,
                    num_sides * area,
                )
                .as_str()
                .custom_color(color)
            )?;
        }
        writeln!(
            f,
            " |------------|------------|------------|------------|------------|------------|------------|"
        )?;

        Ok(())
    }
}

fn part1(input_file: &PathBuf) {
    let mut garden = Garden::from_file(input_file);
    println!("{}", garden);

    garden.label_plants();
    println!("{}", garden);

    println!(
        "Total Price: {}",
        garden
            .total_perimeter_times_area()
            .to_string()
            .green()
            .bold()
    );
}

fn part2(input_file: &PathBuf) {
    let mut garden = Garden::from_file(input_file);
    println!("{}", garden);

    garden.label_plants();
    println!("{}", garden);

    println!(
        "Total Price: {}",
        garden
            .total_num_sides_times_area()
            .to_string()
            .green()
            .bold()
    );
}

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use colored::Colorize;
use itertools::Itertools;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day23.txt"));

    if part_opt.is_none() {
        println!("Running Day 23, Part 1");
        part1(&input);
        println!();
        println!("Running Day 23, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 23, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 23, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

fn get_connections_from_file(input_file: &PathBuf) -> HashMap<String, HashSet<String>> {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );
    let lines = BufReader::new(input_file).lines();
    lines
        .into_iter()
        .map(|l| l.expect("Could not read line"))
        .map(|l| {
            let parts: Vec<&str> = l.split("-").collect();
            let c1 = parts[0].to_string();
            let c2 = parts[1].to_string();

            return (c1, c2);
        })
        .for_each(|(c1, c2)| {
            connections
                .entry(c1.clone())
                .or_insert_with(HashSet::new)
                .insert(c2.clone());
            connections
                .entry(c2)
                .or_insert_with(HashSet::new)
                .insert(c1);
        });

    return connections;
}

fn part1(input_file: &PathBuf) {
    let connections = get_connections_from_file(input_file);
    println!("{:#?}", connections);

    let triangles = get_triangles_with_t(&connections);
    triangles.iter().for_each(|c| println!("{:?}", c));

    println!();
    println!(
        "Number of triangles with at least one t: {}",
        triangles.len().to_string().green().bold()
    );
}

fn get_triangles_with_t(connections: &HashMap<String, HashSet<String>>) -> Vec<[String; 3]> {
    let mut triangles: HashSet<[String; 3]> = HashSet::new();

    for key in connections.keys().filter(|k| k.starts_with("t")) {
        let neighbors = connections.get(key).unwrap();

        for neighbor in neighbors {
            let second_order_neighbors = connections.get(neighbor).unwrap();
            let common_neighbors = neighbors.intersection(second_order_neighbors);
            for common_neigbor in common_neighbors {
                let mut v = [key.clone(), neighbor.clone(), common_neigbor.clone()];
                v.sort();
                triangles.insert(v);
            }
        }
    }

    return triangles.into_iter().collect();
}

fn part2(input_file: &PathBuf) {
    let connections = get_connections_from_file(input_file);
    println!("{:#?}", connections);

    let fully_connected_sets = get_fully_connected_sets_with_t(&connections);
    fully_connected_sets
        .iter()
        .for_each(|v| println!("{:?}", v));

    let largest_connected_set = fully_connected_sets
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    let password = largest_connected_set.iter().join(",");

    println!();
    println!(
        "Largest Connected Set: {}",
        format!("{:?}", largest_connected_set)
            .as_str()
            .cyan()
            .bold()
    );
    println!("Password: {}", password.green().bold());
}

fn get_fully_connected_sets_with_t(
    connections: &HashMap<String, HashSet<String>>,
) -> HashSet<Vec<String>> {
    let mut fully_connected_sets: HashSet<Vec<String>> = HashSet::new();

    for key in connections.keys().filter(|k| k.starts_with("t")) {
        let mut visited: HashSet<Vec<String>> = HashSet::new();
        let mut stack: Vec<(Vec<String>, HashSet<String>)> = Vec::new();
        stack.push((vec![key.clone()], connections.get(key).unwrap().clone()));

        while !stack.is_empty() {
            let (mut current_set, working_neighbors) = stack.pop().unwrap();

            if visited.contains(&current_set) {
                continue;
            }

            visited.insert(current_set.clone());

            if working_neighbors.is_empty() {
                current_set.sort();
                fully_connected_sets.insert(current_set);
                continue;
            }

            for neighbor in working_neighbors.iter() {
                // println!("{:?} -> {}", working_neighbors, neighbor);
                let next_level_neighbors = connections.get(neighbor).unwrap();
                let common_neigbors: HashSet<String> = working_neighbors
                    .intersection(next_level_neighbors)
                    .map(|s| s.clone())
                    .collect();

                let mut next_set = current_set.clone();
                next_set.push(neighbor.clone());
                next_set.sort();
                stack.push((next_set, common_neigbors));
            }
        }
    }

    return fully_connected_sets;
}

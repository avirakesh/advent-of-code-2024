use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use colored::Colorize;

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
    todo!("Implement Part2");
}

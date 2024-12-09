use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use colored::Colorize;

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day9.txt"));

    if part_opt.is_none() {
        println!("Running Day 9, Part 1");
        part1(&input);
        println!();
        println!("Running Day 9, Part 2");
        part2(&input);
        return;
    }

    let part = part_opt.unwrap();
    match part {
        1 => {
            println!("Running Day 9, Part 1");
            part1(&input)
        }
        2 => {
            println!("Running Day 9, Part 2");
            part2(&input)
        }
        _ => panic!("Invalid Part :("),
    }
}

fn part1(input_file: &PathBuf) {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );

    let lines = BufReader::new(input_file).lines();
    let mut diskmap: Option<Vec<i32>> = None;
    for line in lines {
        let line = line.expect("Could not read line");
        diskmap = Some(
            line.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
        break;
    }

    let diskmap: Vec<i32> = diskmap.unwrap();
    println!("Diskmap: {:?}", diskmap);

    // Each entry is now "(size, id)", where id for empty blocks is 0
    let diskmap: Vec<(i32, i32)> = diskmap
        .into_iter()
        .enumerate()
        .filter(|(_, v)| *v != 0)
        .map(|(idx, v)| {
            if idx % 2 == 0 {
                (v, idx as i32 / 2)
            } else {
                (v, -1)
            }
        })
        .collect();

    println!("[(size, id)]: {:?}", diskmap);

    let mut defragged_diskmap: Vec<(i32, i32)> = Vec::new();

    let mut last_filled_ptr = last_full_block_before_idx(diskmap.len() as i32, &diskmap);
    let (mut last_filled_size, mut last_filled_id) = diskmap[last_filled_ptr.unwrap()];

    let mut idx = 0;
    let (mut size, mut id) = diskmap[idx];
    while idx < diskmap.len() {
        if last_filled_ptr.is_none() || idx >= last_filled_ptr.unwrap() {
            break;
        }

        if id != -1 {
            defragged_diskmap.push((size, id));
            idx += 1;
            (size, id) = diskmap[idx];
            continue;
        }

        if size == last_filled_size {
            defragged_diskmap.push((size, last_filled_id));
            last_filled_ptr = last_full_block_before_idx(last_filled_ptr.unwrap() as i32, &diskmap);
            if last_filled_ptr.is_none() {
                break;
            }
            (last_filled_size, last_filled_id) = diskmap[last_filled_ptr.unwrap()];

            idx += 1;
            (size, id) = diskmap[idx];
            continue;
        }

        if size < last_filled_size {
            defragged_diskmap.push((size, last_filled_id));
            last_filled_size = last_filled_size - size;

            idx += 1;
            (size, id) = diskmap[idx];
            continue;
        }

        // size must be > filled_size
        defragged_diskmap.push((last_filled_size, last_filled_id));
        last_filled_ptr = last_full_block_before_idx(last_filled_ptr.unwrap() as i32, &diskmap);
        size = size - last_filled_size;
        if last_filled_ptr.is_none() {
            break;
        }
        (last_filled_size, last_filled_id) = diskmap[last_filled_ptr.unwrap()];
    }
    if last_filled_ptr.is_some() && idx == last_filled_ptr.unwrap() {
        defragged_diskmap.push((last_filled_size, last_filled_id));
    }

    println!("Defragged: {:?}", defragged_diskmap);
    println!(
        "Checksum: {}",
        format!("{}", calculate_checksum(&defragged_diskmap))
            .as_str()
            .green()
            .bold()
    );
}

fn calculate_checksum(defragged_diskmap: &Vec<(i32, i32)>) -> i64 {
    let mut num_blocks: i64 = 0;
    let mut checksum: i64 = 0;

    for (size, id) in defragged_diskmap {
        if *id == -1 {
            num_blocks += *size as i64;
            continue;
        }

        let block_sum: i64 = (num_blocks..(num_blocks + (*size) as i64)).sum();
        checksum += block_sum * (*id as i64);
        num_blocks += *size as i64;
    }

    return checksum;
}

fn last_full_block_before_idx(idx: i32, diskmap: &Vec<(i32, i32)>) -> Option<usize> {
    let idx = diskmap.len() as i32 - 1 - idx;
    let num_to_skip = idx + 1;
    return diskmap
        .iter()
        .enumerate()
        .rev()
        .skip(num_to_skip as usize)
        .filter(|(_, (_, id))| *id != -1)
        .map(|(idx, _)| idx)
        .next();
}

fn part2(input_file: &PathBuf) {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );

    let lines = BufReader::new(input_file).lines();
    let mut diskmap: Option<Vec<i32>> = None;
    for line in lines {
        let line = line.expect("Could not read line");
        diskmap = Some(
            line.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
        break;
    }

    let diskmap: Vec<i32> = diskmap.unwrap();
    println!("Diskmap: {:?}", diskmap);

    // Each entry is now "(size, id, has_moved)", where id for empty blocks is 0
    let diskmap: Vec<(i32, i32, bool)> = diskmap
        .into_iter()
        .enumerate()
        .filter(|(_, v)| *v != 0)
        .map(|(idx, v)| {
            if idx % 2 == 0 {
                (v, idx as i32 / 2)
            } else {
                (v, -1)
            }
        })
        .map(|(size, id)| (size, id, false))
        .collect();

    println!("[(size, id)]: {:?}", diskmap);

    let mut defragged_diskmap: Vec<(i32, i32, bool)> = diskmap.clone();

    let mut idx = (defragged_diskmap.len() - 1) as i32;
    while idx >= 0 {
        if defragged_diskmap[idx as usize].1 == -1 {
            idx -= 1;
            continue;
        }

        let (fill_size, fill_id, moved) = defragged_diskmap[idx as usize];
        if moved {
            idx -= 1;
            continue;
        }

        let empty_idx = first_empty_idx(&defragged_diskmap, fill_size, idx as usize);
        if empty_idx.is_none() {
            idx -= 1;
            continue;
        }
        let empty_idx = empty_idx.unwrap();
        let empty_size = defragged_diskmap[empty_idx].0;

        defragged_diskmap[empty_idx] = (fill_size, fill_id, true);

        if empty_size > fill_size {
            let remaining_size = empty_size - fill_size;
            defragged_diskmap.insert(empty_idx + 1, (remaining_size, -1, false));
            idx += 1;
        }

        let block_removed = insert_empty_block(&mut defragged_diskmap, fill_size, idx as usize);
        idx -= if block_removed { 1 } else { 0 };

        idx -= 1;
    }

    println!("Defragged: {:?}", defragged_diskmap);
    let defragged_diskmap: Vec<(i32, i32)> =
        defragged_diskmap.iter().map(|(s, i, _)| (*s, *i)).collect();
    println!(
        "Checksum: {}",
        format!("{}", calculate_checksum(&defragged_diskmap))
            .as_str()
            .green()
            .bold()
    );
}

fn first_empty_idx(
    diskmap: &Vec<(i32, i32, bool)>,
    min_size: i32,
    max_idx: usize,
) -> Option<usize> {
    return diskmap
        .iter()
        .enumerate()
        .filter(|(idx, (size, id, _))| *id == -1 && *size >= min_size && *idx <= max_idx)
        .map(|(idx, _)| idx)
        .next();
}

fn insert_empty_block(diskmap: &mut Vec<(i32, i32, bool)>, size: i32, idx: usize) -> bool {
    // Returns true if a block was removed to the left of idx.
    if idx == 0 && diskmap[idx + 1].1 == -1 {
        let removed_size = diskmap[idx + 1].0;
        diskmap[idx] = (size + removed_size, -1, false);
        let _ = diskmap.remove(idx + 1);
        return false;
    }

    if idx == diskmap.len() - 1 && diskmap[idx - 1].1 == -1 {
        let removed_size = diskmap[idx - 1].0;
        diskmap[idx - 1] = (size + removed_size, -1, false);
        let _ = diskmap.remove(idx).0;
        return true;
    }

    if idx > 0 && diskmap[idx - 1].1 == -1 && idx < diskmap.len() - 1 && diskmap[idx + 1].1 == -1 {
        let (removed_1, _, _) = diskmap[idx - 1];
        let (removed_2, _, _) = diskmap[idx + 1];
        diskmap[idx] = (size + removed_1 + removed_2, -1, false);
        let _ = diskmap.remove(idx + 1);
        let _ = diskmap.remove(idx - 1).0;
        return true;
    }

    if idx > 0 && diskmap[idx - 1].1 == -1 {
        let (removed_size, _, _) = diskmap[idx - 1];
        diskmap[idx] = (size + removed_size, -1, false);
        let _ = diskmap.remove(idx - 1).0;
        return true;
    }

    if idx < diskmap.len() - 1 && diskmap[idx + 1].1 == -1 {
        let (removed_size, _, _) = diskmap[idx + 1];
        diskmap[idx] = (size + removed_size, -1, false);
        let _ = diskmap.remove(idx + 1).0;
        return false;
    }

    diskmap[idx] = (size, -1, false);
    return false;
}

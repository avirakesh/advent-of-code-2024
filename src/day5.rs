use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn main(part_opt: Option<u32>, input_opt: Option<PathBuf>) {
    let input = input_opt.unwrap_or(PathBuf::from("input/day5.txt"));

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

fn part1(input_file: &PathBuf) {
    let (rules, book) = get_rules_and_book(input_file);
    println!();
    let reverse_rules = generate_reverse_rules_index(rules);
    println!("Reverse Rules: {:?}", reverse_rules);
    println!();

    let mut sum = 0;
    for pages in book {
        if are_pages_valid(&pages, &reverse_rules) {
            println!("Valid Pages: {:?}", pages);
            sum += pages[pages.len() / 2]
        }
    }

    println!("Sum: {:?}", sum);
}

fn get_rules_and_book(input_file: &PathBuf) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let input_file = File::open(input_file).expect(
        format!(
            "Could not open input file: {}",
            input_file.to_string_lossy()
        )
        .as_str(),
    );
    let lines = BufReader::new(input_file).lines();

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut book: Vec<Vec<i32>> = Vec::new();
    let mut is_pages_section = false;

    for line in lines {
        let line = line.expect("Could not read line");
        if line.is_empty() {
            is_pages_section = true;
            continue;
        }
        if !is_pages_section {
            let parts: Vec<&str> = line.split("|").collect();
            let rule = (
                parts[0]
                    .parse::<i32>()
                    .expect(format!("Could not parse rule from {}", line).as_str()),
                parts[1]
                    .parse::<i32>()
                    .expect(format!("Could not parse rule from {}", line).as_str()),
            );
            rules.push(rule);
        } else {
            let pages: Vec<i32> = line
                .split(",")
                .map(|s| {
                    s.parse::<i32>()
                        .expect(format!("Could not parse page from {}", line).as_str())
                })
                .collect();
            book.push(pages);
        }
    }

    println!("Rules: {:?}", rules);
    println!("Pages: {:?}", book);

    return (rules, book);
}

fn generate_reverse_rules_index(rules: Vec<(i32, i32)>) -> HashMap<i32, HashSet<i32>> {
    // Reverse rules means that for any key, the value is the set of pages that should NOT
    // come after it.
    let mut reverse_rules = HashMap::new();
    for (i, j) in rules {
        let entry = reverse_rules.entry(j).or_insert(HashSet::new());
        entry.insert(i);
    }
    return reverse_rules;
}

fn are_pages_valid(pages: &Vec<i32>, reverse_rules: &HashMap<i32, HashSet<i32>>) -> bool {
    for i in 0..pages.len() - 1 {
        for j in (i + 1)..pages.len() {
            let p1 = pages[i];
            let p2 = pages[j];

            if reverse_rules.contains_key(&p1) && reverse_rules[&p1].contains(&p2) {
                // Found an invalid page sequence.
                return false;
            }
        }
    }

    return true;
}

fn part2(input_file: &PathBuf) {
    let (rules, book) = get_rules_and_book(input_file);
    println!();
    let reverse_rules = generate_reverse_rules_index(rules);
    println!("Reverse Rules: {:?}", reverse_rules);
    println!();

    let mut sum = 0;
    for mut pages in book {
        if are_pages_valid(&pages, &reverse_rules) {
            continue;
        }

        fix_invalid_pages(&mut pages, &reverse_rules);
        println!("Fixed Pages: {:?}", pages);
        sum += pages[pages.len() / 2]
    }
    println!("Sum: {:?}", sum)
}

fn fix_invalid_pages(pages: &mut Vec<i32>, reverse_rules: &HashMap<i32, HashSet<i32>>) {
    for start_idx in 0..pages.len() {
        let mut curr_idx: usize = start_idx + 1;
        while curr_idx < pages.len() {
            let p1 = pages[start_idx];
            let p2 = pages[curr_idx];

            if reverse_rules.contains_key(&p1) && reverse_rules[&p1].contains(&p2) {
                // Swap the offending entries fixing this particular sequence.
                pages[start_idx] = p2;
                pages[curr_idx] = p1;

                // reset curr_idx to start_idx + 1 and continue checking for invalid sequences.
                curr_idx = start_idx + 1;
                continue;
            }

            // No rules violated, move on to the next page.
            curr_idx += 1;
        }
    }
}

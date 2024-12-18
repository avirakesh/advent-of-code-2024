use clap::Parser;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(Parser, Debug)]
struct Args {
    // Day to run
    #[arg(short, long)]
    day: u32,

    #[arg(short, long)]
    part: Option<u32>,

    // Overriden Input
    #[arg(short, long)]
    input_file: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1::main(args.part, args.input_file),
        2 => day2::main(args.part, args.input_file),
        3 => day3::main(args.part, args.input_file),
        4 => day4::main(args.part, args.input_file),
        5 => day5::main(args.part, args.input_file),
        6 => day6::main(args.part, args.input_file),
        7 => day7::main(args.part, args.input_file),
        8 => day8::main(args.part, args.input_file),
        9 => day9::main(args.part, args.input_file),
        10 => day10::main(args.part, args.input_file),
        11 => day11::main(args.part, args.input_file),
        12 => day12::main(args.part, args.input_file),
        13 => day13::main(args.part, args.input_file),
        14 => day14::main(args.part, args.input_file),
        15 => day15::main(args.part, args.input_file),
        16 => day16::main(args.part, args.input_file),
        17 => day17::main(args.part, args.input_file),
        18 => day18::main(args.part, args.input_file),
        _ => panic!("Invalid Day :("),
    }
}

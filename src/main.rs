use clap::Parser;
mod day1;
mod day2;

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
        _ => panic!("Invalid Day :("),
    }
}

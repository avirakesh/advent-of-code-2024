use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // Day to run
    #[arg(short, long)]
    day: u32,

    // Overriden Input
    #[arg(short, long)]
    input_file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    dbg!(args);
    println!("Hello, world!");
}

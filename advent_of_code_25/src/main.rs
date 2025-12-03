use advent_of_code_25::SOLVERS;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Which day to run (1–12)
    day: usize,

    /// Which part: 1 or 2
    part: usize,
}

fn main() {
    let args = Args::parse();

    let day_idx = args.day.checked_sub(1).expect("day must be >= 1");
    let part_idx = args.part.checked_sub(1).expect("part must be 1 or 2");

    let solver = SOLVERS
        .get(day_idx)
        .and_then(|day| day.get(part_idx))
        .expect("invalid day or part");
    solver();
}

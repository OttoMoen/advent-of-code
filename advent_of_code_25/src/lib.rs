pub mod day1;
pub mod day2;
pub mod day3;

type Solver = fn();

pub const SOLVERS: [&[Solver; 2]; 3] = [
    &[day1::part1, day1::part2],
    &[day2::part1, day2::part2],
    &[day3::part1, day3::part2],
];

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

type Solver = fn();

pub const SOLVERS: [&[Solver; 2]; 6] = [
    &[day1::part1, day1::part2],
    &[day2::part1, day2::part2],
    &[day3::part1, day3::part2],
    &[day4::part1, day4::part2],
    &[day5::part1, day5::part2],
    &[day6::part1, day6::part2],
];

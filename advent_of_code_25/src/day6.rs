use std::fs;

#[derive(Clone)]
pub struct MathProblem {
    pub numbers: Vec<u64>,
    pub symbol: char,
}

impl MathProblem {
    pub fn new(numbers: Vec<u64>, symbol: char) -> Self {
        MathProblem { numbers, symbol }
    }

    pub fn solve_problem(&self) -> u64 {
        if self.symbol == '+' {
            self.numbers.iter().sum()
        } else {
            self.numbers.iter().fold(1u64, |a, &b| a.wrapping_mul(b))
        }
    }
}

pub fn part1() {
    let contents = fs::read_to_string("data/day6.txt").expect("Could not read data");
    let mut total = 0;
    let mut number_rows: Vec<Vec<u64>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut symbol_row: Vec<char> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        let line = line.trim();
        let chars = line.split_whitespace();
        if i < 4 {
            for c in chars {
                number_rows[i].push(c.parse().expect("Invalid number"));
            }
        } else {
            for c in chars {
                symbol_row.push(c.chars().next().unwrap());
            }
        }
    }

    for ((((first, second), third), fourth), symbol) in number_rows[0]
        .iter()
        .zip(number_rows[1].iter())
        .zip(number_rows[2].iter())
        .zip(number_rows[3].iter())
        .zip(symbol_row.iter())
    {
        let problem = MathProblem::new(vec![*first, *second, *third, *fourth], *symbol);
        total += problem.solve_problem();
    }
    println!("Grand total: {total}");
}

pub fn part2() {
    let contents = fs::read_to_string("data/day6.txt").expect("Could not read data");
    let mut total = 0;
    let mut number_rows: Vec<Vec<char>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut symbol_row: Vec<char> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if i < 4 {
            let chars = line.chars();
            for c in chars {
                number_rows[i].push(c);
            }
        } else {
            let chars = line.split_whitespace();
            for c in chars {
                symbol_row.push(c.chars().next().unwrap());
            }
        }
    }

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut problem_numbers: Vec<u64> = Vec::new();
    for (((first, second), third), fourth) in number_rows[0]
        .iter()
        .zip(number_rows[1].iter())
        .zip(number_rows[2].iter())
        .zip(number_rows[3].iter())
    {
        let chars = [first, second, third, fourth];
        let s: String = chars.iter().copied().collect();
        if s == "    " {
            numbers.push(problem_numbers);
            problem_numbers = Vec::new();
        } else {
            problem_numbers.push(s.trim().parse().unwrap());
        }
    }
    numbers.push(problem_numbers);

    for (ns, symbol) in numbers.iter().zip(symbol_row.iter()) {
        let problem = MathProblem::new(ns.clone(), *symbol);
        total += problem.solve_problem();
    }

    println!("Grand total: {total}");
}

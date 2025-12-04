use std::fs;

pub fn part1() {
    let contents = fs::read_to_string("data/day3.txt").expect("Could not read data");
    let mut total: u64 = 0;
    for line in contents.split("\n") {
        total += largest_k_digit_number(line, 2)
    }
    println!("Sum of max pairs: {total}");
}

pub fn part2() {
    let contents = fs::read_to_string("data/day3.txt").expect("Could not read data");
    let mut total: u64 = 0;
    for line in contents.split("\n") {
        total += largest_k_digit_number(line, 12)
    }
    println!("Sum of max pairs: {total}");
}

fn largest_k_digit_number(s: &str, k: usize) -> u64 {
    let digits: Vec<u8> = s
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();

    let n = digits.len();
    if n < k {
        return 0;
    }

    let mut stack = Vec::with_capacity(k);

    for (i, d) in digits.iter().enumerate().take(n) {
        while stack.len() + (n - i) > k && !stack.is_empty() && stack.last().unwrap() < &d {
            stack.pop();
        }

        if stack.len() < k {
            stack.push(d);
        }
    }

    let mut result: u64 = 0;
    for &digit in &stack {
        result = result * 10 + *digit as u64;
    }
    result
}

use std::fs;

pub fn part1() {
    let contents = fs::read_to_string("data/day2.txt").expect("Could not read data");
    let mut total: u64 = 0;
    for line in contents.split(","){
        let parts: Vec<&str> = line.split("-").collect();
        let left: u64 = parts[0].parse().expect("Invalid number");
        let right: u64 = parts[1].parse().expect("Invalid number");

        for i in left..right+1 {
            let digits = i.ilog10() + 1;
            if digits % 2 !=0 {
                continue;
            }

            let half = digits / 2;
            let divisor = 10u64.pow(half);

            let left = i/divisor;
            let right = i % divisor;

            if left == right {
                total += i;
            }

        }
    }
    println!("Sum of invalid IDs: {total}")
}

pub fn part2() {
    let contents = fs::read_to_string("data/day2.txt").expect("Could not read data");
    let mut total: u64 = 0;
    for line in contents.split(","){
        let parts: Vec<&str> = line.split("-").collect();
        let left: u64 = parts[0].parse().expect("Invalid number");
        let right: u64 = parts[1].parse().expect("Invalid number");

        for i in left..right+1 {
            let s = i.to_string();
            let total_len = s.len();

            if (1..=total_len/2).any(|period| {
                total_len % period == 0 &&
                s[..period].repeat(total_len/period) == s
            }){
                total +=i;
            }

        }
    }
    println!("Sum of invalid IDs: {total}")
}
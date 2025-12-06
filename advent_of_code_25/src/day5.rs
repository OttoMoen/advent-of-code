use std::fs;

#[derive(Clone)]
pub struct Interval {
    pub low: u64,
    pub high: u64,
}

impl Interval {
    pub fn from_string(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = s.split("-").collect();
        let low: u64 = parts[0].parse().expect("Invalid number");
        let high: u64 = parts[1].parse().expect("Invalid number");
        Ok(Interval { low, high })
    }

    pub fn contains(&self, value: u64) -> bool {
        self.low <= value && value <= self.high
    }

    pub fn length(&self) -> u64 {
        self.high - self.low + 1
    }
}

pub fn merge_intervals(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return vec![];
    }
    intervals.sort_by_key(|i| i.low);

    let mut merged = Vec::with_capacity(intervals.len());
    let mut current = intervals[0].clone();

    for next in intervals {
        if current.high >= next.low {
            current.high = current.high.max(next.high);
        } else {
            merged.push(current);
            current = next;
        }
    }
    merged.push(current);
    merged
}

pub fn part1() {
    let contents = fs::read_to_string("data/day5.txt").expect("Could not read data");
    let mut total = 0;
    let mut first_half = true;
    let mut intervals: Vec<Interval> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            first_half = false;
            continue;
        }

        if first_half {
            intervals.push(Interval::from_string(line).unwrap());
        } else {
            ids.push(line.parse().expect("Invalid number"));
        }
    }

    let merged_intervals = merge_intervals(intervals);

    for id in ids {
        for interval in &merged_intervals {
            if interval.contains(id) {
                total += 1;
                break;
            }
        }
    }
    println!("Number of fresh ids: {total}");
}

pub fn part2() {
    let contents = fs::read_to_string("data/day5.txt").expect("Could not read data");
    let mut total = 0;
    let mut intervals: Vec<Interval> = Vec::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        intervals.push(Interval::from_string(line).unwrap());
    }

    let merged_intervals = merge_intervals(intervals);
    for interval in &merged_intervals {
        total += interval.length();
    }

    println!("Number of fresh ids: {total}");
}

use std::collections::HashMap;

use crate::utils::{Map, Pos};

pub fn part1() {
    let mut map = Map::from_file("data/day7.txt").unwrap();
    let mut total = 0;

    let mut positions: Vec<Pos> = Vec::new();
    let mut visited: Vec<Pos> = Vec::new();
    let start = map.find_char_in_row(0, 'S').unwrap();
    positions.push(Pos::new(start.row + 1, start.col));

    while let Some(curr_pos) = positions.pop() {
        visited.push(curr_pos);
        if curr_pos.row >= map.rows || curr_pos.col >= map.cols {
            continue;
        }
        if map.get_pos(curr_pos).unwrap() == '^' {
            let left = Pos::new(curr_pos.row, curr_pos.col - 1);
            let right = Pos::new(curr_pos.row, curr_pos.col + 1);
            if !visited.contains(&left) {
                positions.push(left);
            }
            if !visited.contains(&right) {
                positions.push(right);
            }
        } else if map.get_pos(curr_pos).unwrap() == '.' {
            map.set_pos(curr_pos, '|');
            let down = Pos::new(curr_pos.row + 1, curr_pos.col);
            if !visited.contains(&down) {
                positions.push(down);
            }
        }
    }

    for pos in map.positions() {
        if map.get_pos(pos).unwrap() == '^' {
            let up = Pos::new(pos.row - 1, pos.col);
            if map.get_pos(up).unwrap() == '|' {
                total += 1;
            }
        }
    }

    println!("Number of splits: {total}");
}

pub fn part2() {
    let map = Map::from_file("data/day7.txt").unwrap();
    let mut positions: Vec<Pos> = Vec::new();
    let mut visited: Vec<Pos> = Vec::new();
    let mut destinations: HashMap<Pos, Vec<Pos>> = HashMap::new();
    let start = map.find_char_in_row(0, 'S').unwrap();
    let first_split = Pos::new(start.row + 2, start.col);
    positions.push(first_split);

    while let Some(curr_start) = positions.pop() {
        let mut row = curr_start.row;
        loop {
            row += 1;
            let curr_pos = Pos::new(row, curr_start.col - 1);

            if curr_pos.row == map.rows - 1 {
                destinations.entry(curr_start).or_default().push(curr_pos);
                break;
            }

            if map.get_pos(curr_pos).unwrap() == '.' {
                continue;
            }

            if map.get_pos(curr_pos).unwrap() == '^' {
                if !visited.contains(&curr_pos) {
                    positions.push(curr_pos);
                    visited.push(curr_pos);
                }
                destinations.entry(curr_start).or_default().push(curr_pos);
                break;
            }
        }

        let mut row = curr_start.row;
        loop {
            let curr_pos = Pos::new(row, curr_start.col + 1);
            row += 1;

            if curr_pos.row == map.rows - 1 {
                destinations.entry(curr_start).or_default().push(curr_pos);
                break;
            }

            if map.get_pos(curr_pos).unwrap() == '.' {
                continue;
            }

            if map.get_pos(curr_pos).unwrap() == '^' {
                if !visited.contains(&curr_pos) {
                    positions.push(curr_pos);
                    visited.push(curr_pos);
                }
                destinations.entry(curr_start).or_default().push(curr_pos);
                break;
            }
        }
    }

    let mut memo = HashMap::new();
    fn dfs(p: Pos, g: &HashMap<Pos, Vec<Pos>>, memo: &mut HashMap<Pos, u64>) -> u64 {
        if let Some(&ways) = memo.get(&p) {
            return ways;
        }
        let ways = match g.get(&p) {
            None => 1,
            Some(children) => children.iter().map(|&next| dfs(next, g, memo)).sum(),
        };
        memo.insert(p, ways);
        ways
    }

    let total = dfs(first_split, &destinations, &mut memo);

    println!("Number of timelines: {total}");
}

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

pub struct Map {
    grid: Vec<String>,
    rows: usize,
    cols: usize,
}

impl Map {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let grid: Vec<String> = reader.lines().map_while(Result::ok).collect();

        let rows = grid.len();
        let cols = if rows > 0 { grid[0].len() } else { 0 };

        Ok(Map { grid, rows, cols })
    }

    pub fn get_pos(&self, pos: Pos) -> Option<char> {
        if pos.row < self.rows && pos.col < self.cols {
            self.grid[pos.row].chars().nth(pos.col)
        } else {
            None
        }
    }

    pub fn set_pos(&mut self, pos: Pos, new_char: char) {
        assert!(pos.row < self.rows, "row out of bounds");
        assert!(pos.col < self.cols, "col out of bounds");

        let row = &mut self.grid[pos.row];
        row.replace_range(pos.col..pos.col + 1, &new_char.to_string());
    }

    pub fn is_position_free(&self, row: usize, col: usize) -> bool {
        if row < self.rows && col < self.cols {
            let neighbor = self.grid[row].chars().nth(col).unwrap();
            neighbor == '.'
        } else {
            true
        }
    }

    pub fn count_free_neighbors8(&self, pos: Pos) -> i32 {
        let mut free_neighbors = 0;
        let row = pos.row as isize;
        let col = pos.col as isize;

        for dr in [-1, 0, 1] {
            for dc in [-1, 0, 1] {
                if dr == 0 && dc == 0 {
                    continue;
                }

                let nr = (row + dr) as usize;
                let nc = (col + dc) as usize;

                if self.is_position_free(nr, nc) {
                    free_neighbors += 1;
                }
            }
        }
        free_neighbors
    }

    pub fn positions(&self) -> impl Iterator<Item = Pos> {
        (0..self.rows).flat_map(move |r| (0..self.cols).map(move |c| Pos { row: r, col: c }))
    }
}

pub fn part1() {
    let map = Map::from_file("data/day4.txt").unwrap();
    let mut total = 0;
    for pos in map.positions() {
        let c = map.get_pos(pos).unwrap();
        if c == '@' && map.count_free_neighbors8(pos) >= 5 {
            total += 1;
        }
    }
    println!("Number of free rolls: {total}");
}

pub fn part2() {
    let mut map = Map::from_file("data/day4.txt").unwrap();
    let mut total = 0;
    loop {
        let mut to_remove = Vec::new();
        for pos in map.positions() {
            let c = map.get_pos(pos).unwrap();
            if c == '@' && map.count_free_neighbors8(pos) >= 5 {
                to_remove.push(pos);
            }
        }
        if to_remove.is_empty() {
            break;
        }

        for pos in to_remove {
            map.set_pos(pos, '.');
            total += 1;
        }
    }
    println!("Number of removed rolls: {total}");
}

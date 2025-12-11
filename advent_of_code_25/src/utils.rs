use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Pos { row, col }
    }
}

#[derive(Debug)]
pub struct Map {
    grid: Vec<String>,
    pub rows: usize,
    pub cols: usize,
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

    pub fn find_char_in_row(&self, row: usize, char_to_find: char) -> Option<Pos> {
        if row < self.rows {
            self.grid[row]
                .chars()
                .position(|c| c == char_to_find)
                .map(|pos| Pos::new(row, pos))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug)]
pub struct ParsePos3DError(String);

impl std::fmt::Display for ParsePos3DError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse Pos3D: {}", self.0)
    }
}

impl FromStr for Pos3D {
    type Err = ParsePos3DError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split(",").map(|s| s.trim()).collect();
        Ok(Pos3D {
            x: coords[0].parse::<i64>().unwrap(),
            y: coords[1].parse::<i64>().unwrap(),
            z: coords[2].parse::<i64>().unwrap(),
        })
    }
}

impl Pos3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Pos3D { x, y, z }
    }

    pub fn distance_to(&self, pos: &Pos3D) -> f64 {
        let dx = self.x - pos.x;
        let dy = self.y - pos.y;
        let dz = self.z - pos.z;

        let dist_squared = (dx as i128).pow(2) + (dy as i128).pow(2) + (dz as i128).pow(2);

        (dist_squared as f64).sqrt()
    }

    pub fn distance_squared_to(&self, pos: &Pos3D) -> i128 {
        let dx = self.x - pos.x;
        let dy = self.y - pos.y;
        let dz = self.z - pos.z;

        (dx as i128).pow(2) + (dy as i128).pow(2) + (dz as i128).pow(2)
    }
}

#[derive(Debug)]
pub struct Pos3DPairWithDistSq {
    pub a: Pos3D,
    pub b: Pos3D,
    pub dist_sq: i128,
}

impl Ord for Pos3DPairWithDistSq {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist_sq.cmp(&self.dist_sq) // max-heap: largest dist_sq pops first
    }
}

impl PartialOrd for Pos3DPairWithDistSq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Pos3DPairWithDistSq {}
impl PartialEq for Pos3DPairWithDistSq {
    fn eq(&self, other: &Self) -> bool {
        self.dist_sq == other.dist_sq
    }
}

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub struct Node {
    pub id: u64,
    pub cost: u32,
    pub coord: Coord,
    pub dir: Dir,
    pub num_steps: u32,
}

impl Node {
    pub fn new() -> Node {
        Node{ 
            id: 0, 
            cost: 0, 
            coord: Coord{ 
                row: 0, 
                col: 0 
            }, 
            dir: Dir::No, 
            num_steps: 0
        }
    }

    pub fn new_with_hash(
        cost: u32, 
        coord: Coord, 
        dir: Dir, 
        num_steps: u32
    ) -> Node {
        let hash = Self::calculate_hash(&coord, &dir, num_steps);
        Node{
            id: hash,
            cost: cost,
            coord: coord,
            dir: dir,
            num_steps: num_steps,
        }

    }
    
    fn calculate_hash(coord: &Coord, dir: &Dir, num_steps: u32) -> u64 {
        let mut hasher = DefaultHasher::new();
        coord.hash(&mut hasher);
        dir.hash(&mut hasher);
        num_steps.hash(&mut hasher);
        hasher.finish()
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Hash)]
#[derive(Eq, PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

impl Coord {
    pub fn is_out_of_bounds(&self, rows: i32, cols: i32) -> bool {
        if self.row < 0 || self.row >= rows || self.col < 0 || self.col >= cols {
            return true;
        }
        false 
    }

    pub fn move_it(&self, dir: Dir) -> Coord {
        match dir {
            Dir::Up =>  return Coord{ row: self.row - 1, col: self.col },
            Dir::Right => return Coord{ row: self.row, col: self.col + 1 },
            Dir::Down => return Coord{ row: self.row + 1, col: self.col },
            Dir::Left => return Coord{ row: self.row, col: self.col - 1 },
            Dir::No => return Coord{ row: self.row, col: self.col },
        }
    }

    pub fn get_cost(&self, grid: &Vec<Vec<u32>>) -> u32 {
        if self.is_out_of_bounds(grid.len() as i32, grid.first().map_or(0, Vec::len) as i32) {
            return 0;
        }
        grid[self.row as usize][self.col as usize]
    }
}

#[derive(Hash)]
#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub enum Dir {
    Right,
    Left,
    Up,
    Down,
    No,
}

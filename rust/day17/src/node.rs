use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Eq, PartialEq)]
pub struct Node {
    pub id: u64,
    pub pred_id: u64,
    pub cost: u32,
    pub coord: Coord,
    pub dir: Dir,
    pub num_steps: u32,
}

impl Node {
    pub fn new() -> Node {
        Node{ 
            id: 0, 
            pred_id: 0, 
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
        pred_id: u64, 
        cost: u32, 
        coord: Coord, 
        dir: Dir, 
        num_steps: u32
    ) -> Node {
        let hash = Self::calculate_hash(coord, dir, num_steps);
        Node{
            id: hash,
            pred_id: pred_id,
            cost: cost,
            coord: coord,
            dir: dir,
            num_steps = num_steps,
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
struct Coord {
    row: u32,
    col: u32,
}

#[derive(Hash)]
#[derive(Eq, PartialEq)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
    No,
}

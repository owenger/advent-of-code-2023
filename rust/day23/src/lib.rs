use std::fs;
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let grid = parse_input(input);
    let res = dijkstra(&grid);
    Ok(())
}

fn dijkstra(grid: &Vec<Vec<char>>) -> i32 {
    let mut visited: Vec<Node> = Vec::new();
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let start_col = get_start_col(grid);
    let start_node = Node{ cost: 0, coord: (0, start_col), history: Vec::new()};
    heap.push(start_node);
    0
}

fn get_start_col(grid: &Vec<Vec<char>>) -> usize {
    for i in 0..grid.first().map_or(0, Vec::len) {
        if grid[0][i] == '.' {
            return i;
        }
    }
    println!("Could not find starting col.");
    0
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct Node {
    cost: u32,
    coord: (usize, usize),
    history: Vec<(usize, usize)>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

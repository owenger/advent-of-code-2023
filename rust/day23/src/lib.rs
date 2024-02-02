use std::fs;
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let grid = parse_input(input);
    let res = dijkstra(&grid);
    println!("Res: {res}");
    Ok(())
}

fn dijkstra(grid: &Grid) -> u32 {
    let mut visited: Vec<Node> = Vec::new();
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let start_col = get_start_col(&grid.grid);
    let start_node = Node{ cost: 0, coord: (0, start_col), history: vec![(0, start_col)]};
    heap.push(start_node);
    
    loop {
        if heap.len() == 0 {
            break;
        }
        let cur_node = heap.pop().unwrap();
        println!("{:?}", cur_node);

        if cur_node.coord.0 == grid.rows - 1 {
            return cur_node.cost;
        }

        if grid.can_go(&cur_node.coord, Dir::Right) {
            let new_coord = (cur_node.coord.0, cur_node.coord.1 + 1);
            if !cur_node.history.contains(&new_coord) {
                let mut new_history = cur_node.history.clone();
                new_history.push(new_coord);
                heap.push(Node{
                    cost: cur_node.cost + 1,
                    coord: new_coord,
                    history: new_history,
                })
            }
        }
        if grid.can_go(&cur_node.coord, Dir::Down) {
            let new_coord = (cur_node.coord.0 + 1, cur_node.coord.1);
            if !cur_node.history.contains(&new_coord) {
                let mut new_history = cur_node.history.clone();
                new_history.push(new_coord);
                heap.push(Node{
                    cost: cur_node.cost + 1,
                    coord: new_coord,
                    history: new_history,
                })
            }
        } 
        if grid.can_go(&cur_node.coord, Dir::Left) {
            let new_coord = (cur_node.coord.0, cur_node.coord.1 - 1);
            if !cur_node.history.contains(&new_coord) {
                let mut new_history = cur_node.history.clone();
                new_history.push(new_coord);
                heap.push(Node{
                    cost: cur_node.cost + 1,
                    coord: new_coord,
                    history: new_history,
                })
            }
        }
        if grid.can_go(&cur_node.coord, Dir::Up) {
            let new_coord = (cur_node.coord.0 - 1, cur_node.coord.1);
            if !cur_node.history.contains(&new_coord) {
                let mut new_history = cur_node.history.clone();
                new_history.push(new_coord);
                heap.push(Node{
                    cost: cur_node.cost + 1,
                    coord: new_coord,
                    history: new_history,
                })
            }
        }

    }
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

fn parse_input(input: String) -> Grid {
    Grid::from_grid(input.lines().map(|line| line.chars().collect()).collect())
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

struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn from_grid(grid: Vec<Vec<char>>) -> Grid {
        Grid{
            rows: grid.len(),
            cols: grid.first().map_or(0, Vec::len),
            grid: grid,
        }
    }

    pub fn can_go(&self, coord: &(usize, usize), dir: Dir) -> bool {
        match dir {
            Dir::Right => {
                if coord.1 >= self.cols - 1 {
                    return false;
                }
                match self.grid[coord.0][coord.1 + 1] {
                    '.' => return true,
                    '>' => return true,
                    _ => return false,
                }
            }
            Dir::Down => {
                if coord.0 >= self.rows - 1 {
                    return false;
                }
                match self.grid[coord.0 + 1][coord.1] {
                    '.' => return true,
                    'v' => return true,
                    _ => return false,
                }
            }
            Dir::Left => {
                if coord.1 == 0 {
                    return false;
                }
                match self.grid[coord.0][coord.1 - 1] {
                    '.' => return true,
                    '<' => return true,
                    _ => return false,
                }
            }
            Dir::Up => {
                if coord.0 == 0 {
                    return false;
                }
                match self.grid[coord.0 - 1][coord.1] {
                    '.' => return true,
                    '^' => return true,
                    _ => return false,
                }
            }
        }
    }

}

enum Dir {
    Right,
    Down,
    Left,
    Up
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

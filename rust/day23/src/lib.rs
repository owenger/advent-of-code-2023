use std::fs;
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let grid = parse_input(input);
    let res = dijkstra(&grid);
    println!("Res: {res}");
    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let mut grid = parse_input(input);
    remove_slopes(&mut grid.grid);
    let node_net = grid.get_node_net();
    println!("{:?}", node_net);
    //println!("{},{}", grid.rows, grid.cols);

    // let res = dijkstra(&grid);
    // println!("Res: {res}");
    Ok(())
}

fn dijkstra(grid: &Grid) -> u32 {
    let mut visited: Vec<Node> = Vec::new();
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut costs: Vec<Vec<u32>> = vec![vec![0; grid.cols]; grid.rows];
    let start_col = grid.get_start_col();
    let start_node = Node{ cost: 0, coord: Coord{ row: 0, col: start_col }, history: vec![Coord{ row: 0, col: start_col }]};
    heap.push(start_node);
    let mut res_cost: u32 = 0;
    let mut biggest_history: Vec<Coord> = Vec::new();
    let mut prev_cost = 0;
    let mut max_dist: usize = 0;
    
    loop {
        if heap.len() == 0 {
            break;
        }
        let cur_node = heap.pop().unwrap();
        let cur_dist = cur_node.coord.row + cur_node.coord.col;
        if cur_dist > max_dist {
            max_dist = cur_dist;
            println!("Max dist: {max_dist}");
        }
        //costs[new_coord.0][new_coord.1] = new_cost;
        if cur_node.coord.row == grid.rows - 1 {
            if cur_node.cost > res_cost {
                res_cost = cur_node.cost;
                biggest_history = cur_node.history.clone();
            }
        }

        if grid.can_go(&cur_node.coord, Dir::Right) {
            let new_coord = Coord{ row: cur_node.coord.row, col: cur_node.coord.col + 1};
            let new_cost = cur_node.cost + 1;
            if !cur_node.history.contains(&new_coord) && new_cost > costs[new_coord.row][new_coord.col] {
                let mut new_history = cur_node.history.clone();
                new_history.push(new_coord.clone());
                heap.push(Node{
                    cost: cur_node.cost + 1,
                    coord: new_coord,
                    history: new_history,
                });
            }
        }
        if grid.can_go(&cur_node.coord, Dir::Down) {
            let new_coord = Coord{ row: cur_node.coord.row + 1, col: cur_node.coord.col };
            let new_cost = cur_node.cost + 1;
            if !cur_node.history.contains(&new_coord) && new_cost > costs[new_coord.row][new_coord.col] {
                let mut new_history = cur_node.history.clone();
                new_history.push(new_coord.clone());
                heap.push(Node{
                    cost: cur_node.cost + 1,
                    coord: new_coord,
                    history: new_history,
                });
            }
        } 
        if grid.can_go(&cur_node.coord, Dir::Left) {
            let new_coord = Coord{ row: cur_node.coord.row, col: cur_node.coord.col - 1 };
            let new_cost = cur_node.cost + 1;
            if !cur_node.history.contains(&new_coord) && new_cost > costs[new_coord.row][new_coord.col] {
                let mut new_history = cur_node.history.clone();
                new_history.push(new_coord.clone());
                heap.push(Node{
                    cost: cur_node.cost + 1,
                    coord: new_coord,
                    history: new_history,
                });
            }
        }
        if grid.can_go(&cur_node.coord, Dir::Up) {
            let new_coord = Coord{ row: cur_node.coord.row - 1, col: cur_node.coord.col };
            let new_cost = cur_node.cost + 1;
            if !cur_node.history.contains(&new_coord) && new_cost > costs[new_coord.row][new_coord.col] {
                let mut new_history = cur_node.history.clone();
                new_history.push(new_coord.clone());
                heap.push(Node{
                    cost: cur_node.cost + 1,
                    coord: new_coord,
                    history: new_history,
                });
            }
        }
    }
    //print_history(&biggest_history, grid.rows, grid.cols);
    res_cost
}


fn parse_input(input: String) -> Grid {
    Grid::from_grid(input.lines().map(|line| line.chars().collect()).collect())
}

fn remove_slopes(input: &mut Vec<Vec<char>>) {
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '#' {
                continue;
            }
            input[i][j] = '.';
        }
    }
}

fn print_history(history: &Vec<(usize, usize)>, rows: usize, cols: usize) {
    let mut show_vec: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    for moment in history {
        show_vec[moment.0][moment.1] = '#';
    }
    for row in show_vec {
        println!("{:?}", row);
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct Node {
    cost: u32,
    coord: Coord,
    history: Vec<Coord>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(Debug)]
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

    pub fn can_go(&self, coord: &Coord, dir: Dir) -> bool {
        match dir {
            Dir::Right => {
                if coord.col >= self.cols - 1 {
                    return false;
                }
                match self.grid[coord.row][coord.col + 1] {
                    '.' => return true,
                    '>' => return true,
                    _ => return false,
                }
            }
            Dir::Down => {
                if coord.row >= self.rows - 1 {
                    return false;
                }
                match self.grid[coord.row + 1][coord.col] {
                    '.' => return true,
                    'v' => return true,
                    _ => return false,
                }
            }
            Dir::Left => {
                if coord.col == 0 {
                    return false;
                }
                match self.grid[coord.row][coord.col - 1] {
                    '.' => return true,
                    '<' => return true,
                    _ => return false,
                }
            }
            Dir::Up => {
                if coord.row == 0 {
                    return false;
                }
                match self.grid[coord.row - 1][coord.col] {
                    '.' => return true,
                    '^' => return true,
                    _ => return false,
                }
            }
            Dir::No => return true,
        }
    }

    pub fn get_node_net(&self) -> HashMap<Coord, Vec<(Coord, u32)>> {
        let mut nodes: HashMap<Coord, Vec<(Coord, u32)>> = HashMap::new(); 
        let mut nodes_to_check: Vec<Coord> = Vec::new();
        let mut nodes_checked: Vec<Coord> = Vec::new();

        nodes_to_check.push(Coord{ row: 0, col: self.get_start_col() });

        loop {
            if nodes_to_check.len() == 0 {
                break;
            }
            let cur_node = nodes_to_check.pop().expect("No node in checklist");
            nodes_checked.push(cur_node.clone());

            if self.can_go(&cur_node, Dir::Right) {
                let right_coord = Coord{ row: cur_node.row, col: cur_node.col + 1 };
                let (right_cost, right_end) = self.run_path(&right_coord, Dir::Left, &mut nodes_to_check, &nodes_checked);
                nodes.entry(cur_node.clone()).or_insert_with(Vec::new).push((right_end, right_cost));
            }
            if self.can_go(&cur_node, Dir::Down) {
                let down_coord = Coord{ row: cur_node.row + 1, col: cur_node.col };
                let (down_cost, down_end) = self.run_path(&down_coord, Dir::Up, &mut nodes_to_check, &nodes_checked);
                nodes.entry(cur_node.clone()).or_insert_with(Vec::new).push((down_end, down_cost));
            }
            if self.can_go(&cur_node, Dir::Left) {
                let left_coord = Coord{ row: cur_node.row, col: cur_node.col - 1 };
                let (left_cost, left_end) = self.run_path(&left_coord, Dir::Right, &mut nodes_to_check, &nodes_checked);
                nodes.entry(cur_node.clone()).or_insert_with(Vec::new).push((left_end, left_cost));
            }
            if self.can_go(&cur_node, Dir::Up) {
                let up_coord = Coord{ row: cur_node.row - 1, col: cur_node.col };
                let (up_cost, up_end) = self.run_path(&up_coord, Dir::Down, &mut nodes_to_check, &nodes_checked);
                nodes.entry(cur_node.clone()).or_insert_with(Vec::new).push((up_end, up_cost));
            }
        }
        nodes
    }

    fn run_path(&self, coord: &Coord, from_dir: Dir, nodes_to_check: &mut Vec<Coord>, nodes_checked: &Vec<Coord>) -> (u32, Coord) {
        let mut cost: u32 = 1;
        let mut end_node: Coord = Coord{ row: 0, col: 0 };
        if self.number_of_non_neighbours(&coord, '#')  > 2 {
            if !nodes_checked.contains(&coord) {
                nodes_to_check.push(coord.clone());
            }
            end_node = coord.clone();
            return (cost, end_node);
        }
        if from_dir != Dir::Right && self.can_go(&coord, Dir::Right) {
            let (plus_cost, end_node) = self.run_path(&Coord{ row: coord.row, col: coord.col + 1 }, Dir::Left, nodes_to_check, &nodes_checked);
            cost += plus_cost;
        }
        if from_dir != Dir::Down && self.can_go(&coord, Dir::Down) {
            let (plus_cost, end_node) = self.run_path(&Coord{ row: coord.row + 1, col: coord.col }, Dir::Up, nodes_to_check, &nodes_checked);
            cost += plus_cost;
        }
        if from_dir != Dir::Left && self.can_go(&coord, Dir::Left) {
            let (plus_cost, end_node) = self.run_path(&Coord{ row: coord.row, col: coord.col - 1 }, Dir::Right, nodes_to_check, &nodes_checked);
            cost += plus_cost;
        }
        if from_dir != Dir::Up && self.can_go(&coord, Dir::Up) {
            let (plus_cost, end_node) = self.run_path(&Coord{ row: coord.row - 1, col: coord.col }, Dir::Down, nodes_to_check, &nodes_checked);
            cost += plus_cost;
        }
        (cost, end_node)
    }

    fn number_of_non_neighbours(&self, coord: &Coord, non_value: char) -> u32 {
        let mut total: u32 = 0;
        if coord.row > 0 && self.grid[coord.row - 1][coord.col] != non_value {
            total += 1;
        }
        if coord.col > 0 && self.grid[coord.row][coord.col - 1] != non_value {
            total += 1;
        }
        if coord.row < self.rows - 1 && self.grid[coord.row + 1][coord.col] != non_value {
            total += 1;
        }
        if coord.col < self.cols - 1 && self.grid[coord.row][coord.col + 1] != non_value {
            total += 1;
        }
        total
    }

    pub fn get_start_col(&self) -> usize {
        for i in 0..self.grid.first().map_or(0, Vec::len) {
            if self.grid[0][i] == '.' {
                return i;
            }
        }
        println!("Could not find starting col.");
        0
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
#[derive(Hash)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Eq, PartialEq)]
#[derive(Debug)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
    No
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

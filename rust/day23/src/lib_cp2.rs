use std::fs;
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let grid = parse_input(input);
    let node_net = grid.get_node_net();
    println!("{:?}", node_net);
    let start_col = grid.get_start_col();
    let start_coord = Coord{ row: 0, col: start_col };
    let res = dijkstrav2(&node_net, start_coord, grid.rows - 1);
    println!("Res: {res}");
    //let res = dijkstra(&grid);
    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let mut grid = parse_input(input);
    remove_slopes(&mut grid.grid);
    let node_net = grid.get_node_net();
    let start_col = grid.get_start_col();
    let start_coord = Coord{ row: 0, col: start_col };
    let res = dijkstrav2(&node_net, start_coord, grid.rows - 1);

    // let res = dijkstra(&grid);
    println!("Res: {res}");
    Ok(())
}

fn dijkstrav2(net: &HashMap<Coord, Vec<(Coord, u32)>>, start_coord: Coord, end_row: usize) -> u32 {
    let mut node_vec: Vec<Node> = Vec::new();
    let start_node = Node{ cost: 0, coord: start_coord.clone(), history: vec![start_coord.clone()]};
    node_vec.push(start_node);
    let mut res_cost: u32 = 0;
    let mut biggest_history: Vec<Coord> = Vec::new();
    let mut count: u32 = 0;

    loop {
        if node_vec.len() == 0 {
            break;
        }
        count += 1;
        if count % 1000000 == 0 {
            println!("Count: {count}");
        }
        let cur_node = node_vec.pop().expect("Heap is empty");
        if cur_node.coord.row == end_row {
            if cur_node.cost > res_cost {
                res_cost = cur_node.cost;
                println!("Res cost: {res_cost}");
                biggest_history = cur_node.history.clone();
            }
        }
        if !net.contains_key(&cur_node.coord) {
            continue;
        }
        let candidates = net.get(&cur_node.coord).unwrap().clone();
        if candidates.len() == 1 && candidates[0].0 == (Coord{ row: 0, col: 0 }) {
            continue;
        }
        for candidate in &candidates {
            if !cur_node.history.contains(&candidate.0) {
                let mut new_history = cur_node.history.clone();
                new_history.push(candidate.0.clone());
                let new_node = Node{
                    cost: cur_node.cost + candidate.1,
                    coord: candidate.0.clone(),
                    history: new_history,
                };
                node_vec.push(new_node);
            }
        }
    }
    println!("{:?}", biggest_history);
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

fn print_history(history: &Vec<Coord>, rows: usize, cols: usize) {
    let mut show_vec: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    for moment in history {
        show_vec[moment.row][moment.col] = '#';
    }
    for row in show_vec {
        println!("{:?}", row);
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
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
struct SparseGrid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
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
            let debug = cur_node == Coord{ row: 140, col: 139 };

            let can_go_right = self.can_go(&cur_node, Dir::Right);
            let can_go_down = self.can_go(&cur_node, Dir::Down);
            let mut can_go_left = self.can_go(&cur_node, Dir::Left);
            let mut can_go_up = self.can_go(&cur_node, Dir::Up);

            can_go_left &= can_go_down;
            can_go_up &= can_go_right;


            if can_go_right {
                let right_coord = Coord{ row: cur_node.row, col: cur_node.col + 1 };
                let (right_cost, right_end) = self.run_path(&right_coord, Dir::Left, &mut nodes_to_check, &nodes_checked, debug);
                nodes.entry(cur_node.clone()).or_insert_with(Vec::new).push((right_end, right_cost));
            }
            if can_go_down {
                let down_coord = Coord{ row: cur_node.row + 1, col: cur_node.col };
                let (down_cost, down_end) = self.run_path(&down_coord, Dir::Up, &mut nodes_to_check, &nodes_checked, debug);
                nodes.entry(cur_node.clone()).or_insert_with(Vec::new).push((down_end, down_cost));
            }
            if can_go_left {
                let left_coord = Coord{ row: cur_node.row, col: cur_node.col - 1 };
                let (left_cost, left_end) = self.run_path(&left_coord, Dir::Right, &mut nodes_to_check, &nodes_checked, debug);
                nodes.entry(cur_node.clone()).or_insert_with(Vec::new).push((left_end, left_cost));
            }
            if can_go_up {
                let up_coord = Coord{ row: cur_node.row - 1, col: cur_node.col };
                let (up_cost, up_end) = self.run_path(&up_coord, Dir::Down, &mut nodes_to_check, &nodes_checked, debug);
                nodes.entry(cur_node.clone()).or_insert_with(Vec::new).push((up_end, up_cost));
            }
        }
        nodes
    }

    fn run_path(&self, coord: &Coord, from_dir: Dir, nodes_to_check: &mut Vec<Coord>, nodes_checked: &Vec<Coord>, debug: bool) -> (u32, Coord) {
        let mut cost: u32 = 1;
        let mut end_node: Coord = Coord{ row: 0, col: 0 };
        if self.number_of_non_neighbours(&coord, '#')  > 2 || coord.row == 0 || coord.row == self.rows - 1 {
            if !nodes_checked.contains(&coord) {
                nodes_to_check.push(coord.clone());
            }
            end_node = coord.clone();
            return (cost, end_node);
        }
        if from_dir != Dir::Right && self.can_go(&coord, Dir::Right) {
            let result = self.run_path(&Coord{ row: coord.row, col: coord.col + 1 }, Dir::Left, nodes_to_check, &nodes_checked, debug);
            let plus_cost = result.0;
            end_node = result.1;
            cost += plus_cost;
        }
        if from_dir != Dir::Down && self.can_go(&coord, Dir::Down) {
            let result = self.run_path(&Coord{ row: coord.row + 1, col: coord.col }, Dir::Up, nodes_to_check, &nodes_checked, debug);
            let plus_cost = result.0;
            end_node = result.1;
            cost += plus_cost;
        }
        if from_dir != Dir::Left && self.can_go(&coord, Dir::Left) {
            let result = self.run_path(&Coord{ row: coord.row, col: coord.col - 1 }, Dir::Right, nodes_to_check, &nodes_checked, debug);
            let plus_cost = result.0;
            end_node = result.1;
            cost += plus_cost;
        }
        if from_dir != Dir::Up && self.can_go(&coord, Dir::Up) {
            let result = self.run_path(&Coord{ row: coord.row - 1, col: coord.col }, Dir::Down, nodes_to_check, &nodes_checked, debug);
            let plus_cost = result.0;
            end_node = result.1;
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

use std::fs;
use std::error::Error;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let grid = parse_input(input);
    let node_net = grid.get_node_net();
    let start_col = grid.get_start_col();
    let end_col = grid.get_end_col();
    let start_coord = Coord{ row: 0, col: start_col };
    let end_coord = Coord { row: grid.rows - 1, col: end_col };
    let mut history: HashMap<u16, (bool, u8)> = HashMap::new();
    let res = dfs(&node_net, &mut history, start_coord.hash(), end_coord.hash(), 0, 0);
    println!("Res: {res}");
    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let mut grid = parse_input(input);
    remove_slopes(&mut grid.grid);
    let node_net = grid.get_node_net();
    let start_col = grid.get_start_col();
    let end_col = grid.get_end_col();
    let start_coord = Coord{ row: 0, col: start_col };
    let end_coord = Coord { row: grid.rows - 1, col: end_col };
    let mut history: HashMap<u16, (bool, u8)> = HashMap::new();
    let res = dfs(&node_net, &mut history, start_coord.hash(), end_coord.hash(), 0, 0);
    println!("Res: {res}");
    Ok(())
}

fn dfs(net: &HashMap<u16, Vec<(u16, u32)>>, history: &mut HashMap<u16, (bool, u8)>, hash: u16, end_hash: u16, cost: u32, depth: u8) -> u32 {
    if hash == end_hash {
        return cost;
    }

    if let Some(&have_been) = history.get(&hash) {
        if have_been.0 {
            return 0;
        }
    }
    history.insert(hash, (true, depth));

    let mut highest_cost: u32 = 0;

    if let Some(candidates) = net.get(&hash) {
        for candidate in candidates {
            let cost_candidate = dfs(net, history, candidate.0, end_hash, cost + candidate.1, depth + 1);
            if cost_candidate > highest_cost {
                highest_cost = cost_candidate;
            }
        }
    }
    history.insert(hash, (false, 0));
    highest_cost
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

fn print_history_dict(history: &HashMap<u16, (bool, u8)>, rows: usize, cols: usize ) {
    let mut vec: Vec<(Coord, u8)> = Vec::new();
    for (key, value) in history {
        if value.0 {
            vec.push((Coord::unhash(*key), value.1));
        }
    }

    let mut show_vec: Vec<Vec<String>> = vec![vec![String::from("."); cols]; rows];
    for moment in vec {
        show_vec[moment.0.row][moment.0.col] = moment.1.to_string();
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

    pub fn get_node_net(&self) -> HashMap<u16, Vec<(u16, u32)>> {
        let mut nodes: HashMap<u16, Vec<(u16, u32)>> = HashMap::new(); 
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
            let can_go_left_prov = self.can_go(&cur_node, Dir::Left);
            let can_go_up_prov = self.can_go(&cur_node, Dir::Up);
            let can_go_left = can_go_left_prov && can_go_down && can_go_up_prov;
            let can_go_up = can_go_up_prov && can_go_right && can_go_left_prov;

            if can_go_right {
                let right_coord = Coord{ row: cur_node.row, col: cur_node.col + 1 };
                let (right_cost, right_end) = self.run_path(&right_coord, Dir::Left, &mut nodes_to_check, &nodes_checked, debug);
                nodes.entry(cur_node.hash()).or_insert_with(Vec::new).push((right_end.hash(), right_cost));
            }
            if can_go_down {
                let down_coord = Coord{ row: cur_node.row + 1, col: cur_node.col };
                let (down_cost, down_end) = self.run_path(&down_coord, Dir::Up, &mut nodes_to_check, &nodes_checked, debug);
                nodes.entry(cur_node.hash()).or_insert_with(Vec::new).push((down_end.hash(), down_cost));
            }
            if can_go_left {
                let left_coord = Coord{ row: cur_node.row, col: cur_node.col - 1 };
                let (left_cost, left_end) = self.run_path(&left_coord, Dir::Right, &mut nodes_to_check, &nodes_checked, debug);
                nodes.entry(cur_node.hash()).or_insert_with(Vec::new).push((left_end.hash(), left_cost));
            }
            if can_go_up {
                let up_coord = Coord{ row: cur_node.row - 1, col: cur_node.col };
                let (up_cost, up_end) = self.run_path(&up_coord, Dir::Down, &mut nodes_to_check, &nodes_checked, debug);
                nodes.entry(cur_node.hash()).or_insert_with(Vec::new).push((up_end.hash(), up_cost));
            }
        }
        nodes
    }

    fn run_path(&self, coord: &Coord, from_dir: Dir, nodes_to_check: &mut Vec<Coord>, nodes_checked: &Vec<Coord>, debug: bool) -> (u32, Coord) {
        let mut cost: u32 = 1;
        let mut end_node: Coord = Coord{ row: 0, col: 0 };
        if self.number_of_non_neighbours(&coord, '#')  > 2 || coord.row == 0 || coord.row == self.rows - 1 {
            if !nodes_checked.contains(&coord) && !nodes_to_check.contains(&coord) {
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

    pub fn get_end_col(&self) -> usize {
        for i in 0..self.grid.first().map_or(0, Vec::len) {
            if self.grid[self.rows - 1][i] == '.' {
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

impl Coord {
    pub fn hash(&self) -> u16 {
        (self.row * 200 + self.col) as u16
    }

    pub fn unhash(number: u16) -> Coord {
        Coord{ row: (number / 200) as usize, col: (number % 200) as usize }
    }
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

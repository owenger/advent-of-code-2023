use std::fs;
use std::error::Error;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

const MAX_INPUT: i32 = 26501365;
const FINDABLE_FIELDS: i32 = 7498;
const MIN_STEPS_EDGE: i32 = 181;
const MIN_STEPS_CORNER: i32 = 259;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let max_steps = 64;
    let input = fs::read_to_string(input_path)?;
    let mut grid = parse_input(input);
    let start_row = grid.start_row;
    let start_col = grid.start_col;
    let total = walk_grid(&mut grid, start_row, start_col, max_steps);
    println!("Total: {total}");
    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input_nr: i32 = MAX_INPUT;
    let input = fs::read_to_string(input_path)?;
    let mut grid = parse_input(input);
    let mut total: i32 = FINDABLE_FIELDS;
    let number_of_1d: i32 = (150 - (grid.rows + 1) / 2) / grid.rows;
    let remainder_1d: i32 = (150 - (grid.rows + 1) / 2) % grid.rows;
    println!("{number_of_1d}, {remainder_1d}");
    return Ok(());

    // going up
    grid.start_row = grid.rows - 1;
    let mut steps_left = MAX_INPUT - (grid.rows + 1) / 2;
    loop {
        if steps_left <= 0 {
            break;
        }
        if steps_left >= MIN_STEPS_EDGE {
            total += FINDABLE_FIELDS;
            steps_left -= grid.rows;
            continue;
        }
        total += dijkstras(&mut grid, input_nr);
        steps_left -= grid.rows;
    }

    // going down
    grid.start_row = 0;
    steps_left = MAX_INPUT - (grid.rows + 1) / 2;
    loop {
        if steps_left <= 0 {
            break;
        }
        if steps_left >= MIN_STEPS_EDGE {
            total += FINDABLE_FIELDS;
            steps_left -= grid.rows;
            continue;
        }
        total += dijkstras(&mut grid, input_nr);
        steps_left -= grid.rows;
    }



    println!("Total: {total}");
    // grid.start_row = grid.start_row;
    // grid.start_col = grid.cols - 1;
    // total = dijkstras(&mut grid, input_nr);
    // println!("Total: {total}");
    Ok(())
}

fn dijkstras(grid: &mut Grid, max_steps: i32) -> i32 {
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut heap = BinaryHeap::new();
    heap.push(Cell{ row: grid.start_row, col: grid.start_col, cost: max_steps});
    let mut count = 0;

    loop {
        if heap.len() == 0 {
            break;
        }
        let cur_cell = heap.pop().unwrap();
        if visited.contains(&(cur_cell.row, cur_cell.col)) {
            continue;
        }
        visited.push((cur_cell.row, cur_cell.col));
        if cur_cell.cost % 2 == 0 {
            count += 1;
        }
        if count == FINDABLE_FIELDS {
            println!("Max steps: {}", max_steps - cur_cell.cost);
            return count;
        }
        if cur_cell.cost == 0 {
            continue;
        }
        if cur_cell.row > 0 {
            let next_cell = Cell{ row: cur_cell.row - 1, col: cur_cell.col, cost: cur_cell.cost - 1 };
            if grid.grid[next_cell.row as usize][next_cell.col as usize] && !visited.contains(&(next_cell.row, next_cell.col)) {
                heap.push(next_cell);
            }
        }
        if cur_cell.col > 0 {
            let next_cell = Cell{ row: cur_cell.row, col: cur_cell.col - 1, cost: cur_cell.cost - 1 };
            if grid.grid[next_cell.row as usize][next_cell.col as usize] && !visited.contains(&(next_cell.row, next_cell.col)) {
                heap.push(next_cell);
            }
        }
        if cur_cell.row < grid.rows - 1 {
            let next_cell = Cell{ row: cur_cell.row + 1, col: cur_cell.col, cost: cur_cell.cost - 1 };
            if grid.grid[next_cell.row as usize][next_cell.col as usize] && !visited.contains(&(next_cell.row, next_cell.col)) {
                heap.push(next_cell);
            }
        }
        if cur_cell.col < grid.cols - 1 {
            let next_cell = Cell{ row: cur_cell.row, col: cur_cell.col + 1, cost: cur_cell.cost - 1 };
            if grid.grid[next_cell.row as usize][next_cell.col as usize] && !visited.contains(&(next_cell.row, next_cell.col)) {
                heap.push(next_cell);
            }
        }
    }
    count
}

fn walk_grid(grid: &mut Grid, row: i32, col: i32, count: i32) -> i32 {
    if row < 0 
    || row >= grid.rows 
    || col < 0 
    || col >= grid.cols
    || !grid.grid[row as usize][col as usize] {
        return 0;
    }
    let mut total: i32 = 0;
    if grid.hash_map.contains_key(&(row, col)) {
        if grid.hash_map.get(&(row, col)).unwrap() >= &count {
            return 0;
        }
        grid.hash_map.insert((row, col), count);
    } else {
        grid.hash_map.insert((row, col), count);
        if count % 2 == 0 {
            total = 1;
        }
    }
    if count == 0 {
        return 1;
    }
    total += walk_grid(grid, row + 1, col, count - 1);
    if total == FINDABLE_FIELDS {
        if count > grid.max_steps_to_go {
            grid.max_steps_to_go = count;
        }
        return total;
    }
    total += walk_grid(grid, row - 1, col, count - 1);
    if total == FINDABLE_FIELDS {
        if count > grid.max_steps_to_go {
            grid.max_steps_to_go = count;
        }
        return total;
    }
    total += walk_grid(grid, row, col + 1, count - 1);
    if total == FINDABLE_FIELDS {
        if count > grid.max_steps_to_go {
            grid.max_steps_to_go = count;
        }
        return total;
    }
    total += walk_grid(grid, row, col - 1, count - 1);
    if total == FINDABLE_FIELDS {
        if count > grid.max_steps_to_go {
            grid.max_steps_to_go = count;
        }
    }
    total
}

fn parse_input(input: String) -> Grid {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut start: (i32, i32) = (0, 0);

    for (row, line) in input.lines().enumerate() {
        let mut grid_row: Vec<bool> = Vec::new();
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    grid_row.push(true);
                }
                '#' => grid_row.push(false),
                'S' => {
                    grid_row.push(true);
                    start = (row as i32, col as i32);
                }
                _ => (),
            }
        }
        grid.push(grid_row);
    }
    Grid::from(grid, start.0, start.1)
}

#[derive(Clone)]
#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<bool>>,
    hash_map: HashMap<(i32, i32), i32>,
    rows: i32,
    cols: i32,
    start_row: i32,
    start_col: i32,
    max_steps_to_go: i32,
}

impl Grid {
    pub fn from(grid: Vec<Vec<bool>>, start_row: i32, start_col: i32) -> Grid {
        let hash_map: HashMap<(i32, i32), i32> = HashMap::new();
        Grid {
            rows: grid.len() as i32,
            cols: grid.first().map_or(0, Vec::len) as i32,
            grid: grid,
            hash_map: hash_map,
            start_row: start_row,
            start_col: start_col,
            max_steps_to_go: 0,
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
struct Cell {
    row: i32,
    col: i32,
    cost: i32,
}

impl Ord for Cell {
    // Compare costs directly for max-heap behavior
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

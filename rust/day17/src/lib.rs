use std::fs;
use std::io;
use std::error::Error;
use std::collections::BinaryHeap;
use node::Node;
use node::Dir;

pub mod node;

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let grid = parse_input(input);
    let mut heap = BinaryHeap::new();
    let mut visited: Vec<u64> = Vec::new();

    let node = Node::new();

    let rows = grid.len() as i32;
    let cols = grid.first().map_or(0, Vec::len) as i32;
    let target_row = cols - 1;
    let target_col = rows - 1;
    let max_steps = 3;

    println!("Targets: row: {target_row}, col: {target_col}");

    heap.push(node);


    let mut max_combo: i32 = 0;

    loop {
        let cur = heap.pop().unwrap_or(Node::new());

        if visited.contains(&cur.min_id) {
            continue;
        }

        if cur.coord.row == target_row && cur.coord.col == target_col {
            println!("Result: {}", cur.cost);
        }

        let mut up_cost: u32 = cur.cost;
        let mut right_cost: u32 = cur.cost;
        let mut down_cost: u32 = cur.cost;
        let mut left_cost: u32 = cur.cost;

        for mv in 1..=4 {
            let up_coord = cur.coord.move_it_steps(Dir::Up, mv);
            let right_coord = cur.coord.move_it_steps(Dir::Right, mv);
            let down_coord = cur.coord.move_it_steps(Dir::Down, mv);
            let left_coord = cur.coord.move_it_steps(Dir::Left, mv);

            if !up_coord.is_out_of_bounds(rows, cols) 
                && cur.dir != Dir::Up 
                && cur.dir != Dir::Down {
                up_cost = up_coord.get_cost(&grid);
                heap.push(Node::new_with_hash(
                    up_cost,
                    up_coord,
                    Dir::Up,
                    1,
                ))
            }

            if !right_coord.is_out_of_bounds(rows, cols) && cur.dir != Dir::Right && cur.dir != Dir::Left {
                right_cost = up_coord.get_cost(&grid);
                heap.push(Node::new_with_hash(
                    right_cost,
                    right_coord,
                    Dir::Right,
                    1,
                ))
            }

            if !down_coord.is_out_of_bounds(rows, cols) && cur.dir != Dir::Up && cur.dir != Dir::Down {
                down_cost = up_coord.get_cost(&grid);
                heap.push(Node::new_with_hash(
                    down_cost,
                    down_coord,
                    Dir::Down,
                    1,
                ))
            }

            if !left_coord.is_out_of_bounds(rows, cols) && cur.dir != Dir::Right && cur.dir != Dir::Left {
                left_cost = up_coord.get_cost(&grid);
                heap.push(Node::new_with_hash(
                    left_cost,
                    left_coord,
                    Dir::Left,
                    1,
                ))
            }
        }
        break;
    }
    Ok(())
}

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let grid = parse_input(input);
    let mut heap = BinaryHeap::new();
    let mut visited: Vec<u64> = Vec::new();
    let node = Node::new();

    let rows = grid.len() as i32;
    let cols = grid.first().map_or(0, Vec::len) as i32;

    let target_row = cols - 1;
    let target_col = rows - 1;
    let max_steps = 3;

    println!("Targets: row: {target_row}, col: {target_col}");

    heap.push(node);


    let mut max_combo: i32 = 0;

    loop {
        let cur = heap.pop().unwrap_or(Node::new());

        if cur.coord.row + cur.coord.col > max_combo {
            max_combo = cur.coord.row + cur.coord.col;
            println!("Max combo: {max_combo}");
        }

        if cur.coord.is_out_of_bounds(rows, cols) || visited.contains(&cur.id) || visited.contains(&cur.min_id) {
            continue;
        }

        if cur.coord.row == target_row && cur.coord.col == target_col {
            println!("Result: {}", cur.cost);
            break;
        }

        visited.push(cur.id);

        let up_coord = cur.coord.move_it(Dir::Up);
        let right_coord = cur.coord.move_it(Dir::Right);
        let down_coord = cur.coord.move_it(Dir::Down);
        let left_coord = cur.coord.move_it(Dir::Left);

        let up_cost = up_coord.get_cost(&grid);
        let right_cost = right_coord.get_cost(&grid);
        let down_cost = down_coord.get_cost(&grid);
        let left_cost = left_coord.get_cost(&grid);

        if !up_coord.is_out_of_bounds(rows, cols) && cur.dir != Dir::Down {
            let mut num_steps = 1;
            if cur.dir == Dir::Up {
                num_steps = cur.num_steps + 1;
            }
            if num_steps <= max_steps {
                heap.push(Node::new_with_hash(
                    cur.cost + up_cost,
                    up_coord,
                    Dir::Up,
                    num_steps
                ))
            }
        }

        if !right_coord.is_out_of_bounds(rows, cols) && cur.dir != Dir::Left {
            let mut num_steps = 1;
            if cur.dir == Dir::Right {
                num_steps = cur.num_steps + 1;
            }
            if num_steps <= max_steps {
                heap.push(Node::new_with_hash(
                    cur.cost + right_cost,
                    right_coord,
                    Dir::Right,
                    num_steps
                ))
            }
        }
        
        if !down_coord.is_out_of_bounds(rows, cols) && cur.dir != Dir::Up {
            let mut num_steps = 1;
            if cur.dir == Dir::Down {
                num_steps = cur.num_steps + 1;
            }
            if num_steps <= max_steps {
                heap.push(Node::new_with_hash(
                    cur.cost + down_cost,
                    down_coord,
                    Dir::Down,
                    num_steps
                ))
            }
        }

        if !left_coord.is_out_of_bounds(rows, cols) && cur.dir != Dir::Right {
            let mut num_steps = 1;
            if cur.dir == Dir::Left {
                num_steps = cur.num_steps + 1;
            }
            if num_steps <= max_steps {
                heap.push(Node::new_with_hash(
                    cur.cost + left_cost,
                    left_coord,
                    Dir::Left,
                    num_steps
                ))
            }
        }
    }



    println!("Running...");
    Ok(())
}

fn parse_input(input: String) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| line.chars()
             .map(|ch| ch.to_digit(10).unwrap_or(0))
             .collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

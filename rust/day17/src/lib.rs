use std::fs;
use std::io;
use std::error::Error;
use std::collections::BinaryHeap;
use node::Node;
use node::Dir;

pub mod node;

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

    heap.push(node);

    loop {
        let cur = heap.pop().unwrap_or(Node::new());

        // println!("Visited: {:?}\n cur: {:?}", visited, cur);
        // let mut input = String::new();
        // io::stdin().read_line(&mut input)
        //     .expect("Failed to read line");

        if cur.coord.is_out_of_bounds(rows, cols) || visited.contains(&cur.id) {
            continue;
        }

        // println!("Cost: {}, Coord: {:?}, Preds: {:?}", cur.cost, cur.coord, cur.preds);
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

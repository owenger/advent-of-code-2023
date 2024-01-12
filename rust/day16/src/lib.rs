use std::fs;
use std::error::Error;
use std::cmp;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let char_vec = parse_input(input);
    let rows = char_vec.len();
    let cols = char_vec.first().map_or(0, Vec::len);
    let mut visited: Vec<Vec<(bool, bool)>> = vec![vec![(false, false); cols]; rows];

    walk_vec(&mut visited, &char_vec, &Coord{ row: 0, col: 0 }, &Direction::Horizontal(true));

    let total = count_visited(&visited);

    println!("Total: {total}");

    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let char_vec = parse_input(input);
    let rows = char_vec.len();
    let cols = char_vec.first().map_or(0, Vec::len);
    let visited: Vec<Vec<(bool, bool)>> = vec![vec![(false, false); cols]; rows];

    let mut max: usize = 0;
    for i in 0..rows {
        let char_vec_cl = char_vec.clone();
        let mut visited_cl = visited.clone();
        walk_vec(&mut visited_cl, &char_vec_cl, &Coord{ row: i as i32, col: 0 }, &Direction::Horizontal(true));
        let res = count_visited(&visited_cl);
        max = cmp::max(max, count_visited(&visited_cl));
        let char_vec_cl_1 = char_vec.clone();
        let mut visited_cl_1 = visited.clone();
        walk_vec(&mut visited_cl_1, &char_vec_cl_1, &Coord{ row: i as i32, col: (cols - 1) as i32}, &Direction::Horizontal(false));
        let res_1 = count_visited(&visited_cl);
        max = cmp::max(max, res_1);
    }
    for i in 0..cols {
        let char_vec_cl = char_vec.clone();
        let mut visited_cl = visited.clone();
        walk_vec(&mut visited_cl, &char_vec_cl, &Coord{ row: 0, col: i as i32 }, &Direction::Vertical(true));
        let res = count_visited(&visited_cl);
        max = cmp::max(max, res);
        let char_vec_cl_1 = char_vec.clone();
        let mut visited_cl_1 = visited.clone();
        walk_vec(&mut visited_cl_1, &char_vec_cl_1, &Coord{ row: (rows - 1) as i32, col: i as i32 }, &Direction::Vertical(false));
        let res_1 = count_visited(&visited_cl_1);
        max = cmp::max(max, res_1);
    }

    println!("Max: {max}");
    Ok(())
}

fn count_visited(visited: &Vec<Vec<(bool, bool)>>) -> usize {
    let mut total: usize = 0;
    for row in 0..visited.len() {
        for col in 0..visited.first().map_or(0, Vec::len) {
            if visited[row][col].0 || visited[row][col].1 {
                total += 1;
            }
        }
    }
    total
}

fn walk_vec(visited: &mut Vec<Vec<(bool, bool)>>, chars: &Vec<Vec<char>>, coord: &Coord, direction: &Direction) {
    let rows: i32 = visited.len() as i32;
    let cols: i32 = visited.first().map_or(0, Vec::len) as i32;

    if coord.row < 0 || coord.row >= rows || coord.col < 0 || coord.col >= cols {
        return;
    }

    let skip = chars[coord.row as usize][coord.col as usize] == '/' || chars[coord.row as usize][coord.col as usize] == '\\';

    let has_horizontal = visited[coord.row as usize][coord.col as usize].0;
    let has_vertical = visited[coord.row as usize][coord.col as usize].1;


    if !skip {
        if let Direction::Vertical(_) = direction {
            if visited[coord.row as usize][coord.col as usize].0 {
                return;
            } else {
                visited[coord.row as usize][coord.col as usize].0 = true;
            }
        } else {
            if visited[coord.row as usize][coord.col as usize].1 {
                return;
            } else {
                visited[coord.row as usize][coord.col as usize].1 = true;
            }
        }
    } else {
            visited[coord.row as usize][coord.col as usize].0 = true;
    }

    match chars[coord.row as usize][coord.col as usize] {
        '.' => {
            let (new_coord, new_dir) = dot_fn(&coord, &direction);
            walk_vec(visited, chars, &new_coord, &new_dir);
        }
        '/' => {
            let (new_coord, new_dir) = fslash_fn(&coord, &direction);
            walk_vec(visited, chars, &new_coord, &new_dir);
        }
        '\\' => {
            let (new_coord, new_dir) = bslash_fn(&coord, &direction);
            walk_vec(visited, chars, &new_coord, &new_dir);
        }
        '-' => {
            let res = dash_fn(&coord, &direction);
            for r in res {
                walk_vec(visited, chars, &r.0, &r.1);
            }
        }
        '|' => {
            let res = pipe_fn(&coord, &direction);
            for r in res {
                walk_vec(visited, chars, &r.0, &r.1);
            }
        }
        _ => (),
    }
}

fn dot_fn(coord: &Coord, direction: &Direction) -> (Coord, Direction) {
    match direction {
        Direction::Horizontal(forward) => {
            if *forward {
                return (Coord { row: coord.row, col: coord.col + 1 }, direction.clone());
            } else {
                return (Coord { row: coord.row, col: coord.col - 1 }, direction.clone());
            }
        }
        Direction::Vertical(forward)  => {
            if *forward {
                return (Coord { row: coord.row + 1, col: coord.col }, direction.clone());
            } else {
                return (Coord { row: coord.row - 1, col: coord.col }, direction.clone());
            }
        }
    }

}

fn fslash_fn(coord: &Coord, direction: &Direction) -> (Coord, Direction) {
    match direction {
        Direction::Horizontal(forward) => {
            if *forward {
                return (Coord { row: coord.row - 1, col: coord.col }, Direction::Vertical(false));
            } else {
                return (Coord { row: coord.row + 1, col: coord.col }, Direction::Vertical(true));
            }
        }
        Direction::Vertical(forward)  => {
            if *forward {
                return (Coord { row: coord.row, col: coord.col - 1 }, Direction::Horizontal(false));
            } else {
                return (Coord { row: coord.row, col: coord.col + 1 }, Direction::Horizontal(true));
            }
        }
    }
}

fn bslash_fn(coord: &Coord, direction: &Direction) -> (Coord, Direction) {
    match direction {
        Direction::Horizontal(forward) => {
            if *forward {
                return (Coord { row: coord.row + 1, col: coord.col }, Direction::Vertical(true));
            } else {
                return (Coord { row: coord.row - 1, col: coord.col }, Direction::Vertical(false));
            }
        }
        Direction::Vertical(forward)  => {
            if *forward {
                return (Coord { row: coord.row, col: coord.col + 1 }, Direction::Horizontal(true));
            } else {
                return (Coord { row: coord.row, col: coord.col - 1 }, Direction::Horizontal(false));
            }
        }
    }
}

fn dash_fn(coord: &Coord, direction: &Direction) -> Vec<(Coord, Direction)> {
    let mut output: Vec<(Coord, Direction)> = Vec::new();
    match direction {
        Direction::Horizontal(forward) => {
            if *forward {
                output.push((Coord{ row: coord.row, col: coord.col + 1 }, direction.clone()));
                return output;
            } else {
                output.push((Coord{ row: coord.row, col: coord.col - 1 }, direction.clone()));
                return output;
            }
        }
        Direction::Vertical(forward) => {
            output.push((Coord{ row: coord.row, col: coord.col + 1 }, Direction::Horizontal(true)));
            output.push((Coord{ row: coord.row, col: coord.col - 1 }, Direction::Horizontal(false)));
            return output;
        }
    }
}

fn pipe_fn(coord: &Coord, direction: &Direction) -> Vec<(Coord, Direction)> {
    let mut output: Vec<(Coord, Direction)> = Vec::new();
    match direction {
        Direction::Vertical(forward) => {
            if *forward {
                output.push((Coord{ row: coord.row + 1, col: coord.col }, direction.clone()));
                return output;
            } else {
                output.push((Coord{ row: coord.row - 1, col: coord.col }, direction.clone()));
                return output;
            }
        }
        Direction::Horizontal(forward) => {
            output.push((Coord{ row: coord.row + 1, col: coord.col }, Direction::Vertical(true)));
            output.push((Coord{ row: coord.row - 1, col: coord.col }, Direction::Vertical(false)));
            return output;
        }
    }
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars()
             .collect())
        .collect()
}

#[derive(Clone)]
struct Coord {
    row: i32,
    col: i32, 
}

#[derive(Debug)]
#[derive(Clone)]
enum Direction {
    Horizontal(bool),
    Vertical(bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

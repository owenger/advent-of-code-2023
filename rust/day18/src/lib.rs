use std::fs;
use std::error::Error;
use regex::Regex;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let commands = parse_input(input);
    let dimensions = get_dimensions(&commands);
    let mut grid: Vec<Vec<bool>> = vec![vec![false; dimensions.cols as usize]; dimensions.rows as usize];
    walk_grid(&mut grid, &commands, &dimensions);
    count_non_lava(&grid, &dimensions);

    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let mut commands = parse_input(input);
    commands = convert_commands(commands);
    let res = draw_around(&commands);
    println!("Res: {res}");

    Ok(())
}

fn draw_around(commands: &Vec<Command>) -> i64 {
    let mut total: i64 = 0;
    let mut row: i64 = 0;
    let mut col: i64 = 0;

    let mut last_coords: (i64, i64) = (0, 0);

    for i in 0..commands.len() {
        let mut cur_coords: (i64, i64) = (0, 0);
        if i != commands.len() - 1 {
            match commands[i].dir {
                'R' => {
                    col += commands[i].num;
                    if commands[i + 1].dir == 'U' {
                        row += 1;
                        cur_coords = (row - 1, col);
                    } else if commands[i + 1].dir == 'D' {
                        col += 1;
                        cur_coords = (row, col);
                    } else {
                        println!("Cur: {}, next: {}", commands[i].dir, commands[i + 1].dir);
                    }
                }
                'D' => {
                    row += commands[i].num;
                    if commands[i + 1].dir == 'R' {
                        col -= 1;
                        cur_coords = (row, col + 1);
                    } else if commands[i + 1].dir == 'L' {
                        row += 1;
                        cur_coords = (row, col);
                    } else {
                        println!("Cur: {}, next: {}", commands[i].dir, commands[i + 1].dir);
                    }
                }
                'L' => {
                    col -= commands[i].num;
                    if commands[i + 1].dir == 'D' {
                        row -= 1;
                        cur_coords = (row + 1, col);
                    } else if commands[i + 1].dir == 'U' {
                        col -= 1;
                        cur_coords = (row, col);
                    } else {
                        println!("Cur: {}, next: {}", commands[i].dir, commands[i + 1].dir);
                    }
                }
                'U' => {
                    row -= commands[i].num;
                    if commands[i + 1].dir == 'L' {
                        col += 1;
                        cur_coords = (row, col - 1);
                    } else if commands[i + 1].dir == 'R' {
                        row -= 1;
                        cur_coords = (row, col);
                    } else {
                        println!("Cur: {}, next: {}", commands[i].dir, commands[i + 1].dir);
                    }
                }
                _ => (),
            }
        }
        total += cur_coords.0 * last_coords.1 - last_coords.0 * cur_coords.1;
        last_coords.0 = cur_coords.0;
        last_coords.1 = cur_coords.1;
    }
    total.abs() / 2
}

fn calculate_area(commands: &Vec<Command>) -> i64 {
    let mut cur_row: i64 = 0;
    let mut cur_col: i64 = 0;
    let mut next_row: i64 = 0;
    let mut next_col: i64 = 0;

    let mut total: i64 = 0;

    for command in commands {
        match command.dir {
            'U' => next_row -= command.num,
            'R' => next_col += command.num,
            'D' => next_row += command.num,
            'L' => next_col -= command.num,
            _ => println!("Nope"),
        }
        println!("xdiff: {}, ydiff: {}", next_row - cur_row, next_col - cur_col);
        total += cur_row * next_col - cur_col * next_row;
        cur_row = next_row;
        cur_col = next_col;
    }
    total = total.abs() / 2;
    println!("Total: {total}");
    total
}

fn count_non_lava(reference_grid: &Vec<Vec<bool>>, dims: &Dimensions) -> u64 {
    let mut grid = reference_grid.clone();
    let mut count: u64 = 0;

    for row in 0..dims.rows {
        count += seed_non_lava(&mut grid, row, 0);
        count += seed_non_lava(&mut grid, row, dims.cols - 1);
    }
    for col in 1..dims.cols-1 {
        count += seed_non_lava(&mut grid, 0, col);
        count += seed_non_lava(&mut grid, dims.rows - 1, col);
    }

    let real_count = dims.rows * dims.cols - count as i64;

    println!("Count: {real_count}");
    //print_bool_matrix(&grid);

    count
}

fn seed_non_lava(grid: &mut Vec<Vec<bool>>, row: i64, col: i64) -> u64 {
    if row < 0 || row >= grid.len() as i64 || col < 0 || col >= grid.first().map_or(0, Vec::len) as i64 {
        return 0;
    }
    if grid[row as usize][col as usize] == true {
        return 0;
    }
    let mut count: u64 = 1;
    grid[row as usize][col as usize] = true;
    count += seed_non_lava(grid, row + 1, col);
    count += seed_non_lava(grid, row - 1, col);
    count += seed_non_lava(grid, row, col + 1);
    count += seed_non_lava(grid, row, col - 1);
    count
}

fn walk_grid(grid: &mut Vec<Vec<bool>>, commands: &Vec<Command>, dims: &Dimensions) {
    grid[dims.start_row as usize][dims.start_col as usize] = true;

    let mut cur_row: usize = dims.start_row as usize;
    let mut cur_col: usize = dims.start_col as usize;

    for command in commands {
        match command.dir {
            'U' => {
                for _ in 0..command.num {
                    cur_row -= 1;
                    grid[cur_row][cur_col] = true;
                }
            }
            'R' => {
                for _ in 0..command.num {
                    cur_col += 1;
                    grid[cur_row][cur_col] = true;
                }
            }
            'D' => {
                for _ in 0..command.num {
                    cur_row += 1;
                    grid[cur_row][cur_col] = true;
                }
            }
            'L' => {
                for _ in 0..command.num {
                    cur_col -= 1;
                    grid[cur_row][cur_col] = true;
                }
            }
            _ => (),

        }
    }

}

fn get_dimensions(commands: &Vec<Command>) -> Dimensions {
    let mut cur_row: i64 = 0;
    let mut max_row: i64 = 0;
    let mut min_row: i64 = 0;
    let mut cur_col: i64 = 0;
    let mut max_col: i64 = 0;
    let mut min_col: i64 = 0;

    for i in 0..commands.len() {
        match commands[i].dir {
            'U' => {
                cur_row -= commands[i].num;
                if cur_row < min_row {
                    min_row = cur_row;
                }
            }
            'R' => {
                cur_col += commands[i].num;
                if cur_col > max_col {
                    max_col = cur_col;
                }
            }
            'D' => {
                cur_row += commands[i].num;
                if cur_row > max_row {
                    max_row = cur_row;
                }
            }
            'L' => {
                cur_col -= commands[i].num;
                if cur_col < min_col {
                    min_col = cur_col;
                }
            }
            _ => (),
        }
    }

    Dimensions{
        rows: max_row - min_row + 1,
        cols: max_col - min_col + 1,
        start_row: -1 * min_row,
        start_col: -1 * min_col,
    }

}

fn convert_commands(prev_commands: Vec<Command>) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    
    for command in prev_commands {
        let mut new_dir: char = 'U';
        match command.col.chars().last().unwrap() {
            '0' => new_dir = 'R',
            '1' => new_dir = 'D',
            '2' => new_dir = 'L',
            '3' => new_dir = 'U',
            _ => (),
        }
        let first_five: String = command.col.chars().take(5).collect();
        let new_num = i64::from_str_radix(&first_five, 16).unwrap();
        commands.push(Command{
            dir: new_dir,
            num: new_num,
            col: command.col,
        });
    }
    commands
}

fn parse_input(input: String) -> Vec<Command> {
    let re = Regex::new(r"(\w)\s+(\d+)\s+\(#(\w+)\)").unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let mut output_vec: Vec<Command> = Vec::new();

    for line in lines {
        match re.captures(line) {
            Some(caps) => {
                let dir_str = caps.get(1).unwrap().as_str();
                let num_str = caps.get(2).unwrap().as_str();
                let col_str = caps.get(3).unwrap().as_str();

                let dir: char = dir_str.chars().next().unwrap();
                let num: i64 = num_str.parse().unwrap_or(0);
                let col = col_str.to_string();

                output_vec.push(Command{ dir: dir, num: num, col: col });
            }
            None => println!("Nothing captured"),
        }
    }

    output_vec
}

fn print_bool_matrix(matrix: &Vec<Vec<bool>>) {
    for row in matrix {
        for &value in row {
            let char_to_print = if value { '#' } else { '.' };
            print!("{} ", char_to_print);
        }
        println!(); // New line at the end of each row
    }
}

#[derive(Debug)]
struct Command {
    pub dir: char,
    pub num: i64,
    pub col: String,
}

#[derive(Debug)]
struct Dimensions {
    pub rows: i64,
    pub cols: i64,
    pub start_row: i64,
    pub start_col: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

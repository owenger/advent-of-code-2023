use std::fs;
use std::error::Error;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let split_input: Vec<&str> = input.split(',')
                                        .map(|s| s.trim_matches('\n'))
                                        .collect();
    let mut result: u32 = 0;
    for split in split_input {
        result += get_string_hash(&split);
    }
    println!("Result: {result}");
    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let split_input: Vec<&str> = input.split(',')
                                        .map(|s| s.trim_matches('\n'))
                                        .collect();

    let commands = parse_input(split_input);
    let boxes = execute_commands(&commands);
    let res = get_res(&boxes);
    println!("Res: {res}");


    //follow_command(&split_input);

    Ok(())
}

fn get_res(boxes: &Vec<Vec<(String, u32)>>) -> u32 {
    let mut res: u32 = 0;

    for i in 0..boxes.len() {
        for j in 0..boxes[i as usize].len() {
            let sub_res = (i + 1) as u32 * (j + 1) as u32 * boxes[i as usize][j as usize].1;
            res += sub_res;
        }
    }
    res
}

fn execute_commands(commands: &Vec<Command>) -> Vec<Vec<(String, u32)>> {
    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];

    for command in commands {
        if command.cmd_type {
            insert_lens(&mut boxes[get_string_hash(&command.label) as usize], &command);
        } else {
            remove_lens(&mut boxes[get_string_hash(&command.label) as usize], &command);
        }
    }
    boxes
}

fn remove_lens(bx: &mut Vec<(String, u32)>, command: &Command) {
    for i in 0..bx.len() {
        if bx[i].0 == command.label {
            bx.remove(i);
            return;
        }
    }
}

fn insert_lens(bx: &mut Vec<(String, u32)>, command: &Command) {
    let mut exists = false;

    for i in 0..bx.len() {
        if bx[i].0 == command.label {
            bx[i].1 = command.number;
            exists = true;
        }
    }
    if !exists {
        bx.push((command.label.clone(), command.number));
    }
}

fn parse_input(input: Vec<&str>) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    for com in input {
        let mut label: String = String::new();
        let mut number: u32 = 0;
        let mut cmd_type: bool = false;
        for ch in com.chars() {
            if ch.is_digit(10) {
                number = ch.to_digit(10).unwrap_or(0);
            } else if ch == '=' {
                cmd_type = true;
            } else if ch == '-' {
                cmd_type = false;
            } else {
                label.push(ch);
            }
        }
        commands.push(Command {
            hash: get_string_hash(&label),
            cmd_type: cmd_type,
            label: label,
            number: number,
        })
    }
    commands
} 

fn get_string_hash(input: &str) -> u32 {
    let mut cur_value: u32 = 0;
    for ch in input.chars() {
        cur_value += ch as u32;
        cur_value *= 17;
        cur_value = cur_value % 256;
    }
    cur_value
}

#[derive(Debug)]
struct Command {
    hash: u32,
    cmd_type: bool,
    label: String,
    number: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

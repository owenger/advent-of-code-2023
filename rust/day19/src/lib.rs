use std::fs;
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    parse_input(input);
    Ok(())
}

fn parse_input(input: String) -> HashMap<String, Vec<Instruction>> {
    let mut instructions: HashMap<String, Vec<Instruction>> = HashMap::new();

    let first_re = Regex::new(r"(\w+{\w+})");

    for line in input.lines() {
        if let Some(caps) = first_re.captures(line) {
            let letter_str = caps.get(1).unwrap().as_str();
            let number_str = caps.get(2).unwrap().as_str();
        }
    }

    instructions 
}

struct Part {
    pub x: i32,
    pub m: i32,
    pub a: i32,
    pub s: i32,
}

struct Instruction {
    compare_gt: bool,
    number: i32,
    res: Res,
}

enum Res {
    Accepted,
    Rejected,
    Forwarded(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

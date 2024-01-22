use std::fs;
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let (parts, instructions) = parse_input(input);
    let res = walk_parts(parts, instructions);

    Ok(())
}

fn walk_parts(parts: Vec<Part>, hash_map: HashMap<String, Vec<Instruction>>) -> i32 {
    let mut total: i32 = 0;
    let cur_key = String::from("in");

    for part in parts {
        if let Some(instructions) = hash_map.get(&cur_key) {

        } else {
            println!("Fail");
        }
    }
    total
}

fn parse_input(input: String) -> (Vec<Part>, HashMap<String, Vec<Instruction>>) {
    let mut instructions: HashMap<String, Vec<Instruction>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    let first_re = Regex::new(r"(\w+)\{([^}]+)\}").unwrap();
    let second_re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    for line in input.lines() {
        if let Some(caps) = first_re.captures(line) {
            let key = caps.get(1).unwrap().as_str().to_string();
            let instruction_string = caps.get(2).unwrap().as_str();
            //println!("Key: {key}, InstrStr: {instruction_string}");
            instructions.insert(key, interpret_construction_str(instruction_string));
        }
        if let Some(caps) = second_re.captures(line) {
            let x: i32 = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
            let m: i32 = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
            let a: i32 = caps.get(3).unwrap().as_str().parse().unwrap_or(0);
            let s: i32 = caps.get(4).unwrap().as_str().parse().unwrap_or(0);
            parts.push(Part { x: x, m: m, a: a, s: s });
        }
    }

    (parts, instructions) 
}

fn interpret_construction_str(input: &str) -> Vec<Instruction> {
    let mut output: Vec<Instruction> = Vec::new();
    let re = Regex::new(r"(\w)([<>])(\d+):(\w+)").unwrap();
    for instr in input.split(',') {
        let mut instruction = Instruction::new();
        if !instr.contains(':') {
            instruction.compare_gt = true;
            instruction.number = 0;
            match instr {
                "A" => instruction.res = Res::Accepted,
                "R" => instruction.res = Res::Rejected,
                _ => instruction.res = Res::Forwarded(instr.to_string()),
            };
        } else {
            if let Some(caps) = re.captures(instr) {
                instruction.part = caps.get(1).unwrap().as_str().chars().next().unwrap();
                match caps.get(2).unwrap().as_str().chars().next().unwrap() {
                    '<' => instruction.compare_gt = false,
                    _ => instruction.compare_gt = true,
                }
                instruction.number = caps.get(3).unwrap().as_str().parse().unwrap_or(0);
                let res_str = caps.get(4).unwrap().as_str();
                match  res_str {
                    "A" => instruction.res = Res::Accepted,
                    "R" => instruction.res = Res::Rejected,
                    _ => instruction.res = Res::Forwarded(res_str.to_string()),
                }
            }
        }
        output.push(instruction);
    }
    output

}

struct Part {
    pub x: i32,
    pub m: i32,
    pub a: i32,
    pub s: i32,
}

impl Part {
    pub fn new() -> Part {
        Part{ x: 0, m: 0, a: 0, s: 0 }
    }
}

#[derive(Debug)]
struct Instruction {
    part: char,
    compare_gt: bool,
    number: i32,
    res: Res,
}

impl Instruction {
    pub fn new() -> Instruction {
        Instruction{ part: 'x', compare_gt: false, number: 0, res: Res::Rejected }
    }

    pub fn check(&self, )
}

#[derive(Debug)]
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

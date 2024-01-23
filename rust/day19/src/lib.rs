use std::fs;
use std::cmp;
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let (parts, instructions) = parse_input(input, true);
    let res = walk_parts(parts, instructions);

    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    println!("Before");
    let input = fs::read_to_string(input_path)?;
    let (_, instructions) = parse_input(input, false);
    println!("After");
    let res = do_part_ranges(&instructions, &RangePart::new(), &Res::Forwarded(String::from("in")));
    println!("Result: {res}");

    Ok(())
}

fn do_part_ranges(
    hash_map: &HashMap<String, Vec<Instruction>>, 
    range_part: &RangePart, 
    res: &Res
    ) -> i64 {
    //println!("Checking range: {:?} on res: {:?}", range_part, res);
    let mut key: String = String::new();
    match res {
        Res::Accepted => {
            return (range_part.x.1 - range_part.x.0 + 1) 
            * (range_part.m.1 - range_part.m.0 + 1)
            * (range_part.a.1 - range_part.a.0 + 1) 
            * (range_part.s.1 - range_part.s.0 + 1);
        }
        Res::Rejected => return 0,
        Res::Forwarded(to) => key = to.clone(),
        Res::Next => (),
    }

    let mut total: i64 = 0;
    let mut gets_res = range_part.clone();
    let mut continues = range_part.clone();
    if let Some(instructions) = hash_map.get(&key) {
        for i in 0..instructions.len() {
            (gets_res, continues) = instructions[i].check_range(continues);
            total += do_part_ranges(hash_map, &gets_res, &instructions[i].res)
        }
    }
    total
}

fn walk_parts(parts: Vec<Part>, hash_map: HashMap<String, Vec<Instruction>>) -> i64 {
    let mut total: i64 = 0;
    for part in parts {
        total += do_part(&part, &hash_map);
    }
    println!("Total: {total}");
    0
}

fn do_part(part: &Part, hash_map: &HashMap<String, Vec<Instruction>>) -> i64 {
    let mut cur_key = String::from("in");
    'outer: loop {
        if let Some(instructions) = hash_map.get(&cur_key) {
            for i in 0..instructions.len() {
                match instructions[i].check(part) {
                    Res::Next => continue,
                    Res::Accepted => return part.x + part.m + part.a + part.s,
                    Res::Rejected => return 0,
                    Res::Forwarded(place) => {
                        cur_key = place;
                        continue 'outer;
                    }
                }
            }
        } else {
            println!("Fail");
        }
    }
}

fn parse_input(input: String, get_parts: bool) -> (Vec<Part>, HashMap<String, Vec<Instruction>>) {
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
        if get_parts {
            if let Some(caps) = second_re.captures(line) {
                let x: i64 = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
                let m: i64 = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
                let a: i64 = caps.get(3).unwrap().as_str().parse().unwrap_or(0);
                let s: i64 = caps.get(4).unwrap().as_str().parse().unwrap_or(0);
                parts.push(Part { x: x, m: m, a: a, s: s });
        }
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

#[derive(Debug)]
struct Part {
    pub x: i64,
    pub m: i64,
    pub a: i64,
    pub s: i64,
}

impl Part {
    pub fn new() -> Part {
        Part{ x: 0, m: 0, a: 0, s: 0 }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct RangePart {
    pub x: (i64, i64),
    pub m: (i64, i64),
    pub a: (i64, i64),
    pub s: (i64, i64),
}

impl RangePart {
    pub fn new() -> RangePart {
        RangePart{ x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000)}
    }
}

#[derive(Debug)]
struct Instruction {
    part: char,
    compare_gt: bool,
    number: i64,
    res: Res,
}

impl Instruction {
    pub fn new() -> Instruction {
        Instruction{ part: 'x', compare_gt: false, number: 0, res: Res::Rejected }
    }

    pub fn check(&self, part: &Part) -> Res {
        let mut number: i64 = 0;
        match self.part {
            'x' => number = part.x,
            'm' => number = part.m,
            'a' => number = part.a,
            's' => number = part.s,
            _ => (),
        }
        if self.compare_gt {
            if number > self.number {
                return self.res.clone();
            }
        } else {
            if number < self.number {
                return self.res.clone();
            }
        }
        Res::Next
    }

    pub fn check_range(&self, range_part: RangePart) -> (RangePart, RangePart) {
        let mut gets_res = range_part.clone();
        let mut continues = range_part.clone();

        let mut gets_res_range: &mut (i64, i64) = &mut gets_res.x;
        let mut continues_range: &mut (i64, i64) = &mut continues.x;
        match self.part {
            'x' => (),
            'm' => {
                gets_res_range = &mut gets_res.m;
                continues_range = &mut continues.m;
            }
            'a' => {
                gets_res_range = &mut gets_res.a;
                continues_range = &mut continues.a;
            }
            's' => {
                gets_res_range = &mut gets_res.s;
                continues_range = &mut continues.s;
            }
            _ => (),
        }
        if self.compare_gt {
            gets_res_range.0 = cmp::max(gets_res_range.0, self.number + 1);
            continues_range.1 = cmp::min(continues_range.1, self.number);
        } else {
            gets_res_range.1 = cmp::min(gets_res_range.1, self.number - 1);
            continues_range.0 = cmp::max(continues_range.0, self.number);
        }

        (gets_res, continues)
    }
}

#[derive(Clone)]
#[derive(Debug)]
enum Res {
    Accepted,
    Rejected,
    Next,
    Forwarded(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_check() {
        let instr = Instruction{ part: 'x', compare_gt: false, number: 50, res: Res::Accepted };
        let range_part = RangePart::new();
        let (res, cont) = instr.check_range(range_part);
        assert_eq!(res.x.0, 1);
        assert_eq!(res.x.1, 49);
        assert_eq!(cont.x.0, 50);
        assert_eq!(cont.x.1, 4000);
    }

    #[test]
    fn test_range_check1() {
        let instr = Instruction{ part: 'x', compare_gt: false, number: 50, res: Res::Accepted };
        let range_part = RangePart::new();
        let (mut res, mut cont) = instr.check_range(range_part);
        println!("Res: {:?}, Cont: {:?}", res.x, cont.x);
        let new_instr = Instruction{ part: 'x', compare_gt: true, number: 400, res: Res::Accepted };
        (res, cont) = new_instr.check_range(cont);
        println!("Res: {:?}, Cont: {:?}", res.x, cont.x);
        assert_eq!(cont.x.0, 50);
        assert_eq!(cont.x.1, 400);
        assert_eq!(res.x.0, 401);
        assert_eq!(res.x.1, 4000);
    }

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

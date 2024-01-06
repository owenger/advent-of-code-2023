use std::error::Error;
use std::fs;
use regex::Regex;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let mut total: usize = 0;
    for line in input.lines() {
        let (mut entries, instructions) = parse_line(line);
        let mut unknowns: Vec<usize> = Vec::new();
        for i in 0..entries.len() {
            if entries[i] == Entry::Unknown {
                unknowns.push(i);
            }
        }
        let nr_possibilities = run_line(&mut entries, &instructions, &unknowns, 0);
        total += nr_possibilities;
        println!("Line Result: {nr_possibilities}");
    }
    println!("Total result: {total}");

    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let mut total: usize = 0;
    for line in input.lines() {
        let (mut entries, instructions) = parse_line(line);
        let (mut entries, instructions) = unfold_line(entries, instructions);
        println!("Unfolded: {:?}", entries);
        let mut unknowns: Vec<usize> = Vec::new();
        for i in 0..entries.len() {
            if entries[i] == Entry::Unknown {
                unknowns.push(i);
            }
        }
        let nr_possibilities = run_line(&mut entries, &instructions, &unknowns, 0);
        total += nr_possibilities;
        println!("Line Result: {nr_possibilities}");
    }
    println!("Total result: {total}");

    Ok(())
}


fn unfold_line(entries: Vec<Entry>, instructions: Vec<usize>) -> (Vec<Entry>, Vec<usize>) {
    let mut new_ent: Vec<Entry> = Vec::new();
    let mut new_inst: Vec<usize> = Vec::new();

    for i in 0..5 {
        for item in &entries {
            new_ent.push(item.clone());
        }
        for item in &instructions {
            new_inst.push(*item);
        }
        if i != 4 {
            new_ent.push(Entry::Unknown);
        } 
    }
    return (new_ent, new_inst);
}

fn run_line(entries: &mut Vec<Entry>, instructions: &[usize], unknowns: &[usize], i: usize) -> usize {
    if i == unknowns.len() {
        let is_v = is_val(&entries, &instructions);
        //println!("Checking {:?}, {is_v}", entries);
        if is_v {
            return 1;
        } else {
            return 0;
        }
    }
    let mut total: usize = 0;
    entries[unknowns[i]] = Entry::Broken;
    total += run_line(entries, &instructions, unknowns, i+1);
    entries[unknowns[i]] = Entry::Working;
    total += run_line(entries, &instructions, unknowns, i+1);

    total
}

fn is_val(entries: &[Entry], instructions: &[usize]) -> bool {
    let mut vec: Vec<usize> = Vec::new();
    let mut count: usize = 0;
    
    for i in 0..entries.len() {
        if entries[i] == Entry::Broken {
            count += 1;
        } else if entries[i] == Entry::Working && count > 0 {
            vec.push(count);
            count = 0;
        }
    }
    if count > 0 {
        vec.push(count);
    }

    if instructions.len() != vec.len() {
        return false;
    }

    for i in 0..instructions.len() {
        if instructions[i] != vec[i] {
            return false;
        }
    }
    true
}

fn get_instruction_size(instructions: &[usize]) -> usize {
    let mut size = instructions.len() - 1;

    for instr in instructions {
        size += instr;
    }

    size
}

fn parse_line(line: &str) -> (Vec<Entry>, Vec<usize>) {
    let mut entries: Vec<Entry> = Vec::new();
    let mut instructions: Vec<usize> = Vec::new();

    // Regex to match the spring states and group sizes separately
    let re = Regex::new(r"([.#?]+)|(\d+)").unwrap();

    for cap in re.captures_iter(line) {
        if let Some(matched) = cap.get(1) {
            // This is a string of spring states
            for ch in matched.as_str().chars() {
                match ch {
                    '.' => entries.push(Entry::Working),
                    '#' => entries.push(Entry::Broken),
                    '?' => entries.push(Entry::Unknown),
                    _ => (),
                }
            }
        } else if let Some(matched) = cap.get(2) {
            // This is a number representing a group size
            if let Ok(number) = matched.as_str().parse::<usize>() {
                instructions.push(number);
            }
        }
    }

    (entries, instructions)
}

#[derive(Debug, Clone)]
enum Entry {
    Broken,
    Working,
    Unknown,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Entry::Working, Entry::Working) => true,
            (Entry::Broken, Entry::Broken) => true,
            (Entry::Unknown, Entry::Unknown) => true,
            // Add additional comparisons for other variants if necessary
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }

    #[test]
    fn is_valid_test_0() {
        let entry_vec = vec![Entry::Broken, Entry::Broken, Entry::Working, Entry::Working, Entry::Broken];
        let instructions = vec![2, 1];

        assert!(is_val(&entry_vec, &instructions));
    }

    #[test]
    fn is_valid_test_1() {
        let entry_vec = vec![Entry::Broken, Entry::Broken, Entry::Working, Entry::Working, Entry::Broken, Entry::Working];
        let instructions = vec![2, 1];

        assert!(is_val(&entry_vec, &instructions));
    }

    #[test]
    fn is_valid_test_2() {
        let entry_vec = vec![Entry::Broken, Entry::Broken, Entry::Working, Entry::Working, Entry::Broken, Entry::Working, Entry::Broken];
        let instructions = vec![2, 1];

        assert!(!is_val(&entry_vec, &instructions));
    }

    #[test]
    fn is_valid_test_3() {
        let entry_vec = vec![Entry::Broken, Entry::Broken, Entry::Working, Entry::Working, Entry::Broken, Entry::Broken];
        let instructions = vec![2, 1];

        assert!(!is_val(&entry_vec, &instructions));
    }

    #[test]
    fn is_valid_test_4() {
        let entry_vec = vec![Entry::Broken, Entry::Working, Entry::Working, Entry::Broken];
        let instructions = vec![1];

        assert!(!is_val(&entry_vec, &instructions));
    }
}

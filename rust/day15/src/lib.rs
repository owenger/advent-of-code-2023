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

pub fn get_string_hash(input: &str) -> u32 {
    let mut cur_value: u32 = 0;
    for ch in input.chars() {
        cur_value += ch as u32;
        cur_value *= 17;
        cur_value = cur_value % 256;
    }
    cur_value

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

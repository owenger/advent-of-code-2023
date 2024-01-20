use std::fs;
use std::error::Error;
use std::collections::HashMap;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let grid = parse_input(input);

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

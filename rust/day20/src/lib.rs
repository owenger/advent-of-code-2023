use std::fs;
use std::error::Error;
use std::collections::HashMap;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    Ok(())
}

struct FlipFlop {
    state: bool,
}

impl Pulsar for FlipFlop {
    fn receive_pulse(&self, pulse: bool) -> Option<bool> {
        if !pulse {
            self.state = !self.state;
            return Some(true);
        }
        None
    }
}

trait Pulsar {
    pub fn receive_pulse(&self, pulse: bool) -> Option<bool>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

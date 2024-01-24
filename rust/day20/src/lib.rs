use std::fs;
use std::error::Error;
use std::collections::HashMap;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    Ok(())
}

struct Pulse {
    origin: String,
    destination: String,
    is_high: bool,
}

struct FlipFlop {
    name: String,
    state: bool,
    destinations: Vec<String>,
}

struct Conj {
    name: String,
    inputs: HashMap<String, bool>,
    destinations: Vec<String>,
}

impl Pulsar for FlipFlop {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let mut pulse_out: Vec<Pulse> = Vec::new();
        if !pulse.is_high {
            self.state = !self.state;
            for i in 0..self.destinations.len() {
                pulse_out.push(
                    Pulse{
                        origin: self.name.clone(),
                        destination: self.destinations[i].clone(),
                        is_high: true
                    })
            }
        }
        pulse_out
    }
}

impl Pulsar for Conj {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let mut pulse_out: Vec<Pulse> = Vec::new();
        self.inputs.insert(pulse.origin, pulse.is_high);
        let all_highs = !self.inputs.any(|&value| value == false);
    }
}

pub trait Pulsar {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

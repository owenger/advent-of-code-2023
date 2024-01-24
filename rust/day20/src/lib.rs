use std::fs;
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    parse_input(input);
    Ok(())
}

fn parse_input(mut input: String) {
    let mut hash: HashMap<String, Box<dyn Pulsar>> = HashMap::new();
    input = input.chars().filter(|&c| c != ' ').collect();

    let re = Regex::new(r"([%&])(\w+)->([\w,]+)").unwrap();

    for line in input.lines() {
        println!("{line}");
        match re.captures(line) {
            Some(caps) => {
                let first = caps.get(1).unwrap().as_str();
                let second = caps.get(2).unwrap().as_str();
                let third = caps.get(3).unwrap().as_str();

                if first.chars().next().unwrap() == '%' {
                    hash.insert(second.to_string(), Box::new(FlipFlop{
                        name: second.to_string(),
                        state: false,
                        destinations: third.split(',').map(|c| c.to_string()).collect(),
                    }));
                }


            }
            None => println!("Nothing, loser"),
        }
    }
}

struct Pulse {
    origin: String,
    destination: String,
    is_high: bool,
}

struct BroadCaster {
    name: String,
    destinations: Vec<String>,
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

impl Pulsar for BroadCaster {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let mut pulse_out: Vec<Pulse> = Vec::new();

        for i in 0..self.destinations.len() {
            pulse_out.push(Pulse{
                origin: self.name.clone(),
                destination: self.destinations[i].clone(),
                is_high: false,
            })
        }
        pulse_out
    }
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
        let all_highs = !self.inputs.values().any(|&value| value == false);
        for i in 0..self.destinations.len() {
            pulse_out.push(Pulse{
                origin: self.name.clone(),
                destination: self.destinations[i].clone(),
                is_high: !all_highs,
            })
        }
        pulse_out
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

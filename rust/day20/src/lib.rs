use std::fs;
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;
use std::fmt::Debug;
use std::collections::VecDeque;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let mut hash = parse_input(input);
    push_button(&mut hash, false);

    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let mut hash = parse_input(input);
    push_button(&mut hash, true);

    Ok(())
}

fn push_button(hash: &mut HashMap<String, Box<dyn Pulsar>>, part2: bool) {
    let mut pulses: Vec<(i64, i64)> = Vec::new();
    let mut count = 0;
    'outer: loop {
        let mut count_low = 0;
        let mut count_high = 0;
        let mut to_process: VecDeque<Pulse> = VecDeque::new();
        to_process.push_back(Pulse{
            origin: String::from(""),
            destination: String::from("broadcaster"),
            is_high: false,
        });

        while let Some(cur_pulse) = to_process.pop_front() {
            if cur_pulse.origin == String::from("bq") && cur_pulse.destination == String::from("ft"){
                if !cur_pulse.is_high {
                    println!("Part 2 Res: {}", count + 1);
                    //break 'outer;
                }
            }
            if cur_pulse.is_high {
                count_high += 1;
            } else {
                count_low += 1;
            }
            if let Some(node) = hash.get_mut(&cur_pulse.destination) {
                let cur_vec = node.receive_pulse(cur_pulse);
                for ps in cur_vec {
                    to_process.push_back(ps);
                }
            }
        }
        count += 1;
        if count % 100000 == 0 {
            println!("Count: {}", count);
        }
        pulses.push((count_low, count_high));
        if check_if_default(&hash) || count == 1000 && !part2{
            break;
        }
    }
    println!("Res: {}", count_pulses(&pulses));
}

fn count_pulses(pulses: &Vec<(i64, i64)>) -> i64 {
    let mut low = 0;
    let mut high = 0;

    for pulse in pulses {
        low += pulse.0;
        high += pulse.1;
    }
    low * high
}

fn check_if_default(hash: &HashMap<String, Box<dyn Pulsar>>) -> bool {
    !hash.values().any(|value| value.is_default() == false)
}

fn parse_input(mut input: String) -> HashMap<String, Box<dyn Pulsar>> {
    let mut hash: HashMap<String, Box<dyn Pulsar>> = HashMap::new();
    input = input.chars().filter(|&c| c != ' ').collect();

    let re = Regex::new(r"([%&])(\w+)->([\w,]+)").unwrap();
    let re_br = Regex::new(r".->([\w,]+)").unwrap();

    let mut input_vecs: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        match re.captures(line) {
            Some(caps) => {
                let first = caps.get(1).unwrap().as_str();
                let second = caps.get(2).unwrap().as_str();
                let third = caps.get(3).unwrap().as_str();

                let name = second.to_string();
                let destinations: Vec<String> = third
                    .split(',')
                    .map(|c| c.to_string())
                    .collect();

                for destination in destinations {
                    input_vecs
                        .entry(destination.clone())
                        .or_insert(Vec::new())
                        .push(name.clone());
                }


                if first.chars().next().unwrap() == '%' {
                    hash.insert(second.to_string(), Box::new(FlipFlop{
                        name: name,
                        state: false,
                        destinations: third.split(',').map(|c| c.to_string()).collect(),
                    }));
                } else {
                    hash.insert(second.to_string(), Box::new(Conj{
                        name: second.to_string(),
                        inputs: HashMap::new(),
                        destinations: third.split(',').map(|c| c.to_string()).collect(),
                    }));
                }
            }
            None => {
                match re_br.captures(line) {
                    Some(caps) => {
                        let first = caps.get(1).unwrap().as_str();
                        let destinations: Vec<String> = first
                            .split(',')
                            .map(|c| c.to_string()).collect();
                        for destination in &destinations {
                            input_vecs
                                .entry(destination.clone())
                                .or_insert(Vec::new())
                                .push(String::from("broadcaster"));
                        }
                        hash
                            .insert(String::from("broadcaster"), Box::new(BroadCaster{
                            name: String::from("broadcaster"),
                            destinations: destinations,
                        }));
                    }
                    None => (),
                }
            }
        }
    }
    for (key, value) in &input_vecs {
        if let Some(node) = hash.get_mut(key) {
            node.add_inputs(&value);
        }
    }
    
    hash
}

#[derive(Debug)]
struct Pulse {
    origin: String,
    destination: String,
    is_high: bool,
}

#[derive(Debug)]
struct BroadCaster {
    name: String,
    destinations: Vec<String>,
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    state: bool,
    destinations: Vec<String>,
}

#[derive(Debug)]
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

    fn is_default(&self) -> bool {
        true
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
                        is_high: self.state,
                    })
            }
        }
        pulse_out
    }

    fn is_default(&self) -> bool {
        !self.state
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

    fn is_default(&self) -> bool {
        !self.inputs.values().any(|&value| value == true)
    }

    fn add_inputs(&mut self, inputs: &Vec<String>) {
        for input in inputs {
            self.inputs.insert(input.clone(), false);
        }

    }
}

pub trait Pulsar: Debug {
    fn receive_pulse(&mut self, pulse: Pulse) -> Vec<Pulse>;
    fn is_default(&self) -> bool;
    fn add_inputs(&mut self, inputs: &Vec<String>) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

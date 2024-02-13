use std::fs;
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let lines = parse_input(input);
    run_vec(lines);
    Ok(())
}

fn run_vec(lines: Vec<Line>) {
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }
            lines[i].crosses2d(&lines[j]);
        }
    }
}

fn parse_input(input: String) -> Vec<Line> {
    let mut line_vec: Vec<Line> = Vec::new();
    let re = Regex::new(r"(\d+),\s*(\d+),\s*(\d+)\s*@\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)").unwrap();
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let x: f64 = caps.get(1).map_or(Ok(0.0), |m| m.as_str().parse()).unwrap_or(0.0);
            let y: f64 = caps.get(2).map_or(Ok(0.0), |m| m.as_str().parse()).unwrap_or(0.0);
            let z: f64 = caps.get(3).map_or(Ok(0.0), |m| m.as_str().parse()).unwrap_or(0.0);
            let u: f64 = caps.get(4).map_or(Ok(0.0), |m| m.as_str().parse()).unwrap_or(0.0);
            let v: f64 = caps.get(5).map_or(Ok(0.0), |m| m.as_str().parse()).unwrap_or(0.0);
            let w: f64 = caps.get(6).map_or(Ok(0.0), |m| m.as_str().parse()).unwrap_or(0.0);
            let line = Line{ x: x, y: y, z: z, u: u, v: v, w: w };
            line_vec.push(line);
        }
    }
    line_vec
}

#[derive(Debug)]
struct Line {
    x: f64,
    y: f64,
    z: f64,
    u: f64,
    v: f64,
    w: f64,
}

impl Line {
    pub fn crosses2d(&self, other: &Line) -> Option<(f64, f64)> {
        let x_diff: f64 = other.x - self.x;
        let y_diff: f64 = other.y - self.y;

        let t: f64 = (x_diff * other.v - y_diff * other.u) / (self.u * other.v - self.v * other.u);
        let s: f64 = (x_diff * self.v - y_diff * self.u) / (other.u * self.v - other.v * self.u);


        let x_new = self.x + t * self.v;
        let x_new2 = other.x + s * other.v;
        println!("{x_new}");
        println!("{x_new2}\n\n");

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }
}

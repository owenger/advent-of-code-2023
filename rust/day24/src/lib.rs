use std::fs;
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

pub fn run_part_1(input_path: String, test: bool) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let lines = parse_input(input);
    let mut total: u32 = 0;
    if test {
        total = calculate2dboxcrosses(lines, 7.0, 27.0);
    } else {
        total = calculate2dboxcrosses(lines, 200000000000000.0, 400000000000000.0);
    }
    println!("Total: {total}");
    Ok(())
}

fn calculate2dboxcrosses(lines: Vec<Line>, min: f64, max: f64) -> u32 {
    let mut count: u32 = 0;
    for i in 0..lines.len() {
        for j in i+1..lines.len() {
            if let Some((cross_x, cross_y)) = lines[i].crosses2d(&lines[j]) {
                if cross_x >= min && cross_x <= max && cross_y >= min && cross_y <= max {
                    count += 1;
                }
            }
        }
    }
    return count;
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
        let det: f64 = self.u * other.v - self.v * other.u;

        if det.abs() < 0.00001 {
            return None; // Lines are parallel or coincident
        }

        let x_diff: f64 = other.x - self.x;
        let y_diff: f64 = other.y - self.y;

        let t: f64 = (x_diff * other.v - y_diff * other.u) / det;
        let s: f64 = (x_diff * self.v - y_diff * self.u) / det;

        if t < 0.0 || s < 0.0 {
            return None;
        }


        let x_intersect = self.x + t * self.u;
        let y_intersect = self.y + t * self.v;

        Some((x_intersect, y_intersect))
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

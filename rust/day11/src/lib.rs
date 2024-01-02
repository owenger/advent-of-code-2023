use std::fs;
use std::error::Error;
use std::collections::HashMap;

pub fn run_part_1(input_path: String, expansion: i64) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let bool_input = convert_to_bool_vec2d(input);
    let (x_add, y_add) = get_adding_vectors(&bool_input, expansion);

    let locations = get_locations(bool_input, x_add, y_add);
    let total_distance = get_distances(locations);
    println!("Total distance: {total_distance}");

    Ok(())
}

fn convert_to_bool_vec2d(input_string: String) -> Vec<Vec<bool>> {
    let mut vec_vec: Vec<Vec<bool>> = Vec::new();
    
    for line in input_string.lines() {
        let mut line_vec = Vec::new();
        for ch in line.chars() {
            match ch {
                '#' => line_vec.push(true),
                _ => line_vec.push(false),
            }
        }
        vec_vec.push(line_vec);
    }
    vec_vec
}

fn get_adding_vectors(input: &Vec<Vec<bool>>, mut expansion: i64) -> (Vec<i64>, Vec<i64>) {
    expansion -= 1;
    let rows = input.len();
    let cols = input[0].len();

    let mut x_add_vec = vec![0; cols];
    let mut y_add_vec = vec![0; rows];

    let mut x_count: i64 = 0;
    let mut y_count: i64 = 0;

    let mut false_cols = vec![true; cols];

    for (row, vector) in input.iter().enumerate() {
        y_add_vec[row] = y_count;
        if !vector.contains(&true) {
            y_count += expansion;
        }
        for (col, &value) in vector.iter().enumerate() {
            if value {
                false_cols[col] = false;
            }
        }
    }

    for (col, &value) in false_cols.iter().enumerate() {
        x_add_vec[col] = x_count;

        if value {
            x_count += expansion;
        }
    }

    (x_add_vec, y_add_vec)
}

fn get_locations(input: Vec<Vec<bool>>, x_add: Vec<i64>, y_add: Vec<i64>) -> HashMap<i64, Location> {
    let mut location_map = HashMap::new();
    let mut location_index: i64 = 0;

    for (row, vector) in input.iter().enumerate() {
        for (col, &value) in vector.iter().enumerate() {
            if value {
                let x = col as i64;
                let y = row as i64;
                location_map.insert(location_index, Location { x: x + x_add[col], y: y + y_add[row] });
                location_index += 1;
            }
        }
    }
    location_map
}

fn get_distances(locations: HashMap<i64, Location>) -> i64 {
    let mut distance: i64 = 0;
    for (key_a, location_a) in &locations {
        for (key_b, location_b) in &locations {
            if key_a == key_b {
                continue;
            }
            let x_dist = location_a.x - location_b.x;
            let y_dist = location_a.y - location_b.y;

            distance += x_dist.abs();
            distance += y_dist.abs();
        }
    }
    distance /= 2;
    distance
}

pub struct Location {
    pub x: i64,
    pub y: i64,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        let input = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, true, false],
        ];

        let expected_y_add_vec = vec![0, 1, 2];

        let (x_add_vec, y_add_vec) = get_adding_vectors(&input);

        assert_eq!(y_add_vec, expected_y_add_vec);
    }

    #[test]
    fn test_get_adding_x_vectors() {
        let input = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, true, false],
        ];

        let expected_x_add_vec = vec![0, 1, 1];

        let (x_add_vec, y_add_vec) = get_adding_vectors(&input);

        assert_eq!(x_add_vec, expected_x_add_vec);
    }
}

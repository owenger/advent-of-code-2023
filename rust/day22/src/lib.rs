use std::fs;
use std::error::Error;
use regex::Regex;
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let mut pieces = parse_input(input);
    pieces.sort();
    let support_dependencies = walk_pieces(&pieces);
    let removable = get_removable(&support_dependencies);
    let falls = get_number_of_falls(&support_dependencies);
    println!("Part 1 Res: {}", removable.len());
    
    Ok(())
}

fn get_number_of_falls(deps: &HashMap<i32, Vec<i32>>, removable: &[i32]) -> i32 {
    let mut fallen: Vec<i32> = Vec::new();

    for (key, value) in deps.iter() {

    }
    
    0



    // let mut reversed_deps: HashMap<i32, Vec<i32>> = HashMap::new();
    //
    // for (key, value) in deps.iter() {
    //     if value.len() == 1 {
    //         reversed_deps.entry(*value.first().unwrap()).or_insert_with(Vec::new).push(*key);
    //     }
    // }
    // println!("rd: {:?}", reversed_deps);
    //
    // let mut total: i32 = 0;
    //
    // for (key, value) in reversed_deps.iter() {
    //     let mut to_analyze = value.clone();
    //     total += to_analyze.len() as i32;
    //     loop {
    //         if to_analyze.len() == 0 {
    //             break;
    //         }
    //         let check = to_analyze.pop().unwrap();
    //         if let Some(vec) = reversed_deps.get(&check) {
    //             total += vec.len() as i32;
    //             to_analyze.extend(vec);
    //         }
    //     }
    // }
    // println!("cnt: {total}");


    // 0
}

fn get_removable(deps: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut keys: Vec<i32> = deps.keys().cloned().collect();

    for (key, value) in deps.iter() {
        if value.len() == 1 {
            keys.retain(|&x| x != *value.first().unwrap());
        }
    }
    keys
}

fn walk_pieces(pieces: &[Piece]) -> HashMap<i32, Vec<i32>> {
    let mut support_dependencies: HashMap<i32, Vec<i32>> = HashMap::new();
    let (x_max, y_max, _z_max) = get_dimensions(&pieces);
    let mut occupation: Vec<Vec<Option<Occupation>>> = vec![vec![None; (x_max + 1) as usize]; (y_max + 1) as usize];

    for i in 0..pieces.len() {
        let (height, ids) = find_max_hit(&pieces[i], &occupation);
        //println!("height: {}, ids: {:?}", height, ids);
        place_piece(&pieces[i], &mut occupation, height);
        support_dependencies.insert(pieces[i].id, ids);
    }
    //println!("{:?}", support_dependencies);
    support_dependencies
}

fn place_piece(piece: &Piece, occupation: &mut Vec<Vec<Option<Occupation>>>, height: i32) {
    for y in piece.y1..=piece.y2 {
        for x in piece.x1..=piece.x2 {
            let occ = Occupation{ piece_id: piece.id, height: height };
            occupation[y as usize][x as usize] = Some(occ);
        }
    }
}

fn find_max_hit(piece: &Piece, occupation: &Vec<Vec<Option<Occupation>>>) -> (i32, Vec<i32>) {
    let mut max_hit: i32 = 0;
    let mut ids: Vec<i32> = Vec::new();
    for y in piece.y1..=piece.y2 {
        for x in piece.x1..=piece.x2 {
            match &occupation[y as usize][x as usize] {
                Some(occ) => {
                    if occ.height == max_hit && !ids.contains(&occ.piece_id) {
                        ids.push(occ.piece_id);
                    } else if occ.height > max_hit {
                        max_hit = occ.height;
                        ids = vec![occ.piece_id];
                    }
                }
                None => (),
            }

        }
    }
    (max_hit + (1 + piece.z2 - piece.z1), ids)
}

fn get_dimensions(input: &[Piece]) -> (i32, i32, i32) {
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;
    let mut z_max: i32 = 0;

    for i in 0..input.len() {
        x_max = cmp::max(x_max, input[i].x2);
        y_max = cmp::max(y_max, input[i].y2);
        z_max = cmp::max(z_max, input[i].z2);
    }
    (x_max, y_max, z_max)
}

fn parse_input(input: String) -> Vec<Piece> {
    let mut pieces: Vec<Piece> = Vec::new();
    let re = Regex::new(r"(\w+),(\w+),(\w+)~(\w+),(\w+),(\w+)").unwrap();
    let mut count = 1;
    for line in input.lines() {
        match re.captures(line) {
            Some(caps) => {
                let x_first: i32 = caps.get(1).unwrap().as_str().parse().expect("Not a number");
                let y_first: i32 = caps.get(2).unwrap().as_str().parse().expect("Not a number");
                let z_first: i32 = caps.get(3).unwrap().as_str().parse().expect("Not a number");
                let x_second: i32 = caps.get(4).unwrap().as_str().parse().expect("Not a number");
                let y_second: i32 = caps.get(5).unwrap().as_str().parse().expect("Not a number");
                let z_second: i32 = caps.get(6).unwrap().as_str().parse().expect("Not a number");

                //println!("{x_first}, {x_second}, {y_first}, {y_second}, {z_first}, {z_second}");

                pieces.push(Piece{
                    id: count,
                    x1: cmp::min(x_first, x_second),
                    x2: cmp::max(x_first, x_second),
                    y1: cmp::min(y_first, y_second),
                    y2: cmp::max(y_first, y_second),
                    z1: cmp::min(z_first, z_second),
                    z2: cmp::max(z_first, z_second),
                })
            }
            None => println!("Didn't work!"),
        }
        count += 1;
    }
    pieces
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
struct Piece {
    id: i32,
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

#[derive(Debug)]
#[derive(Clone)]
struct Occupation {
    piece_id: i32,
    height: i32,
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z1.cmp(&other.z1)
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

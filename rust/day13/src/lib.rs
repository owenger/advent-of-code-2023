use std::fs;
use std::error::Error;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    use block::Block;
    let input = fs::read_to_string(input_path)?;
    let blocks = Block::parse_part_1(input);

    let mut res: usize = 0;

    for block in &blocks {
        let first = block.check_vertical().unwrap_or(0);
        let second = block.check_horizontal().unwrap_or(0);
        res += first;
        res += second;

    }

    println!("Result: {res}");

    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    use block::Block;
    let input = fs::read_to_string(input_path)?;
    let blocks = Block::parse_part_1(input);

    let mut res: usize = 0;

    let mut count: usize = 0;

    for block in &blocks {
        let first = block.check_vertical_with_tolerance(1).unwrap_or(0);
        let second = block.check_horizontal_with_tolerance(1).unwrap_or(0);

        if first == 0 && second == 0 {
            println!("No smudge found here: {:?}", block.data);
        }

        if first != 0 && second != 0 {
            println!("Double smudge found here: {:?}", block.data);
        }

        res += first;
        res += second;

        count += 1;
        if count % 1000 == 0 {
            println!("At ({}/{})", count, block.data.len());
        }
    }

    println!("Result: {res}");

    Ok(())
}

mod block {
    #[derive(Debug)]
    pub struct Block {
        pub data: Vec<String>,
    }

    impl Block {
        pub fn from_string_vector(data: Vec<String>) -> Block {
            Block{ data }
        }

        pub fn check_vertical_with_tolerance(&self, tolerance: usize) -> Option<usize> {
            let mut level: usize = 0;
            let mut diff: usize = 0;

            for i in 0..self.data.len()-1 {
                loop {
                    if i < level || i + level + 1 == self.data.len() {
                        if diff == tolerance {
                            return Some((i + 1) * 100);
                        } else {
                            level = 0;
                            diff = 0;
                            break;
                        }
                    }
                    diff += self.get_row_diff(i - level, i + level + 1);
                    if diff > tolerance {
                        level = 0;
                        diff = 0;
                        break;
                    } else {
                        level += 1;
                    }
                }
            }
            None
        }

        pub fn check_horizontal_with_tolerance(&self, tolerance: usize) -> Option<usize> {
            let mut level: usize = 0;
            let mut diff: usize = 0;

            let length = self.data[0].len();

            for i in 0..length-1 {
                loop {
                    if i < level || i + level + 1 == length {
                        if diff == tolerance {
                            return Some(i + 1);
                        } else {
                            level = 0;
                            diff = 0;
                            break;
                        }
                    }
                    diff += self.get_col_diff(i - level, i + level + 1);
                    if diff > tolerance {
                        level = 0;
                        diff = 0;
                        break;
                    } else {
                        level += 1;
                    }
                }
            }
            None
        }

        pub fn check_vertical(&self) -> Option<usize> {
            let mut level: usize = 0;
            
            for i in 0..self.data.len()-1 {
                loop {
                    if i < level || i + level + 1 == self.data.len() {
                        return Some((i + 1) * 100);
                    } else if self.data[i - level] == self.data[i + level + 1] {
                        level += 1;
                    } else {
                        level = 0;
                        break;
                    }
                }
            }
            None
        }

        pub fn check_horizontal(&self) -> Option<usize> {
            let mut level: usize = 0;
            let length = self.data[0].len();

            for i in 0..length-1 {
                loop {
                    if i < level || i + level + 1 == length {
                        return Some(i + 1);
                    } else if self.compare_col(i - level, i + level + 1) {
                        level += 1;
                    } else {
                        level = 0;
                        break;
                    }

                }
            }
            None
        }

        fn compare_col(&self, col_1: usize, col_2: usize) -> bool {
            for i in 0..self.data.len() {
                if self.data[i].chars().nth(col_1) != self.data[i].chars().nth(col_2) {
                    return false;
                }
            }
            true
        }

        pub fn get_col_diff(&self, col_1: usize, col_2: usize) -> usize {
            let mut diff: usize = 0;

            for i in 0..self.data.len() {
                if self.data[i].chars().nth(col_1) != self.data[i].chars().nth(col_2) {
                    diff += 1;
                }
            }
            diff
        }

        pub fn get_row_diff(&self, row_1: usize, row_2: usize) -> usize {
            let mut diff: usize = 0;

            for i in 0..self.data[row_1].len() {
                if self.data[row_1].chars().nth(i) != self.data[row_2].chars().nth(i) {
                    diff += 1;
                }
            }
            diff
        }

        pub fn parse_part_1(input_string: String) -> Vec<Block> {
            let block_strings: Vec<String> = input_string.split("\n\n")
                                        .map(|s| s.to_string())
                                        .collect();
            let mut blocks: Vec<Block> = Vec::new();
            for string in block_strings {
                blocks.push(Block::from_string_vector(string.split("\n")
                                                      .filter(|s| !s.is_empty())
                                                      .map(|s| s.to_string())
                                                      .collect()));
            }
            blocks
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adding_y_vectors() {
        assert!(true);
    }

    #[test]
    fn test_row_difference() {
        let strings = vec![String::from("aabb"), String::from("bbab")];
        let block = block::Block::from_string_vector(strings);
        let diff = block.get_row_diff(0, 1);
        assert_eq!(diff, 3);
    }

    #[test]
    fn test_col_difference() {
        let strings = vec![String::from("aabb"), String::from("bbab"), String::from("abcd")];
        let block = block::Block::from_string_vector(strings);
        let diff = block.get_col_diff(0, 1);
        assert_eq!(diff, 1);
    }

    #[test]
    fn test_row_smudge() {
        let strings = vec!["#.#.###..#..#".to_string(), ".##.###.#....".to_string(), "#.###.#......".to_string(), "....##....##.".to_string(), "#.#.#..#.####".to_string(), "...#.#....##.".to_string(), ".#.##.#.#####".to_string(), "...##.#.#####".to_string(), "...#.#....##.".to_string()];
        let block = block::Block::from_string_vector(strings);
        let res = block.check_vertical_with_tolerance(1).unwrap_or(0);
        let res1 = block.check_horizontal_with_tolerance(1).unwrap_or(0);
        assert_eq!(700, res);
        assert_eq!(0, res1);

    }
}

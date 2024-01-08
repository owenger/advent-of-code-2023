use std::fs;
use std::error::Error;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let block = block::Block::from_string_and_transpose(input);

    let result = block.get_weight();
    println!("Result: {result}");
    Ok(())
}

mod block {
    #[derive(Debug)]
    pub struct Block {
        data: Vec<Vec<char>>,
    }

    impl Block {
        pub fn get_weight(&self) -> usize {
            let mut total: usize = 0;
            for row in &self.data {
                let sub_total = Self::get_row_weight(&row);
                //println!("Row: {:?}, weight: {sub_total}", row);
                total += sub_total;
            }
            total
        }

        fn get_row_weight(line: &[char]) -> usize {
            let mut total: usize = 0;
            let mut count: usize = 0;

            for i in 0..line.len() {
                match line[i] {
                    'O' => count += 1,
                    '#' => {
                        total += Self::get_value(count, i);
                        count = 0;
                    } 
                    _ => (),
                }
            }
            total += Self::get_value(count, line.len());
            total
        }

        fn get_value(count: usize, index: usize) -> usize {
            let mut total: usize = 0;
            for i in 0..count {
                total += index - i;
            }
            //println!("Count: {count}, index: {index}, total: {total}");
            total
        }

        pub fn from_string_and_transpose(input: String) -> Block {
            let mut matrix: Vec<Vec<char>> = Vec::new();
            let mut data: Vec<Vec<char>> = Vec::new();

            for line in input.lines().rev() {
                let chars: Vec<char> = line.chars().collect();
                matrix.push(chars);
            }

            let rows = matrix.len();
            let cols = matrix.first().map_or(0, Vec::len);

            for col in 0..cols {
                let mut line: Vec<char> = Vec::new();
                for row in 0..rows {
                    line.push(matrix[row][col]);
                }
                data.push(line);
            }

            Block{ data: data }
        }

        pub fn rotate_block(&self) {
            let rotated_data = self.data.clone();

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
}

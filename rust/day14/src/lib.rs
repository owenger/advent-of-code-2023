use std::fs;
use std::error::Error;
use std::collections::VecDeque;

pub fn run_part_1(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let block = block::Block::from_string_and_transpose(input);

    let result = block.get_shifted_weight();
    println!("Result: {result}");
    Ok(())
}

pub fn run_part_2(input_path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let mut my_block = block::Block::from_string_and_transpose(input);

    let mut count = 0;
    let mut max_count = 1000000000;
    let max_deque_length = my_block.data.len() * my_block.data.len();

    let mut data_history = VecDeque::with_capacity(max_deque_length);

    let mut reduced = false;

    'outer: loop {
        let result = my_block.get_weight();
        if data_history.len() == max_deque_length {
            data_history.pop_front();
        }
        let hash = my_block.hash_block();
        if !reduced {
            for i in 1..=data_history.len() {
                if data_history[data_history.len() - i] == hash {
                    let diff = i;

                    max_count = (max_count - count) % diff;
                    println!("Count: {count}, diff: {diff}, max_count: {max_count}");
                    count = 0;
                    reduced = true;
                    break;
                }
            }
        }

        count += 1;
        my_block.rotate_block();
        if count == max_count {
            break 'outer;
        }
        data_history.push_back(hash);
    }

    let result = my_block.get_weight();
    println!("Result: {result}");
    Ok(())
}

mod block {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    #[derive(Debug)]
    pub struct Block {
        pub data: Vec<Vec<char>>,
    }

    impl Block {
        pub fn get_shifted_weight(&self) -> usize {
            let mut total: usize = 0;
            for row in &self.data {
                let sub_total = Self::get_row_weight(&row);
                //println!("Row: {:?}, weight: {sub_total}", row);
                total += sub_total;
            }
            total
        }

        pub fn get_weight(&self) -> usize {
            let mut total: usize = 0;

            let rows = self.data.len();
            let cols = self.data.first().map_or(0, Vec::len);

            for row in 0..rows {
                for col in 0..cols {
                    if self.data[row][col] == 'O' {
                        total += col + 1;
                    }
                }
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

        pub fn rotate_block(&mut self) {
            let mut rotated_data: Vec<Vec<char>> = Vec::new();
            Self::push_data_right(&mut self.data);
            //println!("{:?}", self.data);
            for _ in 0..3 {
                self.data = Self::rotate_data(&self.data);
                Self::push_data_right(&mut self.data);
                //println!("{:?}", self.data);
            }
            self.data = Self::rotate_data(&self.data);
            //println!("{:?}", self.data)
        }

        fn rotate_data(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
            let mut new_data: Vec<Vec<char>> = Vec::new();
            
            let rows: usize = data.len();
            let cols: usize = data.first().map_or(0, Vec::len);

            for col in 0..cols {
                let mut row_vec: Vec<char> = Vec::new();
                for row in (0..rows).rev() {
                    row_vec.push(data[row][col]);
                }
                new_data.push(row_vec);
            }

            new_data
        }

        fn push_data_right(data: &mut Vec<Vec<char>>) {
            let rows: usize = data.len();
            let cols: usize = data.first().map_or(0, Vec::len);

            for row in 0..rows {
                let mut count: usize = 0;

                for col in 0..cols {
                    match data[row][col] {
                        'O' => {
                            count += 1;
                            data[row][col] = '.';
                        } 
                        '#' => {
                            Block::fill_up_row(&mut data[row], col, count);
                            count = 0;
                        }
                        _ => (),
                    }
                }
                Block::fill_up_row(&mut data[row], cols, count);
            }
        }

        fn fill_up_row(row: &mut Vec<char>, index: usize, count: usize) {
            for i in 1..=count {
                row[index - i] = 'O';
            }
        }

        pub fn compare_data(data_1: &Vec<Vec<char>>, data_2: &Vec<Vec<char>>) -> bool {
            let rows = data_1.len();
            let cols = data_2.first().map_or(0, Vec::len);

            for row in 0..rows {
                for col in 0..cols {
                    if data_1[row][col] != data_2[row][col] {
                        return false;
                    }
                }
            }
            true
        }

        pub fn hash_block(&self) -> u64 {
            let mut hasher = DefaultHasher::new();
            for row in 0..self.data.len() {
                for col in 0..self.data.first().map_or(0, Vec::len) {
                    self.data[row][col].hash(&mut hasher);
                }
            }
            hasher.finish()
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
    fn test_0() {
        let string: String = String::from("#..\n.#.\n...");
        let mut block = block::Block::from_string_and_transpose(string);
        block.rotate_block();
        assert!(false);
    }
}

use std::{fmt::Display, vec};

#[derive(Debug)]
pub struct Sudoku {
    matrix: [[u8; 9]; 9],
    missing_cell_coords: Vec<(u8, u8)>,
    visited_missing_cell_coords_index: usize,
}

impl Sudoku {
    pub fn new(v: [[u8; 9]; 9]) -> Sudoku {
        Sudoku {
            matrix: v,
            missing_cell_coords: Sudoku::scan_for_missing_cell_coords(v),
            visited_missing_cell_coords_index: 0,
        }
    }

    fn scan_for_missing_cell_coords(v: [[u8; 9]; 9]) -> Vec<(u8, u8)> {
        let mut result: Vec<(u8, u8)> = vec![];

        for (i, row) in v.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 0u8 {
                    result.push((i as u8, j as u8));
                }
            }
        }

        result
    }

    /**
    Return the missing number in the current block. Matrix is divided into 9 blocks as shown below.

    ```txt
    0 0 0 | 0 0 0 | 0 0 0
    0 0 0 | 0 1 0 | 0 2 0
    0 0 0 | 0 0 0 | 0 0 0
    ---------------------
    0 0 0 | 0 0 0 | 0 0 0
    0 3 0 | 0 4 0 | 0 5 0
    0 0 0 | 0 0 0 | 0 0 0
    ---------------------
    0 0 0 | 0 0 0 | 0 0 0
    0 6 0 | 0 7 0 | 0 8 0
    0 0 0 | 0 0 0 | 0 0 0
    ```
    */
    fn get_missing_numbers_in_block(&self) -> Vec<u8> {
        // get the coordinate of the currently visiting cell
        let (y, x) = self.missing_cell_coords[self.visited_missing_cell_coords_index];

        // find which block the cell's living in
        let x_block_offset = (x) / 3;
        let y_block_offset = (y) / 3;

        let mut missing_numbers: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        for i in (3 * x_block_offset)..(3 * x_block_offset + 3) {
            for j in (3 * y_block_offset)..(3 * y_block_offset + 3) {
                let num = self.matrix[j as usize][i as usize];

                let index = missing_numbers.iter().position(|x| *x == num);
                if let Some(x) = index {
                    missing_numbers.remove(x);
                }
            }
        }

        missing_numbers
    }

    fn look_for_available_numbers_in_row_and_col(&self) -> Vec<u8> {
        let col_available_numbers = self.look_for_available_numbers_in_column();
        let row_available_numbers = self.look_for_available_numbers_in_row();

        let mut result = vec![];

        // closure to check that there's value `x` in `col_available_numbers`
        let in_cols = |x: u8| -> bool { col_available_numbers.iter().any(|f| *f == x) };

        for n in row_available_numbers.iter() {
            if in_cols(*n) {
                result.push(*n)
            }
        }

        result
    }

    fn look_for_available_numbers_in_column(&self) -> Vec<u8> {
        let mut available_numbers: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        // get the coordinate of the currently visiting cell
        let (y, x) = self.missing_cell_coords[self.visited_missing_cell_coords_index];

        for i in 0..9 {
            if i == y {
                continue; // skip the current cell
            }

            let cell_value = self.matrix[i as usize][x as usize];
            if cell_value != 0 {
                let index = available_numbers.iter().position(|x| *x == cell_value);

                if let Some(x) = index {
                    available_numbers.remove(x);
                }
            }
        }

        available_numbers
    }

    fn look_for_available_numbers_in_row(&self) -> Vec<u8> {
        let mut available_numbers: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        // get the coordinate of the currently visiting cell
        let (y, x) = self.missing_cell_coords[self.visited_missing_cell_coords_index];

        for i in 0..9 {
            if i == x {
                continue; // skip the current cell
            }

            let cell_value = self.matrix[y as usize][i as usize];
            if cell_value != 0 {
                let index = available_numbers.iter().position(|x| *x == cell_value);

                if let Some(x) = index {
                    available_numbers.remove(x);
                }
            }
        }

        available_numbers
    }

    fn place_number(&mut self) {
        // get current cell coordinate
        let (curr_cell_coord_y, curr_cell_coord_x) =
            self.missing_cell_coords[self.visited_missing_cell_coords_index];

        // find missing numbers in the current block
        let missing_numbers = self.get_missing_numbers_in_block();

        // find available numbers in row and column
        let available_numers_in_col_and_row = self.look_for_available_numbers_in_row_and_col();

        // find placable numbers
        let mut placable_numbers: Vec<u8> = vec![];
        for num in missing_numbers.iter() {
            if available_numers_in_col_and_row.iter().any(|x| x == num) {
                placable_numbers.push(*num);
            }
        }

        // if there's nothing we can place, move backward and retry
        if placable_numbers.len() == 0 {
            self.matrix[curr_cell_coord_y as usize][curr_cell_coord_x as usize] = 0;
            self.move_backward();
            self.move_backward();
            return;
        }

        // sort the result, so we can try bigger num when move backward
        placable_numbers.sort();

        let current_cell_number =
            self.matrix[curr_cell_coord_y as usize][curr_cell_coord_x as usize];

        for num in placable_numbers.iter() {
            // always try a bigger num than the current cell
            if *num > current_cell_number {
                self.matrix[curr_cell_coord_y as usize][curr_cell_coord_x as usize] = *num;

                // todo: might need to change
                // self.move_forward();

                return;
            }
        }

        self.matrix[curr_cell_coord_y as usize][curr_cell_coord_x as usize] = 0;
        self.move_backward();
        self.move_backward();
    }

    pub fn move_forward(&mut self) -> bool {
        if self.missing_cell_coords.len() - 1 == self.visited_missing_cell_coords_index {
            return false;
        }

        self.visited_missing_cell_coords_index += 1;
        return true;
    }

    pub fn move_backward(&mut self) -> bool {
        if self.visited_missing_cell_coords_index != 0 {
            self.visited_missing_cell_coords_index -= 1;
            return true;
        }

        return false;
    }

    pub fn solve(&mut self) {
        let max_loop_count = 100_000u64;
        let mut loop_count = 0u64;

        self.place_number();

        while self.move_forward() {
            if loop_count >= max_loop_count {
                break;
            }

            self.place_number();

            loop_count += 1;
        }
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("");

        for (i, row) in self.matrix.iter().enumerate() {
            let mut tmp = String::from("");
            for (j, cell) in row.iter().enumerate() {
                let cell_str = cell.to_string();
                tmp.push_str(&cell_str);

                if j == 2 || j == 5 {
                    tmp.push_str(" | ");
                } else {
                    tmp.push(' ');
                }
            }
            output.push_str(&tmp);
            output.push('\n');
            if i == 2 || i == 5 {
                output.push_str("---------------------\n");
            }
        }

        write!(f, "{}", output)
    }
}

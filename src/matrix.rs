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

    pub fn print_locked_cells_coords(&self) {
        print!("locked cells coordinates are: ");
        for v in self.missing_cell_coords.iter() {
            print!("({}, {}), ", v.0, v.1);
        }
        print!("\n");
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
        let (x, y) = self.missing_cell_coords[self.visited_missing_cell_coords_index];

        // find which block the cell's living in
        let x_block_offset = 9 / (x + 1);
        let y_block_offset = 9 / (y + 1);

        let mut missing_numbers: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        for i in (3 * x_block_offset)..(x_block_offset + 3) {
            for j in (3 * y_block_offset)..(y_block_offset + 3) {
                let num = self.matrix[i as usize][j as usize];

                let index = missing_numbers.iter().position(|x| *x == num).unwrap();
                missing_numbers.remove(index);
            }
        }

        missing_numbers
    }

    fn look_for_available_numbers_in_column(&self) -> Vec<u8> {
        // get the coordinate of the currently visiting cell
        let (x, y) = self.missing_cell_coords[self.visited_missing_cell_coords_index];


    }

    fn look_for_available_numbers_in_row(&self) -> Vec<u8> {}

    fn place_number(&mut self) {
        let missing_numbers = self.get_missing_numbers_in_block();

        // scan row
        // scan column
    }

    pub fn move_forward(&mut self) -> bool {
        if self.missing_cell_coords.len() != self.visited_missing_cell_coords_index {
            self.visited_missing_cell_coords_index += 1;
            return true;
        }

        return false;
    }

    pub fn move_backward(&mut self) -> bool {
        if self.visited_missing_cell_coords_index != 0 {
            self.visited_missing_cell_coords_index -= 1;
            return true;
        }

        return false;
    }

    pub fn solve(&self) {

        // see if the current cell can place any number, place it.
        // if the current cell cannot place any number, then backtrack 1 cell and then retry and move forward
        // keep doing until reaching the last cell
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

#[derive(Clone, Copy, Debug)]
pub struct Row([Cell; 9]);

impl Row {
    pub fn new(v: [u8; 9]) -> Row {
        let mut cells = [Cell(0); 9];

        for (i, c) in v.iter().enumerate() {
            cells[i] = Cell::new(*c);
        }

        Row(cells)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell(u8);

impl Cell {
    pub fn new(i: u8) -> Cell {
        Cell(i)
    }
}

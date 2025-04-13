/// Represents a cell token in the user input
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

impl Cell {
    /// Returns true if range (this:other) is valid
    pub fn compare_cells(&self, other: &Self) -> bool {
        self.row <= other.row && self.col <= other.col
    }

    /// Returns true if the cell is within the spreadsheet's bound i.e row <= 999 and col <= 18278
    pub fn is_valid_cell(&self) -> bool {
        self.row >=1 && self.row <= 999 && self.col >= 1 && self.col <= 18278 
    }
}

/// Represents a value (cell or constant) token in the user input
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Value {
    Cell(Cell),
    Constant(i32),
}

/// Converts a cell's string representation to Cell struct. Example - A1 -> Cell { row: 1, col: 1 }
pub fn convert_string_to_cell(cell: &str) -> Option<Cell> {
    let mut col = 0;
    let mut row_start = 0;

    // Starts parsing the column
    for (i, c) in cell.chars().enumerate() {
        if c.is_ascii_alphabetic() {
            col = col * 26 + ((c as u8 - b'A' + 1) as usize);
        } else {
            row_start = i;
            break;
        }
    }

    // If no alphabet in the string or no digits in the string
    if row_start == 0 || row_start >= cell.len() {
        return None;
    }

    // Parses the row number of the cell
    match cell[row_start..].parse::<usize>() {
        Ok(row) => {
            let cell_struct = Cell {row: row, col: col};
            Some(cell_struct)
        }
        Err(_) => None,
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_cells_returns_true() {
        let cell_1 = Cell {row: 5, col: 5};
        let cell_2 = vec![
            Cell { row: 6, col: 6 },    // Reactangle range
            Cell { row: 5, col: 6 },    // Row range
            Cell { row: 6, col: 5 },    // Column range
            Cell { row: 5, col: 5 },    // One cell ranges are valid
        ];
        for cell in cell_2 {
            assert!(cell_1.compare_cells(&cell));
        }
    }

    #[test]
    fn test_compare_cells_returns_false() {
        let cell_1 = Cell {row: 5, col: 5};
        let cell_2 = vec![
            Cell { row: 4, col: 4 },    // Reactangle range
            Cell { row: 5, col: 4 },    // Row range
            Cell { row: 4, col: 5 },    // Column range
        ];
        for cell in cell_2 {
            assert!(!cell_1.compare_cells(&cell));
        }
    }

    #[test]
    fn test_is_valid_cell_returns_some() {
        let valid_cells = vec![
            Cell { row: 1, col: 1 },        // Top-leftmost cell of the spreadsheet
            Cell { row: 100, col: 100 },    // Arbitrary cell in the workseet
            Cell { row: 999, col: 18278 },  // Bottom-rightmost cell in the spreadsheet 
        ];

        let invalid_cells = vec![
            Cell { row: 0, col: 0 },            // Spreadsheet is (1,1)-indexed
            Cell { row: 1000, col: 100 },       // Row is out of bounds
            Cell { row: 100, col: 100000 },      // Column in out of bounds
            Cell { row: 1000, col: 100000 }     // Both rows and columns are out of bound
        ];

        for cell in valid_cells {
            assert!(cell.is_valid_cell());
        }

        for cell in invalid_cells {
            assert!(!cell.is_valid_cell());
        }
    }

    #[test]
    fn test_convert_string_to_cell_valid() {
        assert_eq!(
            convert_string_to_cell("A1"),
            Some(Cell { row: 1, col: 1 })
        );
        assert_eq!(
            convert_string_to_cell("Z99"),
            Some(Cell { row: 99, col: 26 })
        );
        assert_eq!(
            convert_string_to_cell("AA10"),
            Some(Cell { row: 10, col: 27 })
        );
        assert_eq!(
            convert_string_to_cell("ZZ999"),
            Some(Cell { row: 999, col: 702 })
        );
        assert_eq!(
            convert_string_to_cell("ZZZ1"),
            Some(Cell { row: 1, col: 18278 })
        );
    }

    #[test]
    fn test_convert_string_to_cell_invalid() {
        assert_eq!(convert_string_to_cell("123"), None);        // No column
        assert_eq!(convert_string_to_cell("AB"), None);         // No row
        assert_eq!(convert_string_to_cell("A0"), Some(Cell { row: 0, col: 1 }));    // Valid parse but invalidated by is_valid_cell
        assert_eq!(convert_string_to_cell(""), None);           // Empty string
        assert_eq!(convert_string_to_cell("A-1"), None);        // String to integer parsing failed
        assert_eq!(convert_string_to_cell("A1.2"), None);        // String to integer parsing failed
    }
}

use crate::spreadsheet::SpreadSheet;

/// Converts the column string to column number using 1-indexing
fn column_number_to_letters(mut col: usize) -> String {
    let mut letters = String::new();
    while col > 0 {
        col -= 1;
        letters.insert(0, (b'A' + (col % 26) as u8) as char);
        col /= 26;
    }
    letters
}

/// Prints a 10x10 grid with (start_row, start_col as the top-left cell)
pub fn print_sheet(
    start_row: usize,
    start_col: usize,
    spreadsheet: &SpreadSheet,
    max_rows: usize,
    max_cols: usize,
) {
    print!("\t");
    for col in start_col..(start_col + 10).min(max_cols + 1) {
        print!("{}\t", column_number_to_letters(col));
    }
    println!();

    for row in start_row..(start_row + 10).min(max_rows + 1) {
        print!("{}\t", row);
        for col in start_col..(start_col + 10).min(max_cols + 1) {
            let value = spreadsheet.get_cell_value(row - 1, col - 1);
            if value.is_none() {
                print!("ERR\t");
            } else {
                print!("{}\t", value.unwrap());
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_number_to_letters_single_letter() {
        assert_eq!(column_number_to_letters(1), "A");
        assert_eq!(column_number_to_letters(2), "B");
        assert_eq!(column_number_to_letters(26), "Z");
    }

    #[test]
    fn test_column_number_to_letters_double_letters() {
        assert_eq!(column_number_to_letters(27), "AA");
        assert_eq!(column_number_to_letters(28), "AB");
        assert_eq!(column_number_to_letters(52), "AZ");
        assert_eq!(column_number_to_letters(53), "BA");
    }

    #[test]
    fn test_column_number_to_letters_triple_letters() {
        assert_eq!(column_number_to_letters(702), "ZZ");
        assert_eq!(column_number_to_letters(703), "AAA");
        assert_eq!(column_number_to_letters(704), "AAB");
        assert_eq!(column_number_to_letters(728), "AAZ");
        assert_eq!(column_number_to_letters(729), "ABA");
    }

    #[test]
    fn test_column_number_to_letters_large_values() {
        assert_eq!(column_number_to_letters(18278), "ZZZ"); // 26^3
        assert_eq!(column_number_to_letters(18279), "AAAA");
    }

    #[test]
    fn test_column_number_to_letters_zero() {
        assert_eq!(column_number_to_letters(0), ""); // Should return empty string
    }
}

fn column_number_to_letters(mut col: usize) -> String {
    let mut letters = String::new();
    while col > 0 {
        col -= 1;
        letters.insert(0, (b'A' + (col % 26) as u8) as char);
        col /= 26;
    }
    letters
}

pub fn print_sheet(start_row: usize, start_col: usize, max_rows: usize, max_cols: usize) {
    print!("\t");
    for col in start_col..(start_col + 10).min(max_cols) {
        print!("{}\t", column_number_to_letters(col));
    }
    println!();

    for row in start_row..(start_row + 10).min(max_rows) {
        print!("{}\t", row);
        for _col in start_col..(start_col + 10).min(max_cols) {
            let value = 1000;
            print!("{}\t", value);
        }
        println!();
    }
}

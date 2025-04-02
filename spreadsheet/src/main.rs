mod utils;
mod cell;
mod equation;
mod spreadsheet;

use spreadsheet::SpreadSheet;

fn main() {
	let mut spreadsheet = SpreadSheet::<i32>::new(20000,1000);

	for row in 0..20000{
		for col in 0..1000{
			spreadsheet.set_cell_value(row,col,1);
			print!("{}", spreadsheet.get_cell_value(row,col));
		}
	}
}

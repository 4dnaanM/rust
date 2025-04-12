mod utils;
mod operand;
mod equation;
mod spreadsheet;

use equation::Equation;
use operand::{SharedOperand, Operand};
use spreadsheet::SpreadSheet;
use utils::{Coordinate,Type};

use std::cell::RefCell;

fn main() {
	let mut spreadsheet = SpreadSheet::new(20,10);

	for row in 0..10{
		for col in 0..10{
			let ops = vec![
				SharedOperand::new(RefCell::new(
					if row >=1 {spreadsheet.cells[row-1][col].borrow().clone()} 
					else {Operand::new(Some((row,col)),Some((row+col).try_into().unwrap()))})
				),
				SharedOperand::new(RefCell::new(
					if col >=1 {spreadsheet.cells[row][col-1].borrow().clone()} 
					else {Operand::new(Some((row,col)),Some((row+col).try_into().unwrap()))}))
			];
			let eq = Equation::new(Coordinate(row,col),Some(Type::ADD), Some(ops));
			spreadsheet.set_cell_equation(row,col,eq);
			// print!("|{:6}", spreadsheet.get_cell_value(row, col));
		}
		// println!("|");
	}
	println!("-----------------------------------------");
	let to_change = Coordinate(5,5);
	println!("Updating cell (5,5) to 20");
	spreadsheet.set_cell_equation(
		to_change.0,
		to_change.1, 
		Equation::new(
			to_change, 
			Some(Type::ADD), 
			Some(vec![
				SharedOperand::new(RefCell::new(Operand::new(Some(to_change),Some(10)))),
				SharedOperand::new(RefCell::new(Operand::new(Some(to_change),Some(10))))
			])
		)
	);
	println!("Updated cell (0,0) to 20");
	for row in 0..10{
		for col in 0..10{
			print!("|{:6}", spreadsheet.get_cell_value(row, col));
		}
		println!("|");
	}
	println!("-----------------------------------------");
	


}

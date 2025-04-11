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
	let mut spreadsheet = SpreadSheet::<i32>::new(20,10);

	for row in 0..10{
		for col in 0..10{
			let ops = vec![
				SharedOperand::new(RefCell::new(
					if row >=1 {spreadsheet.cells[row-1][col].borrow().clone()} 
					else {Operand::new(row,col,Some((row+col).try_into().unwrap()),None)})
				),
				SharedOperand::new(RefCell::new(
					if col >=1 {spreadsheet.cells[row][col-1].borrow().clone()} 
					else {Operand::new(row,col,Some((row+col).try_into().unwrap()),None)}))
			];
			let eq = Equation::new(Coordinate(row,col),Some(Type::ADD), Some(ops));
			spreadsheet.set_cell_equation(row,col,eq);
			print!("|{:6}", spreadsheet.get_cell_value(row, col));
		}
		println!("|");
	}
	println!("-----------------------------------------");
	spreadsheet.set_cell_equation(
		0,
		0, 
		Equation::new(
			Coordinate(0,0), 
			Some(Type::ADD), 
			Some(vec![
				SharedOperand::new(RefCell::new(Operand::new(0,0,Some(10), None))),
				SharedOperand::new(RefCell::new(Operand::new(0,0,Some(10), None)))
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

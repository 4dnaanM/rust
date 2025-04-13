mod utils;
mod operand;
mod equation;
mod spreadsheet;

use equation::Equation;
use operand::{SharedOperand, Operand};
use spreadsheet::SpreadSheet;
use utils::{Coordinate,Type};

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
	let m = 10;
	let n = 10;
	let mut spreadsheet = SpreadSheet::new(m,n);

	for row in 0..m{
		for col in 0..n{
			let ops = vec![
				if row>=1 {spreadsheet.cells[row-1][col].clone()} 
				else {Rc::new(RefCell::new(Operand::new(Some((row,col)),Some((row+col).try_into().unwrap()))))}
				,
				if col>=1 {spreadsheet.cells[row][col-1].clone()} 
				else {Rc::new(RefCell::new(Operand::new(Some((row,col)),Some((row+col).try_into().unwrap()))))}
			];
			let eq = Equation::new(Coordinate(row,col),Some(Type::SUB), Some(ops));
			spreadsheet.set_cell_equation(row,col,eq);
		}
	}
	spreadsheet.print();
	let to_change = Coordinate(5,5);
	spreadsheet.set_cell_equation(
		to_change.0,
		to_change.1, 
		Equation::new(
			to_change, 
			Some(Type::ADD), 
			Some(vec![
				// spreadsheet.cells[0][0].clone(),
				spreadsheet.cells[0][0].clone(),
				// SharedOperand::new(RefCell::new(Operand::new(Some(to_change),Some(10)))),
				SharedOperand::new(RefCell::new(Operand::new(Some(to_change),Some(10))))
			])
		)
	);
	spreadsheet.print();

	spreadsheet.set_cell_equation(
		3,
		3, 
		Equation::new(
			to_change, 
			Some(Type::ADD), 
			Some(vec![
				// spreadsheet.cells[0][0].clone(),
				spreadsheet.cells[3][4].clone(),
				// SharedOperand::new(RefCell::new(Operand::new(Some(to_change),Some(10)))),
				SharedOperand::new(RefCell::new(Operand::new(Some(to_change),Some(10))))
			])
		)
	);

	spreadsheet.print();

}

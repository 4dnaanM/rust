mod utils;
mod operand;
mod equation;
mod spreadsheet;

use equation::Equation;
use operand::{SharedOperand, Operand};
use spreadsheet::SpreadSheet;
use utils::{Coordinate,Type};

fn main() {
	let m = 10;
	let n = 10;
	let mut spreadsheet = SpreadSheet::new(m,n);

	for row in 0..m{
		for col in 0..n{
			let ops = vec![
				// if row>=1 {spreadsheet.cells[row-1][col].clone()} 
				// else {SharedOperand::new(Operand::new(Some((row,col)),Some((row+col).try_into().unwrap())))}
				// ,
				// if col>=1 {spreadsheet.cells[row][col-1].clone()} 
				// else {SharedOperand::new(Operand::new(Some((row,col)),Some((row+col).try_into().unwrap())))}
				// SharedOperand::new(Operand::new(Some((row,col)),Some((row+col).try_into().unwrap()))),
				SharedOperand::new(Operand::new(Some((row,col)),Some((0).try_into().unwrap()))),
				SharedOperand::new(Operand::new(Some((row,col)),Some(0)))
			];
			let eq = Equation::new(Coordinate(row,col),Some(Type::ADD), Some(ops));
			spreadsheet.set_cell_equation(row,col,eq);
		}
	}
	spreadsheet.print();

	// // AUTO_RECALC_CHECK
	// let to_change = Coordinate(5,5);
	// spreadsheet.set_cell_equation(
	// 	to_change.0,
	// 	to_change.1, 
	// 	Equation::new(
	// 		to_change, 
	// 		Some(Type::ADD), 
	// 		Some(vec![
	// 			// spreadsheet.cells[0][0].clone(),
	// 			// spreadsheet.cells[0][0].clone(),
	// 			SharedOperand::new(Operand::new(Some(to_change),Some(10))),
	// 			SharedOperand::new(Operand::new(Some(to_change),Some(10)))
	// 		])
	// 	)
	// );
	// spreadsheet.print();

	// CYCLE CHECK
	// spreadsheet.set_cell_equation(
	// 	3,
	// 	3, 
	// 	Equation::new(
	// 		to_change, 
	// 		Some(Type::ADD), 
	// 		Some(vec![
	// 			// spreadsheet.cells[0][0].clone(),
	// 			// spreadsheet.cells[3][4].clone(),
	// 			SharedOperand::new(Operand::new(Some(to_change),Some(10))),
	// 			SharedOperand::new(Operand::new(Some(to_change),Some(10)))
	// 		])
	// 	)
	// );

	// spreadsheet.print();

	// TC

	println!("SpreadSheet Created");

	println!("Setting A(0,1) = 2");
	spreadsheet.set_cell_equation(0,1,
		Equation::new(
			Coordinate(0,1),
			Some(Type::ADD),
			Some(vec![
				SharedOperand::new(Operand::new(Some((0,1)),Some(2))),
				SharedOperand::new(Operand::new(Some((0,1)),Some(0)))
			])
		)
	);
	spreadsheet.print(); 

	println!("Setting B(0,2) = 1 + A(0,1)");
	spreadsheet.set_cell_equation(0,2,
		Equation::new(
			Coordinate(0,2),
			Some(Type::ADD),
			Some(vec![
				SharedOperand::new(Operand::new(Some((0,2)),Some(1))),
				spreadsheet.cells[0][1].clone()
			])
		)
	);
	spreadsheet.print();

	println!("Setting C(0,3) = A(0,1) + B(0,2)");
	spreadsheet.set_cell_equation(0,3,
		Equation::new(
			Coordinate(0,3),
			Some(Type::ADD),
			Some(vec![
				spreadsheet.cells[0][1].clone(),
				spreadsheet.cells[0][2].clone()
			])
		)
	);
	spreadsheet.print();
    
    println!("Setting A(0,1) = 3");
	spreadsheet.set_cell_equation(0,1,
		Equation::new(
			Coordinate(0,1),
			Some(Type::ADD),
			Some(vec![
				SharedOperand::new(Operand::new(Some((0,1)),Some(3))),
				SharedOperand::new(Operand::new(Some((0,1)),Some(0)))
			])
		)
	);
	spreadsheet.print();

	println!("Setting C(0,3) = A(0,1) - B(0,2)");
	spreadsheet.set_cell_equation(0,3,
		Equation::new(
			Coordinate(0,3),
			Some(Type::SUB),
			Some(vec![
				spreadsheet.cells[0][1].clone(),
				spreadsheet.cells[0][2].clone()
			])
		)
	);
	spreadsheet.print();

	println!("Setting B(0,2) = C(0,3)");
	spreadsheet.set_cell_equation(0,2,
		Equation::new(
			Coordinate(0,2),
			Some(Type::ADD),
			Some(vec![
				spreadsheet.cells[0][3].clone(),
				SharedOperand::new(Operand::new(Some((0,2)),Some(0)))
			])
		)
	);
	spreadsheet.print();

	println!("Setting D(0,4) = C(0,3)");
	spreadsheet.set_cell_equation(0,4,
		Equation::new(
			Coordinate(0,4),
			Some(Type::ADD),
			Some(vec![
				spreadsheet.cells[0][3].clone(),
				SharedOperand::new(Operand::new(Some((0,4)),Some(0)))
			])
		)
	);
	spreadsheet.print();

	println!("Setting E(0,5) = MIN[A(0,1),C(0,3)]");
	spreadsheet.set_cell_equation(0,5,
		Equation::new(
			Coordinate(0,5),
			Some(Type::MIN),
			Some(vec![
				spreadsheet.cells[0][3].clone(),
				spreadsheet.cells[0][1].clone()
			])
		)
	);
	spreadsheet.print();
	println!("Setting F(0,6) = MAX[C(0,3),A(0,1)]");
	spreadsheet.set_cell_equation(0,6,
		Equation::new(
			Coordinate(0,6),
			Some(Type::MAX),
			Some(vec![
				spreadsheet.cells[0][3].clone(),
				spreadsheet.cells[0][1].clone()
			])
		)
	);
	spreadsheet.print();
	
	println!("Setting A(0,1) = 10");
	spreadsheet.set_cell_equation(0,1,
		Equation::new(
			Coordinate(0,1),
			Some(Type::ADD),
			Some(vec![
				SharedOperand::new(Operand::new(Some((0,1)),Some(10))),
				SharedOperand::new(Operand::new(Some((0,1)),Some(0)))
			])
		)
	);
	spreadsheet.print();

	println!("Setting G(0,7) = SUM(A(0,1):F(0,1))");
	spreadsheet.set_cell_equation(0,7,
		Equation::new(
			Coordinate(0,7),
			Some(Type::SUM),
			Some(vec![
				spreadsheet.cells[0][1].clone(),
				spreadsheet.cells[0][6].clone()
			])
		)
	);
	spreadsheet.print();

	println!("Setting F(0,8) = AVG(A(0,1):F(0,1))");
	spreadsheet.set_cell_equation(0,8,
		Equation::new(
			Coordinate(0,8),
			Some(Type::AVG),
			Some(vec![
				spreadsheet.cells[0][1].clone(),
				spreadsheet.cells[0][6].clone()
			])
		)
	);
	spreadsheet.print();

	println!("Setting G(0,9) = DEV(A(0,1):F(0,1))");
	spreadsheet.set_cell_equation(0,9,
		Equation::new(
			Coordinate(0,9),
			Some(Type::DEV),
			Some(vec![
				spreadsheet.cells[0][1].clone(),
				spreadsheet.cells[0][6].clone()
			])
		)
	);
	spreadsheet.print();

	println!("Setting D(0,4) = 10");
	spreadsheet.set_cell_equation(0,4,
		Equation::new(
			Coordinate(0,4),
			Some(Type::ADD),
			Some(vec![
				SharedOperand::new(Operand::new(Some((0,1)),Some(10))),
				SharedOperand::new(Operand::new(Some((0,1)),Some(0)))
			])
		)
	);
	spreadsheet.print();
	// (100+121+1+1+1+100)/6 - (28/6)^2 = 32.2, sqrt = 5.67, should display 5


}

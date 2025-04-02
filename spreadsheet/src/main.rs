mod utils {
	#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
	pub struct Coordinate (pub usize, pub usize);
	pub enum Type {
		NUL,
		ADD, 
		SUB, 
		MUL, 
		DIV
	}
}

mod cell {
	use std::collections::HashSet;

	use super::utils::Coordinate;
	use super::equation::Equation;
	struct Cell<T> {
		coordinate: Coordinate, 
		value: T,
		equation: Equation<T>,
		downstream_neighbors: HashSet<Operand<T>>
	}
	
	impl<T: From<i32>> Cell<T> {
		fn new_from_coord(coord: Coordinate) -> Self {
			Cell {
				coordinate: coord,
				value: T::from(0), 
				equation: Equation::new(coord),
				downstream_neighbors: HashSet::<Operand<T>>::new()
			}
		}
		fn new(row: usize, col: usize) -> Self {
			Self::new_from_coord(Coordinate(row,col))
		}
	}

	struct Value<T> {
		coordinate: Coordinate,
		value: T
	} 
	
	impl<T: From<i32>> Value<T> {
		fn new_helper(coord: Coordinate, val: T) -> Self {
			Value {
				coordinate:coord, 
				value: val
			}
		}
		fn new(row:usize, col:usize, val:T) -> Self {
			Self::new_helper(Coordinate(row,col),val)
		}
	}

	pub enum Operand<T> {
		Cell(Box<Cell<T>>), 
		Value(Value<T>)
	}

	impl<T: From<i32>> Operand<T> {
		pub fn new_cell(row: usize, col: usize) -> Self {
			Operand::Cell(Box::new(Cell::new(row,col)))
		}

		pub fn new_value(row: usize, col: usize, val: T) -> Self {
			Operand::Value(Value::new(row,col,val))
		}

		pub fn get_value(&self) -> &T {
			match self {
				Operand::Cell(cell) => &cell.value,
				Operand::Value(val) => &val.value
			}
		}
		pub fn get_coordinate(&self) -> &Coordinate {
			match self {
				Operand::Cell(cell) => &cell.coordinate,
				Operand::Value(val) => &val.coordinate
			}
		}

		pub fn set_value(&mut self, val: T) {
			match self {
				Operand::Cell(cell) => cell.value = val,
				Operand::Value(value) => value.value = val
			}
		}
	}
	
}

mod equation {
	use super::utils::{Coordinate, Type};
	use super::cell::Operand;
	pub struct Equation<T> {
		coordinate: Coordinate, 
		t: Type, 
		operands: Vec<Operand<T>>
	}
	
	impl<T: From<i32>> Equation<T> {
		pub fn new(coord: Coordinate) -> Self {
			Equation {
				coordinate: coord, 
				t: Type::NUL, 
				operands: Vec::<Operand<T>>::new()
			}
		}
	
		fn process_cell_equation(t: Type, operands: Vec<Operand<T>>) -> T {
			T::from(0)
		}
	}

}

mod spreadsheet {

	use super::utils::Type;
	use super::cell::Operand;
	pub struct SpreadSheet<T> {
		m: usize, 
		n: usize, 
		cells: Vec<Vec<Operand<T>>> 
	}	

	impl<T: Copy + From<i32>> SpreadSheet<T> {	
		pub fn new(m: usize, n: usize) -> Self {
			let mut cells = Vec::<Vec::<Operand<T>>>::with_capacity(m);
	
			for i in 0..m {
				let mut row = Vec::<Operand<T>>::with_capacity(n);
				for j in 0..n {
					row.push(Operand::new_value(i, j, T::from(0)));
				}
				cells.push(row);
			}
	
			SpreadSheet { m, n, cells }
		}
		pub fn get_cell_value(&self, row:usize, col:usize) -> &T{
			Operand::get_value(&self.cells[row][col])
		}
	
		pub fn set_cell_value(&mut self, row:usize, col:usize, val: T) {
			// remove earlier equation
			Operand::set_value(&mut self.cells[row][col], val);
		}
	
		pub fn set_cell_equation(
			&mut self, row:usize, col:usize, t:Type, operands:Vec<Operand<T>>
		) {
			// remove earlier equation
			// put current equation
		}
	} 
}



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

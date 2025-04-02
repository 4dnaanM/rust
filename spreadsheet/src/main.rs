use std::collections::HashSet;
use std::vec::Vec; 

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Coordinate (usize, usize);

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

enum Type {
	NUL,
	ADD, 
	SUB, 
	MUL, 
	DIV
}

struct Equation<T> {
	coordinate: Coordinate, 
	t: Type, 
	operands: Vec<Operand<T>>
}

impl<T: From<i32>> Equation<T> {
	fn new(coord: Coordinate) -> Self {
		Equation {
			coordinate: coord, 
			t: Type::NUL, 
			operands: Vec::<Operand<T>>::new()
		}
	}

	fn process_cell_equation(t: Type, operands:Vec<Operand<T>>) -> T{
		T::from(0)
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

enum Operand<T> {
	Cell(Box<Cell<T>>), 
	Value(Value<T>)
}

struct SpreadSheet<T> {
	m: usize, 
	n: usize, 
	cells: Vec<Vec<Operand<T>>> 
}

impl<T: Copy + From<i32>> SpreadSheet<T> {	
	fn new(m: usize, n: usize) -> Self {
        let mut cells = Vec::<Vec::<Operand<T>>>::with_capacity(m);

        for i in 0..m {
            let mut row = Vec::<Operand<T>>::with_capacity(n);
            for j in 0..n {
				row.push(Operand::Value(Value::new(i,j,T::from(0))));
            }
            cells.push(row);
        }

        SpreadSheet { m, n, cells }
    }
	fn get_cell_value(&self, row:usize, col:usize) -> T{
		match &self.cells[row][col] {
			Operand::Cell(cell) => cell.value, 
			Operand::Value(val) => val.value
		}
	}

	fn set_cell_value(&mut self, row:usize, col:usize, val: T) {
		// remove earlier equation
		self.cells[row][col] = Operand::Value(
			Value{coordinate:Coordinate(row,col),value:val}
		);
	}

	fn set_cell_equation(
		&mut self, row:usize, col:usize, t:Type, operands:Vec<Operand<T>>
	) {
		// remove earlier equation
		// put current equation
	}
} 


fn main() {
	let mut spreadsheet = SpreadSheet::<i32>::new(20000,1000);

	for row in 0..20000{
		for col in 0..1000{
			spreadsheet.set_cell_value(row,col,1);
		}
	}
}

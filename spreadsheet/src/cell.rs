use std::collections::HashSet;

use crate::utils::Coordinate;
use crate::equation::Equation;
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
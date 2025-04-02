use std::collections::HashSet;

use crate::utils::Coordinate;
use crate::equation::Equation;

// cell and value are not meant to be public
// callers should work only with operand

struct Cell<T> {
    coordinate: Coordinate, 
    value: T,
    equation: Equation<T>,
    downstream_neighbors: HashSet<Operand<T>>
}

struct Value<T> {
    coordinate: Coordinate,
    value: T
} 

pub enum Operand<T> {
    Cell(Box<Cell<T>>), 
    Value(Value<T>)
}

impl<T: From<i32>> Cell<T> {
    fn new<U: Into<Coordinate>>(input: U) -> Self {
        let coordinate = input.into();
        Cell {
            coordinate,
            value: T::from(0),
            equation: Equation::new(coordinate,None, None),
            downstream_neighbors: HashSet::new(),
        }
    }
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

impl<T: From<i32>> Operand<T> {
    pub fn new(row: usize, col: usize, val: Option<T>) -> Self {
        match val {
            Some(v) => Operand::Value(Value::new(row,col,v)),
            None => Operand::Cell(Box::new(Cell::new((row,col))))
        }
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
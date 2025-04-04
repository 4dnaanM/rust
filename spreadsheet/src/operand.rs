use std::collections::HashSet;

use crate::utils::Coordinate;
use crate::equation::Equation;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

// cell and value are not meant to be public
// callers should work only with operand

struct Cell<'a,T> {
    coordinate: Coordinate, 
    value: T,
    equation: Equation<'a,T>,
    downstream_neighbors: HashSet<&'a Cell<'a,T>>
}

struct Value<T> {
    coordinate: Coordinate,
    value: T
} 

pub enum Operand<'a,T> {
    // it should own the cell or value
    Cell(Cell<'a,T>), 
    Value(Value<T>)
}

impl<'a,T: Clone + Copy + From<i32> + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T>> Cell<'a,T> {
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



impl<T: Clone + Copy + From<i32> + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T>> Value<T> {
    fn new<U: Into<Coordinate>>(input: U, val: T) -> Self {
        let coordinate = input.into();
        Value {
            coordinate:coordinate, 
            value: val
        }
    }
}

impl<'a,T: Clone + Copy + From<i32> + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T>> Operand<'a,T> {
    pub fn new(row: usize, col: usize, val: Option<T>, eq: Option<Equation<'a,T>>) -> Self {
        match eq {
            Some(eq) => {
                let mut c = Operand::Cell(Cell::new((row,col)));
                c.set_equation(eq);
                c
            },
            None => {
                match val {
                    Some(v) => Operand::Value(Value::new((row,col),v)),
                    None => Operand::Cell(Cell::new((row,col)))
                }
            }
        }
        
    }
    
    pub fn get_value(&self) -> T {
        match self {
            Operand::Cell(cell) => cell.value,
            Operand::Value(val) => val.value
        }
    }

    pub fn get_equation(&self) -> &Equation<T> {
        match self {
            Operand::Cell(cell) => &cell.equation,
            Operand::Value(_) => panic!("Value does not have an equation")
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

    pub fn set_equation(&mut self, eq: Equation<'a,T>){
        match self {
            Operand::Cell(cell) => {
                cell.value = eq.process_equation();
                cell.equation = eq;
            },
            Operand::Value(_) => panic!("Value can't have an equation!")
        }
    }

    pub fn is_cell(&mut self) -> bool {
        match self {
            Operand::Cell(_) => true,
            Operand::Value(_) => false
        }
    }
}
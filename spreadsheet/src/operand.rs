use crate::utils::Coordinate;
use crate::equation::Equation;

use std::hash::{Hash, Hasher};

use std::cell::RefCell;
use std::rc::Rc;

use std::ops::{Add,Sub,Mul,Div};

// cell and value are not meant to be public
// callers should work only with sharedoperand and operand

#[derive(Eq, PartialEq, Clone)]
struct Cell<T> {
    pub coordinate: Coordinate,
    pub value: T,
    
    // each cell owns its equation.
    pub equation: Box<Equation<T>>,
    // The equation contains references to other cells in its operands list 
    // When the cell is dropped, the equation is dropped too
    
    // references to other cells that depend on this cell
    pub downstream_neighbors: RefCell<Vec<SharedOperand<T>>>, // Can i use hashset here?
    // when the cell is dropped, each reference in downstream neighbors is dropped, decreasing the ref count
}
impl<T> Hash for Cell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinate.hash(state);
    }
}
impl<'a,T: Clone + Copy + From<i32> + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T>> Cell<T> {
    fn new<U: Into<Coordinate>>(input: U) -> Self {
        let coordinate = input.into();
        Cell {
            coordinate,
            value: T::from(0),
            equation: Box::new(Equation::new(coordinate, None, None)),
            downstream_neighbors: RefCell::new(Vec::new()),
        }
    }

    fn set_equation(&mut self, eq: Equation<T>) {
        
        // remove the old equation's links
        for operand in self.equation.get_operands() {
            if let Operand::Cell(ref neighbor) = *operand.borrow() {
                let mut neighbors = neighbor.downstream_neighbors.borrow_mut();
                neighbors.retain(|x| *x.borrow().get_coordinate() != self.coordinate);
            }
        }

        self.value = eq.process_equation();
        
        // I think the earlier equations should be deleted automatically when we set a new one
        self.equation = Box::new(eq);
        for operand in self.equation.get_operands() {
            if let Operand::Cell(ref neighbor) = *operand.borrow() {
                neighbor.downstream_neighbors.borrow_mut().push(Rc::new(RefCell::new(Operand::Cell(self.clone()))));
            }
        }
    }
}



#[derive(Eq, PartialEq, Clone)]
struct Value<T> {
    coordinate: Coordinate,
    value: T
} 
impl<T> Hash for Value<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinate.hash(state);
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



#[derive(Eq, PartialEq, Clone)]
pub enum Operand<T> {
    // it should own the cell or value
    Cell(Cell<T>), 
    Value(Value<T>)
}
impl<T> Hash for Operand<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Operand::Cell(cell) => cell.hash(state),
            Operand::Value(value) => value.hash(state)
        }
    }
}

impl<'a,T: Clone + Copy + From<i32> + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T>> Operand<T> {
    
    pub fn new<U : Into<Coordinate>>(input: U, val: Option<T>, eq: Option<Equation<T>>) -> Self {
        let coord = input.into();
        let (row,col) = (coord.0, coord.1);
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

    pub fn get_equation(&self) -> Equation<T> {
        match self {
            Operand::Cell(cell) => (*cell.equation).clone(),
            Operand::Value(_) => panic!("Value does not have an equation!")
        }
    }

    pub fn get_downstream_neighbors(&self) -> RefCell<Vec<SharedOperand<T>>> {
        match self {
            Operand::Cell(cell) => cell.downstream_neighbors.clone(),
            Operand::Value(_) => panic!("Value does not have downstream neighbors!")
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

    pub fn set_equation(&mut self, eq: Equation<T>){
        match self {
            Operand::Cell(cell) => {
                cell.set_equation(eq);
            },
            Operand::Value(_) => panic!("Value can't have an equation!")
        }
    }

    pub fn is_cell(&self) -> bool {
        match self {
            Operand::Cell(_) => true,
            Operand::Value(_) => false
        }
    }
}

pub type SharedOperand<T> = Rc<RefCell<Operand<T>>>;
// References to Operands that can be shared and also mutated
// Solely to prevent duplication
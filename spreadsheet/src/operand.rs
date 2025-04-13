use crate::utils::Coordinate;
use crate::equation::Equation;

use std::hash::{Hash, Hasher};

use std::cell::RefCell;
use std::rc::Rc;


// cell and value are not meant to be public
// callers should work only with sharedoperand and operand

#[derive(Eq, PartialEq, Clone)]
struct Cell {
    pub coordinate: Coordinate,
    pub value: i32,
    
    // each cell owns its equation.
    pub equation: Box<Equation>,
    // The equation contains references to other cells in its operands list 
    // When the cell is dropped, the equation is dropped too
    
    // references to other cells that depend on this cell
    pub downstream_neighbors: RefCell<Vec<SharedOperand>>, // Can i use hashset here?
    // when the cell is dropped, each reference in downstream neighbors is dropped, decreasing the ref count
}
impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinate.hash(state);
    }
}
impl Cell {
    fn new<U: Into<Coordinate>>(input: U) -> Self {
        let coordinate = input.into();
        Cell {
            coordinate,
            value: 0,
            equation: Box::new(Equation::new(coordinate, None, None)),
            downstream_neighbors: RefCell::new(Vec::new()),
        }
    }

    fn set_equation(&mut self, eq: Equation, self_ref: SharedOperand) {
        // println!("Cell: set_equation: ");
        // eq.print();
        // println!();
    
        let old_operands = self.equation.get_operands().clone();
    
        for operand in old_operands {
            if let Operand::Cell(ref neighbor) = *operand.borrow() {
                let mut neighbors = neighbor.downstream_neighbors.borrow_mut();
                neighbors.retain(|x| !Rc::ptr_eq(x, &self_ref));
            }
        }
    
        self.value = eq.process_equation();
    
        self.equation = Box::new(eq);
    
        let new_operands = self.equation.get_operands().clone();
        for operand in new_operands {
            if let Operand::Cell(ref neighbor) = *operand.borrow() {
                neighbor.downstream_neighbors.borrow_mut().push(self_ref.clone());
            }
        }
    
    }
    
    fn print (&self) {
        print!("C({},{})->{}, ",self.coordinate.0,self.coordinate.1,self.value);
        self.equation.print();
        print!(", Downstream Neighbors: [");
        for neighbor in self.downstream_neighbors.borrow().iter() {
            let neighbor = neighbor.borrow();
            let coord = neighbor.get_coordinate();
            print!("({},{}),",coord.0,coord.1);
        }
        println!("]");
    }
}



#[derive(Eq, PartialEq, Clone, Hash)]
struct Value {
    value: i32
} 
impl Value {
    fn new(val: i32) -> Self {
        Value {
            value: val
        }
    }
    fn print (&self) {
        println!("V->{}",self.value);
    }
}



#[derive(Eq, PartialEq, Clone)]
pub enum Operand {
    // it should own the cell or value
    Cell(Cell), 
    Value(Value)
}
impl Hash for Operand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Operand::Cell(cell) => cell.hash(state),
            Operand::Value(value) => value.hash(state)
        }
    }
}

impl Operand {
    
    pub fn new<U : Into<Coordinate>>(input: Option<U>, val: Option<i32>) -> Self {
        match val {
            Some(v) => Operand::Value(Value::new(v)),
            None => {
                let coord = input.unwrap().into();
                let (row,col) = (coord.0, coord.1);
                Operand::Cell(Cell::new((row,col)))
            }
        }
    }
        
    pub fn print (&self) {
        match self {
            Operand::Cell(cell) => cell.print(),
            Operand::Value(value) => value.print()
        }
    }
    
    pub fn get_value(&self) -> i32 {
        match self {
            Operand::Cell(cell) => cell.value,
            Operand::Value(val) => val.value
        }
    }

    pub fn get_equation(&self) -> Equation {
        match self {
            Operand::Cell(cell) => (*cell.equation).clone(),
            Operand::Value(_) => panic!("Value does not have an equation!")
        }
    }

    pub fn get_downstream_neighbors(&self) -> RefCell<Vec<SharedOperand>> {
        match self {
            Operand::Cell(cell) => cell.downstream_neighbors.clone(),
            Operand::Value(_) => panic!("Value does not have downstream neighbors!")
        }
    }
    
    pub fn get_coordinate(&self) -> &Coordinate {
        match self {
            Operand::Cell(cell) => &cell.coordinate,
            Operand::Value(_) => panic!("Value does not have a coordinate!")
        }
    }

    pub fn set_value(&mut self, val: i32) {
        match self {
            Operand::Cell(cell) => cell.value = val,
            Operand::Value(value) => value.value = val
        }
    }

    pub fn set_equation(&mut self, eq: Equation, self_ref: SharedOperand) {
        match self {
            Operand::Cell(cell) => {
                cell.set_equation(eq,self_ref);
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

pub type SharedOperand = Rc<RefCell<Operand>>;
// References to Operands that can be shared and also mutated
// Solely to prevent duplication
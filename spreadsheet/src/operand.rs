use crate::spreadsheet::SpreadSheet;
use crate::utils::Coordinate;
use crate::equation::Equation;
use crate::utils::Type;

use std::hash::{Hash, Hasher};
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
// use std::collections::HashSet;

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
    // pub downstream_neighbors: RefCell<HashSet<SharedOperand>>,
    pub downstream_neighbors: RefCell<Vec<SharedOperand>>,
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
            // downstream_neighbors: RefCell::new(HashSet::<SharedOperand>::new()),
            downstream_neighbors: RefCell::new(Vec::<SharedOperand>::new()),
        }
    }

    fn set_equation(&mut self, eq: Equation, self_ref: SharedOperand, spreadsheet_ref: &SpreadSheet) {
        // println!("Cell: set_equation: ");
        // eq.print();
        // println!();
    
        let old_operands = self.equation.get_operands().clone();
    
        for operand in old_operands {
            if let Operand::Cell(ref neighbor) = *operand.borrow() {
                let mut neighbors = neighbor.downstream_neighbors.borrow_mut();
                neighbors.retain(|x| !Rc::ptr_eq(&x.0, &self_ref.0));
                // neighbors.remove(&self_ref);
            }
        }
    
        self.value = eq.process_equation(spreadsheet_ref);
    
        self.equation = Box::new(eq);
    
        let new_operands = self.equation.get_operands().clone();

        if self.equation.t==Type::SUM || self.equation.t == Type::AVG || self.equation.t == Type::DEV || self.equation.t == Type::MIN || self.equation.t == Type::MAX {
            
            let y1 = new_operands[0].borrow().get_coordinate().0;
            let x1 = new_operands[0].borrow().get_coordinate().1;
            let y2 = new_operands[1].borrow().get_coordinate().0;
            let x2 = new_operands[1].borrow().get_coordinate().1;

            for y in y1..=y2 {
                for x in x1..=x2 {
                    if let Operand::Cell(neighbor) = spreadsheet_ref.cells[y][x].borrow().clone() {
                        // print!("SKJHDLKASJ");
                        neighbor.downstream_neighbors.borrow_mut().push(self_ref.clone());
                        println!("added to downstream of ({},{})",neighbor.coordinate.0, neighbor.coordinate.1);
                    }
                }
            }
        }

        else{
            for operand in new_operands {
                if let Operand::Cell(ref neighbor) = *operand.borrow() {
                    // neighbor.downstream_neighbors.borrow_mut().insert(self_ref.clone());
                    neighbor.downstream_neighbors.borrow_mut().push(self_ref.clone());
                    print!("Added to downstream neighbors of: ({},{})",neighbor.coordinate.0, neighbor.coordinate.1);
                }
            }
        }
        // self.print(); 
    
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
    coordinate: Coordinate,
    value: i32,
} 
impl Value {
    fn new(coordinate: Coordinate, val: i32) -> Self {
        Value {
            value: val,
            coordinate: coordinate
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
            Some(v) => Operand::Value(Value::new(input.unwrap().into(),v)),
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

    // pub fn get_downstream_neighbors(&self) -> RefCell<HashSet<SharedOperand>> {
    pub fn get_downstream_neighbors(&self) -> RefCell<Vec<SharedOperand>> {
        match self {
            Operand::Cell(cell) => cell.downstream_neighbors.clone(),
            Operand::Value(_) => panic!("Value does not have downstream neighbors!")
        }
    }
    
    pub fn get_coordinate(&self) -> &Coordinate {
        match self {
            Operand::Cell(cell) => &cell.coordinate,
            Operand::Value(v) => &v.coordinate
        }
    }

    pub fn set_value(&mut self, val: i32) {
        match self {
            Operand::Cell(cell) => cell.value = val,
            Operand::Value(value) => value.value = val
        }
    }

    pub fn set_equation(&mut self, eq: Equation, self_ref: SharedOperand, spreadsheet_ref: &SpreadSheet) {
        match self {
            Operand::Cell(cell) => {
                cell.set_equation(eq,self_ref,spreadsheet_ref);
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


#[derive(Eq, PartialEq, Clone)]
pub struct SharedOperand(pub Rc<RefCell<Operand>>);
impl Hash for SharedOperand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = Rc::as_ptr(&self.0);
        ptr.hash(state);
    }
}

impl SharedOperand {
    pub fn new(op: Operand) -> Self {
        SharedOperand(Rc::new(RefCell::new(op)))
    }
    pub fn borrow(&self) -> Ref<Operand> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> RefMut<Operand> {
        self.0.borrow_mut()
    }
    pub fn clone(&self) -> SharedOperand {
        SharedOperand(self.0.clone())
    }
}

// References to Operands that can be shared and also mutated
// Solely to prevent duplication
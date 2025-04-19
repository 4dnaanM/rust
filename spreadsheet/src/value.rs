use crate::equation::Equation;
use crate::spreadsheet::SpreadSheet;
use crate::utils::Coordinate;
use crate::utils::Type;

use std::cell::{Ref, RefCell, RefMut};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
// use std::collections::HashSet;

// cell and value are not meant to be public
// callers should work only with sharedoperand and operand

#[derive(Eq, PartialEq, Clone)]
struct Cell {
    pub coordinate: Coordinate,
    pub value: Option<i32>,

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
            value: Some(0),
            equation: Box::new(Equation::new(coordinate, None, None)),
            // downstream_neighbors: RefCell::new(HashSet::<SharedOperand>::new()),
            downstream_neighbors: RefCell::new(Vec::<SharedOperand>::new()),
        }
    }

    fn set_equation(
        &mut self,
        eq: Equation,
        self_ref: SharedOperand,
        spreadsheet_ref: &SpreadSheet,
    ) {
        // println!("Cell: set_equation: ");
        // eq.print();
        // println!();

        let old_operands = self.equation.get_operands().clone();

        for operand in old_operands {
            if let Value::Cell(ref neighbor) = *operand.borrow() {
                let mut neighbors = neighbor.downstream_neighbors.borrow_mut();
                neighbors.retain(|x| !Rc::ptr_eq(&x.0, &self_ref.0));
            }
        }

        self.value = eq.process_equation_silent(spreadsheet_ref).0;

        self.equation = Box::new(eq);

        let new_operands = self.equation.get_operands().clone();

        if self.equation.t == Type::SUM
            || self.equation.t == Type::AVG
            || self.equation.t == Type::DEV
            || self.equation.t == Type::MIN
            || self.equation.t == Type::MAX
        {
            let y1 = new_operands[0].borrow().get_coordinate().0;
            let x1 = new_operands[0].borrow().get_coordinate().1;
            let y2 = new_operands[1].borrow().get_coordinate().0;
            let x2 = new_operands[1].borrow().get_coordinate().1;

            for y in y1..=y2 {
                for x in x1..=x2 {
                    let current = &spreadsheet_ref.cells[y][x];
                    let neighbor = current.borrow();
                    if let Value::Cell(ref cell) = *neighbor {
                        cell.downstream_neighbors
                            .borrow_mut()
                            .push(self_ref.clone());
                    }
                }
            }
        } else {
            for operand in new_operands {
                if let Value::Cell(ref neighbor) = *operand.borrow() {
                    neighbor
                        .downstream_neighbors
                        .borrow_mut()
                        .push(self_ref.clone());
                    // print!("Added to downstream neighbors of: ({},{})", neighbor.coordinate.0, neighbor.coordinate.1);
                }
            }
        }
        // self.print();
    }

    // fn print (&self) {
    //     print!("C({},{})->{}, ",self.coordinate.0,self.coordinate.1,self.value);
    //     self.equation.print();
    //     // print!(", Downstream Neighbors: [");
    //     for neighbor in self.downstream_neighbors.borrow().iter() {
    //         let neighbor = neighbor.borrow();
    //         let coord = neighbor.get_coordinate();
    //         print!("({},{}),",coord.0,coord.1);
    //     }
    //     println!("]");
    // }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct Constant {
    // coordinate: Coordinate,
    value: i32,
}
impl Constant {
    fn new(val: i32) -> Self {
        Constant { value: val }
    }
    // fn print (&self) {
    //     println!("V->{}",self.value);
    // }
}

#[derive(Eq, PartialEq, Clone)]
pub enum Value {
    // it should own the cell or const
    Cell(Cell),
    Constant(Constant),
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Cell(cell) => cell.hash(state),
            Value::Constant(value) => value.hash(state),
        }
    }
}

impl Value {
    pub fn new<U: Into<Coordinate>>(input: Option<U>, val: Option<i32>) -> Self {
        match input {
            None => Value::Constant(Constant::new(val.unwrap_or(0))),
            Some(i) => {
                let coord = i.into();
                let (row, col) = (coord.0, coord.1);
                let mut ans = Value::Cell(Cell::new((row, col)));
                if let Some(v) = val { ans.set_value(Some(v)) }
                ans
            }
        }
    }

    // pub fn print (&self) {
    //     match self {
    //         Value::Cell(cell) => cell.print(),
    //         Value::Value(value) => value.print()
    //     }
    // }

    pub fn get_value(&self) -> Option<i32> {
        match self {
            Value::Cell(cell) => cell.value,
            Value::Constant(val) => Some(val.value),
        }
    }

    pub fn get_equation(&self) -> Equation {
        match self {
            Value::Cell(cell) => (*cell.equation).clone(),
            Value::Constant(_) => panic!("Value does not have an equation!"),
        }
    }

    pub fn get_downstream_neighbors(&self) -> RefCell<Vec<SharedOperand>> {
        match self {
            Value::Cell(cell) => cell.downstream_neighbors.clone(),
            Value::Constant(_) => panic!("Value does not have downstream neighbors!"),
        }
    }

    pub fn get_coordinate(&self) -> &Coordinate {
        match self {
            Value::Cell(cell) => &cell.coordinate,
            Value::Constant(_) => panic!("Value does not have a coordinate!"),
        }
    }

    pub fn set_value(&mut self, val: Option<i32>) {
        match self {
            Value::Cell(cell) => cell.value = val,
            Value::Constant(value) => value.value = val.unwrap_or(0),
        }
    }

    pub fn set_equation(
        &mut self,
        eq: Equation,
        self_ref: SharedOperand,
        spreadsheet_ref: &SpreadSheet,
    ) {
        match self {
            Value::Cell(cell) => {
                cell.set_equation(eq, self_ref, spreadsheet_ref);
            }
            Value::Constant(_) => panic!("Value can't have an equation!"),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct SharedOperand(pub Rc<RefCell<Value>>);
impl Hash for SharedOperand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = Rc::as_ptr(&self.0);
        ptr.hash(state);
    }
}

// References to Operands that can be shared and also mutated
// Solely to prevent duplication
impl SharedOperand {
    pub fn new(op: Value) -> Self {
        SharedOperand(Rc::new(RefCell::new(op)))
    }
    pub fn borrow(&self) -> Ref<Value> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> RefMut<Value> {
        self.0.borrow_mut()
    }
    pub fn clone(&self) -> SharedOperand {
        SharedOperand(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_creation() {
        let constant = Constant::new(42);
        assert_eq!(constant.value, 42);
    }

    #[test]
    fn test_cell_creation() {
        let coord = Coordinate(1, 1);
        let cell = Cell::new(coord);
        assert_eq!(cell.coordinate, coord);
        assert_eq!(cell.value, Some(0));
    }

    #[test]
    fn test_value_creation_constant() {
        let value = Value::new(None::<Coordinate>, Some(10));
        assert_eq!(value.get_value(), Some(10));
    }

    #[test]
    fn test_value_creation_cell() {
        let coord = Coordinate(2, 3);
        let value = Value::new(Some(coord), Some(5));
        assert_eq!(value.get_value(), Some(5));
        assert_eq!(value.get_coordinate(), &coord);
    }

    #[test]
    fn test_set_value() {
        let mut value = Value::new(None::<Coordinate>, Some(10));
        value.set_value(Some(20));
        assert_eq!(value.get_value(), Some(20));
    }

    #[test]
    fn test_set_equation() {
        let coord = Coordinate(0, 0);
        let mut cell = Cell::new(coord);
        let equation = Equation::new(coord, None, None);
        let spreadsheet = SpreadSheet::new(3, 3); // Assuming SpreadSheet has a new method
        let shared_operand = SharedOperand::new(Value::Cell(cell.clone()));

        cell.set_equation(equation, shared_operand, &spreadsheet);
        assert!(cell.equation.get_operands().is_empty());
    }

    #[test]
    fn test_shared_operand() {
        let value = Value::new(None::<Coordinate>, Some(15));
        let shared_operand = SharedOperand::new(value);
        assert_eq!(shared_operand.borrow().get_value(), Some(15));
    }

    #[test]
    fn test_downstream_neighbors() {
        let coord1 = Coordinate(1, 1);
        let coord2 = Coordinate(2, 2);
        let cell1 = Cell::new(coord1);
        let cell2 = Cell::new(coord2);
        let shared_operand = SharedOperand::new(Value::Cell(cell2));

        cell1
            .downstream_neighbors
            .borrow_mut()
            .push(shared_operand.clone());
        assert_eq!(cell1.downstream_neighbors.borrow().len(), 1);
    }
}

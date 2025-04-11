use crate::equation::Equation;
use crate::operand::{SharedOperand,Operand};

use std::ops::{Add,Sub,Mul,Div};
use std::cell::RefCell;
use std::rc::Rc;

// should expose set and get for cell value, and set for cell equation
// all the traversal and updation methods should be defined here like findDownstream, toposort 
pub struct SpreadSheet<T> {
    m: usize, 
    n: usize, 
    cells: Vec<Vec<SharedOperand<T>>> 
    
}	

impl<'a,T: Clone + Copy + From<i32> + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T>> SpreadSheet<T> {	
    pub fn new(m: usize, n: usize) -> Self {
        let mut cells = Vec::<Vec::<SharedOperand<T>>>::with_capacity(m);

        for i in 0..m {
            let mut row = Vec::<SharedOperand<T>>::with_capacity(n);
            for j in 0..n {
                row.push(Rc::new(RefCell::new(Operand::new(i, j, Some(T::from(0)),None))));
            }
            cells.push(row);
        }

        SpreadSheet { m, n, cells }
    }
    pub fn get_cell_value(&self, row:usize, col:usize) -> T{
        assert!(col < self.n && row < self.m,"get_cell_value: Invalid cell coordinates ({},{})", row, col);
        Operand::get_value(&(self.cells[row][col].borrow()))
    }

    pub fn set_cell_value(&mut self, row:usize, col:usize, val: T) {
        assert!(col < self.n && row < self.m,"set_cell_value: Invalid cell coordinates ({},{})", row, col);
        // remove earlier equation
        Operand::set_value(&mut (self.cells[row][col]).borrow_mut(), val);
    }

    fn process_cell_equation(&self, row:usize, col:usize) -> T{
        assert!(col < self.n && row < self.m,"process_cell_equation: Invalid cell coordinates ({},{})", row, col);
        Operand::get_equation(&(self.cells[row][col].borrow())).process_equation()
    }
    
    pub fn set_cell_equation(
        &mut self, row:usize, col:usize, eq: Equation<T>
    ) {
        assert!(col < self.n && row < self.m,"set_cell_equation: Invalid cell coordinates ({},{})", row, col);
        if !self.cells[row][col].borrow().is_cell() {
            self.cells[row][col] = Rc::new(RefCell::new(Operand::new(row, col, None, None)));
        }
        // remove earlier equation
        // put current equation
        
        self.cells[row][col].borrow_mut().set_equation(eq);
        
    }

    
}
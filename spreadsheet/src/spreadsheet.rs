use crate::equation::Equation;
use crate::utils::Type;
use crate::operand::Operand;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

// should expose set and get for cell value, and set for cell equation
// all the traversal and updation methods should be defined here like findDownstream, toposort 
pub struct SpreadSheet<'a,T> {
    m: usize, 
    n: usize, 
    cells: Vec<Vec<Operand<'a,T>>> 
}	

impl<'a,T: Clone + Copy + From<i32> + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T>> SpreadSheet<'a,T> {	
    pub fn new(m: usize, n: usize) -> Self {
        let mut cells = Vec::<Vec::<Operand<T>>>::with_capacity(m);

        for i in 0..m {
            let mut row = Vec::<Operand<T>>::with_capacity(n);
            for j in 0..n {
                row.push(Operand::new(i, j, Some(T::from(0)),None));
            }
            cells.push(row);
        }

        SpreadSheet { m, n, cells }
    }
    pub fn get_cell_value(&self, row:usize, col:usize) -> T{
        assert!(col < self.n && row < self.m,"get_cell_value: Invalid cell coordinates ({},{})", row, col);
        Operand::get_value(&self.cells[row][col])
    }

    pub fn set_cell_value(&mut self, row:usize, col:usize, val: T) {
        assert!(col < self.n && row < self.m,"set_cell_value: Invalid cell coordinates ({},{})", row, col);
        // remove earlier equation
        Operand::set_value(&mut self.cells[row][col], val);
    }

    fn process_cell_equation(&self, row:usize, col:usize) -> T{
        assert!(col < self.n && row < self.m,"process_cell_equation: Invalid cell coordinates ({},{})", row, col);
        Operand::get_equation(&self.cells[row][col]).process_equation()
    }
    
    pub fn set_cell_equation(
        &mut self, row:usize, col:usize, eq: Equation<'a,T>
    ) {
        assert!(col < self.n && row < self.m,"set_cell_equation: Invalid cell coordinates ({},{})", row, col);
        if !self.cells[row][col].is_cell() {
            self.cells[row][col] = Operand::new(row, col, None, None);
            self.cells[row][col].set_equation(eq);
            return;
        }

        let old_equation = self.cells[row][col].get_equation();
        // remove earlier equation
        // put current equation
    }

    
}
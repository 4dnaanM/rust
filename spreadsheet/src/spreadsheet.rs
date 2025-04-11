use crate::equation::Equation;
use crate::operand::{SharedOperand,Operand};

use std::ops::{Add,Sub,Mul,Div};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

// should expose set and get for cell value, and set for cell equation
// all the traversal and updation methods should be defined here like findDownstream, toposort 
pub struct SpreadSheet<T> {
    m: usize, 
    n: usize, 
    pub cells: Vec<Vec<SharedOperand<T>>> 
    
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

    // fn set_cell_value(&mut self, row:usize, col:usize, val: T) {
    //     assert!(col < self.n && row < self.m,"set_cell_value: Invalid cell coordinates ({},{})", row, col);
    //     // remove earlier equation
    //     Operand::set_value(&mut (self.cells[row][col]).borrow_mut(), val);
    //     self.do_operation(row, col);
    // }

    fn process_cell_equation(&self, row:usize, col:usize) -> T{
        assert!(col < self.n && row < self.m,"process_cell_equation: Invalid cell coordinates ({},{})", row, col);
        Operand::get_equation(&(self.cells[row][col].borrow())).process_equation()
    }

    fn get_indegrees(&self, row: usize, col: usize, set: &mut HashMap<(usize, usize),usize>) {
        let op = self.cells[row][col].borrow(); 
        *set.entry((row,col)).or_insert(0) += 1;
        for neighbor in op.get_downstream_neighbors().borrow().iter() {
            let neighbor_ref = neighbor.borrow();
            let coord = neighbor_ref.get_coordinate();
            let r = coord.0; 
            let c = coord.1;
            self.get_indegrees(r, c, set);
        }
    }

    fn toposort(&self, mut in_degrees: HashMap<(usize,usize),usize>) -> Vec<(usize,usize)>{
        let mut queue = Vec::new();
        let mut order = Vec::new();

        for ((row,col), indegree) in in_degrees.iter(){
            if *indegree == 0 {
                queue.push((*row,*col));
            }
        }

        while !queue.is_empty() {
            let (row, col) = queue.pop().unwrap();
            order.push((row,col));

            let op = self.cells[row][col].borrow();

            for neighbor in op.get_downstream_neighbors().borrow().iter() {
                let neighbor_ref = neighbor.borrow();
                let coord = neighbor_ref.get_coordinate();
                let r = coord.0; 
                let c = coord.1;
                *in_degrees.get_mut(&(r, c)).unwrap() -= 1;
                if in_degrees[&(r,c)] == 0 {
                    queue.push((r,c));
                }
            }
        }

        order
    }
    
    fn do_operation(&mut self, row: usize, col: usize){
        let mut in_degrees = HashMap::new();
        self.get_indegrees(row,col, &mut in_degrees);
        let order = self.toposort(in_degrees);
        for coord in order{
            self.cells[coord.0][coord.1].borrow_mut().set_value(self.process_cell_equation(coord.0, coord.1));
        }
    } 

    pub fn set_cell_equation(
        &mut self, row:usize, col:usize, eq: Equation<T>
    ) {
        assert!(col < self.n && row < self.m,"set_cell_equation: Invalid cell coordinates ({},{})", row, col);
        if !self.cells[row][col].borrow().is_cell() {
            self.cells[row][col] = Rc::new(RefCell::new(Operand::new(row, col, None, None)));
        }
        
        self.cells[row][col].borrow_mut().set_equation(eq);
        self.do_operation(row, col);

        // do operation on cell
        
    }

    
}
use crate::equation::Equation;
use crate::operand::{SharedOperand,Operand};
use crate::utils::{Type,Coordinate}; 

use std::collections::HashMap;

// should expose set and get for cell value, and set for cell equation
// all the traversal and updation methods should be defined here like findDownstream, toposort 
pub struct SpreadSheet {
    m: usize, 
    n: usize, 
    pub cells: Vec<Vec<SharedOperand>> 
    
}	

impl SpreadSheet {	
    pub fn new(m: usize, n: usize) -> Self {
        let mut cells = Vec::<Vec::<SharedOperand>>::with_capacity(m);

        for i in 0..m {
            let mut row = Vec::<SharedOperand>::with_capacity(n);
            for j in 0..n {
                row.push(SharedOperand::new(Operand::new(Some((i, j)), Some(0))));
            }
            cells.push(row);
        }

        SpreadSheet { m, n, cells }
    }
    pub fn get_cell_value(&self, row:usize, col:usize) -> i32{
        assert!(col < self.n && row < self.m,"get_cell_value: Invalid cell coordinates ({},{})", row, col);
        Operand::get_value(&(self.cells[row][col].borrow()))
    }

    fn process_cell_equation(&self, row:usize, col:usize) -> i32{
        assert!(col < self.n && row < self.m,"process_cell_equation: Invalid cell coordinates ({},{})", row, col);
        Operand::get_equation(&(self.cells[row][col].borrow())).process_equation(self)
    }

    fn get_indegrees(&self, row: usize, col: usize, set: &mut HashMap<(usize, usize),i32>) {
        if set.contains_key(&(row,col)) {
            set.entry((row,col)).and_modify(|e| *e += 1);
            return;
        }
        set.insert((row, col), if set.is_empty() { 0 } else { 1 });
        let op = self.cells[row][col].borrow(); 
        for neighbor in op.get_downstream_neighbors().borrow().iter() {
            let neighbor_ref = neighbor.borrow();
            let coord = neighbor_ref.get_coordinate();
            let r = coord.0; 
            let c = coord.1;
            self.get_indegrees(r, c, set);
        }
        // print!("get_indegrees: Set: {:?}, ",set);
        // op.print();
    }

    fn toposort(&self, mut in_degrees: HashMap<(usize,usize),i32>) -> Option<Vec<(usize,usize)>>{
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

        if in_degrees.len() != order.len() {
            return None; 
        }

        Some(order)
    }
    
    fn do_operation(&mut self, row: usize, col: usize) -> bool{
        // print!("do_operation: ");
        // self.cells[row][col].borrow().print();
        // get all the affected cells and indegrees
        let mut in_degrees = HashMap::new();
        self.get_indegrees(row,col, &mut in_degrees);
        // print!("do_operation: In degrees: {:?}\n", in_degrees);

        //use indegrees to find toposort
        let order = self.toposort(in_degrees);
        if order.is_none() {
            println!("do_operation: Cycle detected in the equation");
            return false;
        }
        let order = order.unwrap();

        // for each cell in the order, process the equation and set the value
        for coord in order{
            let val = self.process_cell_equation(coord.0, coord.1);
            self.cells[coord.0][coord.1].borrow_mut().set_value(val);
        }
        
        return true;
    } 

    // pub fn set_cell_equation(&mut self, row:usize, col:usize, c1: Option<(usize,usize)>, c2: Option<(usize,usize)>, v1: Option<i32>, v2: Option<i32>, t:Option<Type>) {
    //     assert!(col < self.n && row < self.m,"set_cell_equation: Invalid cell coordinates ({},{})", row, col);

    //     let op1 = match c1 {
    //         Some(c) => self.cells[c.0][c.1].clone(),
    //         None => SharedOperand::new(Operand::new(Some((row, col)), v1))
    //     };
    //     let op2 = match c2 {
    //         Some(c) => self.cells[c.0][c.1].clone(),
    //         None => SharedOperand::new(Operand::new(Some((row, col)), v2))
    //     };
    //     let ops = vec![op1, op2];

    //     let eq = Equation::new(Coordinate(row,col), t, Some(ops));
    //     self.set_cell_equation_from_eq(row, col, eq);   
    // }

    pub fn set_cell_equation(&mut self, row:usize, col:usize, eq: Equation) {

        // print!("New equation: ");
        // eq.print();
        // println!();
        
        if !(self.cells[row][col].borrow().is_cell()) {
            // print!("Convert to a cell before setting equation: ");
            // self.cells[row][col].borrow().print(); 
            self.cells[row][col] = SharedOperand::new(Operand::new(Some((row, col)), None));
        }
        // self.cells[row][col].borrow().print(); 

        // self.cells[row][col].borrow_mut().set_equation(eq);
        let cell_ref = self.cells[row][col].clone();
        let old_eq = cell_ref.borrow().get_equation();
        cell_ref.borrow_mut().set_equation(eq, cell_ref.clone(), self);

        if !self.do_operation(row, col) {
            {cell_ref.borrow_mut().set_equation(old_eq, cell_ref.clone(),self);}
            // println!("set_cell_equation: Failed to set equation due to cycle, reverting to old equation");
            // print!("Old equation: ");
            // cell_ref.borrow_mut().get_equation().print();
            // println!();
        };

        
    }

    pub fn print(&self) {
        println!("----------------------------------------------------------------------");
        for row in 0..self.m {
            for col in 0..self.n {
                print!("|{:6}", self.get_cell_value(row, col));
            }
            println!("|");
        }
        println!("----------------------------------------------------------------------");
    }
    
}
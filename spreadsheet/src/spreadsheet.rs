use crate::utils::Type;
use crate::operand::Operand;

// should expose set and get for cell value, and set for cell equation
// all the traversal and updation methods should be defined here like findDownstream, toposort 
pub struct SpreadSheet<T> {
    m: usize, 
    n: usize, 
    cells: Vec<Vec<Operand<T>>> 
}	

impl<T: Copy + From<i32>> SpreadSheet<T> {	
    pub fn new(m: usize, n: usize) -> Self {
        let mut cells = Vec::<Vec::<Operand<T>>>::with_capacity(m);

        for i in 0..m {
            let mut row = Vec::<Operand<T>>::with_capacity(n);
            for j in 0..n {
                row.push(Operand::new(i, j, Some(T::from(0))));
            }
            cells.push(row);
        }

        SpreadSheet { m, n, cells }
    }
    pub fn get_cell_value(&self, row:usize, col:usize) -> &T{
        Operand::get_value(&self.cells[row][col])
    }

    pub fn set_cell_value(&mut self, row:usize, col:usize, val: T) {
        // remove earlier equation
        Operand::set_value(&mut self.cells[row][col], val);
    }

    pub fn set_cell_equation(
        &mut self, row:usize, col:usize, t:Type, operands:Vec<Operand<T>>
    ) {
        // remove earlier equation
        // put current equation
    }
}
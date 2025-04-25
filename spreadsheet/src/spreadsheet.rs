use crate::equation::Equation;
use crate::utils::{Coordinate, Status, Type};
use crate::value::{SharedOperand, Value};

use std::collections::HashMap;

// should expose set and get for cell value, and set for cell equation
// all the traversal and updation methods should be defined here like findDownstream, toposort
#[derive(Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SpreadSheet {
    pub m: usize,
    pub n: usize,
    pub cells: Vec<Vec<SharedOperand>>,
}

#[derive(Debug, Clone)]
pub struct CellEquationParameters {
    pub _coordinates: (usize, usize),
    pub operand1_coordinates: Option<(usize, usize)>,
    pub operand2_coordinates: Option<(usize, usize)>,
    pub operand1_value: Option<i32>,
    pub operand2_value: Option<i32>,
    pub equation_type: Type,
}

impl SpreadSheet {
    pub fn new(m: usize, n: usize) -> Self {
        let mut cells = Vec::<Vec<SharedOperand>>::with_capacity(m);

        for i in 0..m {
            let mut row = Vec::<SharedOperand>::with_capacity(n);
            for j in 0..n {
                row.push(SharedOperand::new(Value::new(Some((i, j)), None)));
            }
            cells.push(row);
        }

        SpreadSheet { m, n, cells }
    }
    pub fn get_cell_value(&self, row: usize, col: usize) -> Option<i32> {
        assert!(
            col < self.n && row < self.m,
            "get_cell_value: Invalid cell coordinates ({},{})",
            row,
            col
        );
        Value::get_value(&(self.cells[row][col].borrow()))
    }

    pub fn get_cell_equation_parameters(&self, row: usize, col: usize) -> CellEquationParameters {
        assert!(
            col < self.n && row < self.m,
            "get_cell_value: Invalid cell coordinates ({},{})",
            row,
            col
        );

        let cell_ref = self.cells[row][col].borrow();

        let coords = cell_ref.get_coordinate();
        let tcoords = (coords.0, coords.1);
        let eq = cell_ref.get_equation();

        let ops = eq.get_operands();

        let op1 = if ops.is_empty() {
            Some(ops[0].borrow())
        } else {
            None
        };
        let op2 = if ops.len() > 1 {
            Some(ops[1].borrow())
        } else {
            None
        };

        let (v1, c1) = match op1 {
            Some(op) => (
                op.get_value(),
                if op.is_cell() {
                    Some((op.get_coordinate().0, op.get_coordinate().1))
                } else {
                    None
                },
            ),
            None => (None, None),
        };

        let (v2, c2) = match op2 {
            Some(op) => (
                op.get_value(),
                if op.is_cell() {
                    Some((op.get_coordinate().0, op.get_coordinate().1))
                } else {
                    None
                },
            ),
            None => (None, None),
        };

        let t = eq.t;
        CellEquationParameters {
            _coordinates: tcoords,
            operand1_coordinates: c1,
            operand2_coordinates: c2,
            operand1_value: v1,
            operand2_value: v2,
            equation_type: t,
        }
    }

    fn process_cell_equation(&self, row: usize, col: usize) -> Option<i32> {
        assert!(
            col < self.n && row < self.m,
            "process_cell_equation: Invalid cell coordinates ({},{})",
            row,
            col
        );
        Value::get_equation(&(self.cells[row][col].borrow())).process_equation(self)
    }

    fn get_indegrees(&self, row: usize, col: usize, set: &mut HashMap<(usize, usize), i32>) {
        if set.contains_key(&(row, col)) {
            set.entry((row, col)).and_modify(|e| *e += 1);
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

    fn toposort(
        &self,
        mut in_degrees: HashMap<(usize, usize), i32>,
    ) -> Option<Vec<(usize, usize)>> {
        let mut queue = Vec::new();
        let mut order = Vec::new();

        for ((row, col), indegree) in in_degrees.iter() {
            if *indegree == 0 {
                queue.push((*row, *col));
            }
        }

        while let Some((row, col)) = queue.pop() {
            order.push((row, col));

            let op = self.cells[row][col].borrow();

            for neighbor in op.get_downstream_neighbors().borrow().iter() {
                let neighbor_ref = neighbor.borrow();
                let coord = neighbor_ref.get_coordinate();
                let r = coord.0;
                let c = coord.1;
                *in_degrees.get_mut(&(r, c)).unwrap() -= 1;
                if in_degrees[&(r, c)] == 0 {
                    queue.push((r, c));
                }
            }
        }

        if in_degrees.len() != order.len() {
            return None;
        }

        Some(order)
    }

    fn do_operation(&mut self, row: usize, col: usize) -> bool {
        // print!("do_operation: ");
        // self.cells[row][col].borrow().print();
        // get all the affected cells and indegrees
        let mut in_degrees = HashMap::new();
        self.get_indegrees(row, col, &mut in_degrees);
        // print!("do_operation: In degrees: {:?}\n", in_degrees);

        //use indegrees to find toposort
        let order = self.toposort(in_degrees);
        if order.is_none() {
            // println!("do_operation: Cycle detected in the equation");
            return false;
        }
        let order = order.unwrap();

        // for each cell in the order, process the equation and set the value
        for coord in order {
            let val = self.process_cell_equation(coord.0, coord.1);
            self.cells[coord.0][coord.1].borrow_mut().set_value(val);
        }

        true
    }

    pub fn set_cell_equation(
        &mut self,
        cor: (usize, usize),
        c1: Option<(usize, usize)>,
        c2: Option<(usize, usize)>,
        v1: Option<i32>,
        v2: Option<i32>,
        t: Type,
    ) -> Status {
        let (row, col) = cor;
        assert!(
            col < self.n && row < self.m,
            "set_cell_equation: Invalid cell coordinates ({},{})",
            row,
            col
        );
        if t == Type::Slp {
            assert!(
                c1.is_none() ^ v1.is_none(),
                "set_cell_equation: Specify either a cell coordinate or a value"
            );
            assert!(
                c2.is_none() && v2.is_none(),
                "set_cell_equation: SLP equation should not have a second operand"
            );
            let op = match c1 {
                Some(c) => self.cells[c.0][c.1].clone(),
                None => SharedOperand::new(Value::new(None::<Coordinate>, v1)),
            };

            let eq = Equation::new(Coordinate(row, col), Some(t), Some(vec![op]));
            return self.set_cell_equation_from_eq(row, col, eq);
        }

        assert!(
            (c1.is_none() ^ v1.is_none()) || (c2.is_none() ^ v2.is_none()),
            "set_cell_equation: Specify either a cell coordinate or a value"
        );

        let op1 = match c1 {
            Some(c) => self.cells[c.0][c.1].clone(),
            None => SharedOperand::new(Value::new(None::<Coordinate>, v1)),
        };
        let op2 = match c2 {
            Some(c) => self.cells[c.0][c.1].clone(),
            None => SharedOperand::new(Value::new(None::<Coordinate>, v2)),
        };
        let ops = vec![op1, op2];

        let eq = Equation::new(Coordinate(row, col), Some(t), Some(ops));
        self.set_cell_equation_from_eq(row, col, eq)
    }

    pub fn _set_cell_value(&mut self, row: usize, col: usize, v: i32) -> Status {
        assert!(
            col < self.n && row < self.m,
            "set_cell_equation: Invalid cell coordinates ({},{})",
            row,
            col
        );

        let op1 = SharedOperand::new(Value::new(None::<Coordinate>, Some(v)));
        let op2 = SharedOperand::new(Value::new(None::<Coordinate>, Some(0)));

        let eq = Equation::new(Coordinate(row, col), Some(Type::Add), Some(vec![op1, op2]));
        self.set_cell_equation_from_eq(row, col, eq)
    }

    fn check_target_in_operands(&self, row: usize, col: usize, eq: Equation) -> bool {
        // println!("check_target_in_operands: Checking for cell ({},{})", row, col);
        let ops = eq.get_operands().clone();
        let op_coords: Vec<_> = ops
            .iter()
            .filter_map(|op| {
                let borrowed_op = op.borrow();
                if borrowed_op.is_cell() {
                    // Assuming has_coordinate checks if get_coordinate is valid
                    Some(*borrowed_op.get_coordinate())
                } else {
                    None
                }
            })
            .collect();

        if op_coords.iter().any(|&op| op == Coordinate(row, col)) {
            return true;
        }

        if eq.t == Type::Sum
            || eq.t == Type::Avg
            || eq.t == Type::Dev
            || eq.t == Type::Min
            || eq.t == Type::Max
        {
            let c1 = eq.get_operands()[0].borrow();
            let c2 = eq.get_operands()[1].borrow();

            let y1 = c1.get_coordinate().0;
            let x1 = c1.get_coordinate().1;
            let y2 = c2.get_coordinate().0;
            let x2 = c2.get_coordinate().1;

            for i in y1..=y2 {
                for j in x1..=x2 {
                    if i == row && j == col {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn set_cell_equation_from_eq(&mut self, row: usize, col: usize, eq: Equation) -> Status {
        // print!("New equation: ");
        // eq.print();
        // println!();

        let cell_ref = self.cells[row][col].clone();

        if self.check_target_in_operands(row, col, eq.clone()) {
            return Status::Err;
        }

        let old_eq = cell_ref.borrow().get_equation().clone();
        cell_ref
            .borrow_mut()
            .set_equation(eq, cell_ref.clone(), self);

        if !self.do_operation(row, col) {
            {
                cell_ref
                    .borrow_mut()
                    .set_equation(old_eq, cell_ref.clone(), self);
            }
            // println!("set_cell_equation: Failed to set equation due to cycle, reverting to old equation");
            // print!("Old equation: ");
            // cell_ref.borrow_mut().get_equation().print();
            // println!();
            return Status::Err;
        };

        Status::Ok
    }

    // pub fn print(&self) {
    //     println!("----------------------------------------------------------------------");
    //     for row in 0..self.m {
    //         for col in 0..self.n {
    //             print!("|{:6}", self.get_cell_value(row, col));
    //         }
    //         println!("|");
    //     }
    //     println!("----------------------------------------------------------------------");
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_spreadsheet() {
        let spreadsheet = SpreadSheet::new(3, 3);
        assert_eq!(spreadsheet.m, 3);
        assert_eq!(spreadsheet.n, 3);
        assert_eq!(spreadsheet.cells.len(), 3);
        assert_eq!(spreadsheet.cells[0].len(), 3);
    }

    #[test]
    fn test_set_and_get_cell_value() {
        let mut spreadsheet = SpreadSheet::new(3, 3);
        let status = spreadsheet._set_cell_value(1, 1, 42);
        assert_eq!(status, Status::Ok);
        assert_eq!(spreadsheet.get_cell_value(1, 1), Some(42));
    }

    #[test]
    fn test_set_cell_equation_addition() {
        let mut spreadsheet = SpreadSheet::new(3, 3);
        spreadsheet._set_cell_value(0, 0, 10);
        spreadsheet._set_cell_value(0, 1, 20);

        let status = spreadsheet.set_cell_equation(
            (0, 2),
            Some((0, 0)),
            Some((0, 1)),
            None,
            None,
            Type::Add,
        );
        assert_eq!(status, Status::Ok);
        assert_eq!(spreadsheet.get_cell_value(0, 2), Some(30));
    }

    #[test]
    fn test_set_cell_equation_subtraction() {
        let mut spreadsheet = SpreadSheet::new(3, 3);
        spreadsheet._set_cell_value(0, 0, 50);
        spreadsheet._set_cell_value(0, 1, 20);

        let status = spreadsheet.set_cell_equation(
            (0, 2),
            Some((0, 0)),
            Some((0, 1)),
            None,
            None,
            Type::Sub,
        );
        assert_eq!(status, Status::Ok);
        assert_eq!(spreadsheet.get_cell_value(0, 2), Some(30));
    }

    #[test]
    fn test_cycle_detection() {
        let mut spreadsheet = SpreadSheet::new(3, 3);
        spreadsheet._set_cell_value(0, 0, 10);
        spreadsheet._set_cell_value(0, 1, 20);

        spreadsheet.set_cell_equation((0, 2), Some((0, 0)), Some((0, 1)), None, None, Type::Add);

        let status =
            spreadsheet.set_cell_equation((0, 0), Some((0, 2)), None, None, None, Type::Add);
        assert_eq!(status, Status::Err); // Cycle detected
    }

    #[test]
    fn test_set_cell_equation_single_operand() {
        let mut spreadsheet = SpreadSheet::new(3, 3);
        spreadsheet._set_cell_value(0, 0, 1);

        let status =
            spreadsheet.set_cell_equation((0, 1), Some((0, 0)), None, None, None, Type::Slp);
        assert_eq!(status, Status::Ok);
        assert_eq!(spreadsheet.get_cell_value(0, 1), Some(1));
    }
}

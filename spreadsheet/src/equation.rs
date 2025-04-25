use super::spreadsheet::SpreadSheet;
use super::utils::Coordinate;
use super::utils::Type;
use super::value::SharedOperand;

use std::hash::{Hash, Hasher};
use std::thread::sleep;
use std::time::Duration;

#[derive(Eq, PartialEq, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Equation {
    pub coordinate: Coordinate,
    pub t: Type,

    // each equation should own its list of operands. When equation changes for a cell, construct a whole new one
    operands: Vec<SharedOperand>, // References to operands
                                  // when the equation is dropped, each reference is dropped, decreasing the ref count
}
impl Hash for Equation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinate.hash(state);
    }
}

impl Equation {
    pub fn new(
        coordinate: Coordinate,
        t: Option<Type>,
        operands: Option<Vec<SharedOperand>>,
    ) -> Self {
        let t = t.unwrap_or(Type::Nul);
        let operands = if t == Type::Nul {
            Vec::<SharedOperand>::new()
        } else {
            operands.expect("Operands cannot be None when Type is not NUL")
        };

        Equation {
            coordinate,
            t,
            operands,
        }
    }

    pub fn get_operands(&self) -> &Vec<SharedOperand> {
        &self.operands
    }

    pub fn process_equation_silent(
        &self,
        spreadsheet_ref: &SpreadSheet,
    ) -> (Option<i32>, Option<i32>) {
        // println!("Processing equation silent: ");
        // self.print();

        // just get the value, don't sleep
        // first ret value is none if any operand's value is None
        // operand is None means that it is an ERR cell
        // second ret value is time to sleep

        let t = self.t;
        if t == Type::Slp {
            let c = self.operands[0].borrow().get_value();

            if c.is_none() {
                return (None, None);
            }
            assert!(c.unwrap() >= 0, "Invalid negative sleep time");
            // do nothing
            return (c, c);
        }

        let operands = &self.operands;
        if operands.is_empty() {
            return (Some(0), None);
        }
        let v1 = operands[0].borrow().get_value();
        if v1.is_none() {
            return (None, None);
        }
        let v1 = v1.unwrap();

        let v2 = operands[1].borrow().get_value();
        if v2.is_none() {
            return (None, None);
        }
        let v2 = v2.unwrap();

        match t {
            Type::Add => (Some(v1 + v2), None),
            Type::Sub => (Some(v1 - v2), None),
            Type::Mul => (Some(v1 * v2), None),
            Type::Div => match v2 {
                0 => (None, None),
                _ => (Some(v1 / v2), None),
            },
            Type::Min => {
                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1 <= x2 && y1 <= y2, "Invalid range!");
                let mut min = i32::MAX;
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        min = min.min(spreadsheet_ref.get_cell_value(y, x).unwrap_or(i32::MAX));
                    }
                }
                (Some(min), None)
            }
            Type::Max => {
                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1 <= x2 && y1 <= y2, "Invalid range!");
                let mut max = i32::MIN;
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        max = max.max(spreadsheet_ref.get_cell_value(y, x).unwrap_or(i32::MIN));
                    }
                }
                (Some(max), None)
            }

            Type::Sum => {
                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1 <= x2 && y1 <= y2, "Invalid range!");
                let mut sum = 0;
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        sum += spreadsheet_ref.get_cell_value(y, x).unwrap_or(0);
                    }
                }
                (Some(sum), None)
            }
            Type::Avg => {
                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1 <= x2 && y1 <= y2, "Invalid range!");
                let mut count = 0;
                let mut sum = 0;
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        let v = spreadsheet_ref.get_cell_value(y, x);
                        sum += v.unwrap_or(0);
                        count += if v.is_some() { 1 } else { 0 };
                    }
                }
                (Some(sum / count), None)
            }
            Type::Dev => {
                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1 <= x2 && y1 <= y2, "Invalid range!");
                let mut count = 0;
                let mut sum = 0;
                let mut sq = 0;
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        let v = spreadsheet_ref.get_cell_value(y, x);
                        sum += v.unwrap_or(0);
                        count += if v.is_some() { 1 } else { 0 };
                        sq += v.unwrap_or(0) * v.unwrap_or(0);
                    }
                }
                let mean = sum as f64 / count as f64;
                let mean_sq = sq as f64 / count as f64;
                let std = (mean_sq - mean * mean).sqrt();
                (Some(std as i32), None)
            }
            _ => {
                panic!("Unsupported operation to process equation");
            }
        }
    }

    pub fn process_equation(&self, spreadsheet_ref: &SpreadSheet) -> Option<i32> {
        // println!("Processing equation: ");
        if self.operands.is_empty() {
            return Some(0);
        }

        let (val, sleep_time) = self.process_equation_silent(spreadsheet_ref);
        if let Some(sleep_time) = sleep_time {
            sleep(Duration::from_millis(sleep_time as u64));
        }
        val
    }

    // pub fn print(&self){
    //     if self.t!=Type::NUL {
    //         let coord0 = self.operands[0].borrow();
    //         let coord1 = self.operands[1].borrow();

    //         let str0 = if coord0.is_cell() {
    //             format!("({},{})", coord0.get_coordinate().0, coord0.get_coordinate().1)
    //         } else {
    //             format!("{}", coord0.get_value())
    //         };

    //         let str1 = if coord1.is_cell() {
    //             format!("({},{})", coord1.get_coordinate().0, coord1.get_coordinate().1)
    //         } else {
    //             format!("{}", coord1.get_value())
    //         };

    //         print!("Equation: {} {} {}", str0, self.t.to_str(), str1);
    //     }
    //     else {
    //         print!("Equation: ({},{}) NUL",self.coordinate.0,self.coordinate.1);
    //     }
    // }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    fn create_mock_operand(value: i32, coordinate: (usize, usize)) -> SharedOperand {
        SharedOperand::new(Value::new(Some(coordinate), Some(value)))
    }

    #[test]
    fn test_equation_addition() {
        let operand1 = create_mock_operand(5, (0, 0));
        let operand2 = create_mock_operand(3, (0, 1));
        let equation = Equation::new(
            Coordinate(0, 2),
            Some(Type::Add),
            Some(vec![operand1, operand2]),
        );

        let spreadsheet = SpreadSheet::new(10, 10); // Mock or real implementation
        assert_eq!(
            equation.process_equation_silent(&spreadsheet),
            (Some(8), None)
        );
    }

    #[test]
    fn test_equation_subtraction() {
        let operand1 = create_mock_operand(10, (0, 0));
        let operand2 = create_mock_operand(4, (0, 1));
        let equation = Equation::new(
            Coordinate(0, 2),
            Some(Type::Sub),
            Some(vec![operand1, operand2]),
        );

        let spreadsheet = SpreadSheet::new(10, 10); // Mock or real implementation
        assert_eq!(
            equation.process_equation_silent(&spreadsheet),
            (Some(6), None)
        );
    }

    #[test]
    fn test_equation_multiplication() {
        let operand1 = create_mock_operand(7, (0, 0));
        let operand2 = create_mock_operand(6, (0, 1));
        let equation = Equation::new(
            Coordinate(0, 2),
            Some(Type::Mul),
            Some(vec![operand1, operand2]),
        );

        let spreadsheet = SpreadSheet::new(10, 10); // Mock or real implementation
        assert_eq!(
            equation.process_equation_silent(&spreadsheet),
            (Some(42), None)
        );
    }

    #[test]
    fn test_equation_division() {
        let operand1 = create_mock_operand(20, (0, 0));
        let operand2 = create_mock_operand(4, (0, 1));
        let equation = Equation::new(
            Coordinate(0, 2),
            Some(Type::Div),
            Some(vec![operand1, operand2]),
        );

        let spreadsheet = SpreadSheet::new(10, 10); // Mock or real implementation
        assert_eq!(
            equation.process_equation_silent(&spreadsheet),
            (Some(5), None)
        );
    }

    #[test]
    fn test_equation_division_by_zero() {
        let operand1 = create_mock_operand(20, (0, 0));
        let operand2 = create_mock_operand(0, (0, 1));
        let equation = Equation::new(
            Coordinate(0, 2),
            Some(Type::Div),
            Some(vec![operand1, operand2]),
        );

        let spreadsheet = SpreadSheet::new(10, 10); // Mock or real implementation
        assert_eq!(equation.process_equation_silent(&spreadsheet), (None, None));
    }

    #[test]
    fn test_equation_minimum() {
        let operand1 = create_mock_operand(0, (0, 0));
        let operand2 = create_mock_operand(0, (1, 1));
        let equation = Equation::new(
            Coordinate(0, 2),
            Some(Type::Min),
            Some(vec![operand1, operand2]),
        );

        let mut spreadsheet = SpreadSheet::new(10, 10); // Mock or real implementation
        spreadsheet.set_cell_value(0, 0, 10);
        spreadsheet.set_cell_value(0, 1, 5);
        spreadsheet.set_cell_value(1, 0, 3);
        spreadsheet.set_cell_value(1, 1, 8);

        assert_eq!(
            equation.process_equation_silent(&spreadsheet),
            (Some(3), None)
        );
    }

    #[test]
    fn test_equation_maximum() {
        let operand1 = create_mock_operand(0, (0, 0));
        let operand2 = create_mock_operand(0, (1, 1));
        let equation = Equation::new(
            Coordinate(0, 2),
            Some(Type::Max),
            Some(vec![operand1, operand2]),
        );

        let mut spreadsheet = SpreadSheet::new(10, 10); // Mock or real implementation
        spreadsheet.set_cell_value(0, 0, 10);
        spreadsheet.set_cell_value(0, 1, 5);
        spreadsheet.set_cell_value(1, 0, 3);
        spreadsheet.set_cell_value(1, 1, 8);

        assert_eq!(
            equation.process_equation_silent(&spreadsheet),
            (Some(10), None)
        );
    }

    #[test]
    fn test_equation_sum() {
        let operand1 = create_mock_operand(0, (0, 0));
        let operand2 = create_mock_operand(0, (1, 1));
        let equation = Equation::new(
            Coordinate(0, 2),
            Some(Type::Sum),
            Some(vec![operand1, operand2]),
        );

        let mut spreadsheet = SpreadSheet::new(10, 10); // Mock or real implementation
        spreadsheet.set_cell_value(0, 0, 10);
        spreadsheet.set_cell_value(0, 1, 5);
        spreadsheet.set_cell_value(1, 0, 3);
        spreadsheet.set_cell_value(1, 1, 8);

        assert_eq!(
            equation.process_equation_silent(&spreadsheet),
            (Some(26), None)
        );
    }

    #[test]
    fn test_equation_average() {
        let operand1 = create_mock_operand(0, (0, 0));
        let operand2 = create_mock_operand(0, (1, 1));
        let equation = Equation::new(
            Coordinate(0, 2),
            Some(Type::Avg),
            Some(vec![operand1, operand2]),
        );

        let mut spreadsheet = SpreadSheet::new(10, 10); // Mock or real implementation
        spreadsheet.set_cell_value(0, 0, 10);
        spreadsheet.set_cell_value(0, 1, 5);
        spreadsheet.set_cell_value(1, 0, 3);
        spreadsheet.set_cell_value(1, 1, 8);

        assert_eq!(
            equation.process_equation_silent(&spreadsheet),
            (Some(6), None)
        );
    }
}

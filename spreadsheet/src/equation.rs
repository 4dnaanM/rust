use super::utils::Type;
use super::operand::SharedOperand;
use super::utils::Coordinate;
use super::spreadsheet::SpreadSheet;

use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Clone)]
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
    pub fn new(coordinate: Coordinate, t: Option<Type>, operands: Option<Vec<SharedOperand>>) -> Self {
        
        let t = t.unwrap_or(Type::NUL);
        let operands = if t == Type::NUL {
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

    pub fn process_equation(&self, spreadsheet_ref: &SpreadSheet) -> i32 {
        let t = self.t;
        let operands = &self.operands;
        assert!(operands.len() <= 2, "Equation must have 2 operands");
        let v1 = operands[0].borrow().get_value();
        let v2 = operands[1].borrow().get_value();
        match t{
            Type::ADD => v1 + v2,
            Type::SUB => v1 - v2,
            Type::MUL => v1 * v2,
            Type::DIV => v1 / v2,
            Type::MIN => v1.min(v2),
            Type::MAX => v1.max(v2),

            Type::SUM => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;
                
                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut sum = 0; 
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        sum += spreadsheet_ref.get_cell_value(y,x);
                    }
                }
                sum
            },
            Type::AVG => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let count = ((y2-y1+1)*(x2-x1+1)) as i32;
                let mut sum = 0; 
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        sum += spreadsheet_ref.get_cell_value(y,x);
                    }
                }
                sum/count
            },
            Type::DEV => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let count = ((y2-y1+1)*(x2-x1+1)) as i32;
                let mut sum = 0; 
                let mut sq = 0;
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        sum += spreadsheet_ref.get_cell_value(y,x);
                        sq += spreadsheet_ref.get_cell_value(y,x)*spreadsheet_ref.get_cell_value(y,x);
                    }
                }
                let mean = sum as f64 / count as f64;
                let mean_sq = sq as f64 / count as f64;
                let std = (mean_sq - mean*mean).sqrt();
                std as i32
            },
            
            _ => {
                panic!("Unsupported operation to process equation");
            }
        }
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
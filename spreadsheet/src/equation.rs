use super::utils::Type;
use super::value::SharedOperand;
use super::utils::Coordinate;
use super::spreadsheet::SpreadSheet;

use std::thread::sleep;
use std::time::Duration; 
use std::{hash::{Hash, Hasher}, i32};

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

    pub fn process_equation_silent(&self, spreadsheet_ref: &SpreadSheet) -> Option<i32> {
        // println!("Processing equation silent: ");
        // just get the value, don't sleep
        let t = self.t;
        if t == Type::SLP {
            let c = self.operands[0].borrow().get_value();

            if c.is_none() {
                return None;
            }
            assert!(c.unwrap() >= 0, "Invalid negative sleep time");
            // do nothing
            return c;
        }

        let operands = &self.operands;
        let v1 = operands[0].borrow().get_value();
        if v1.is_none() {
            return None;
        }
        let v1 = v1.unwrap();

        let v2 = operands[1].borrow().get_value();
        if v2.is_none() {
            return None;
        }
        let v2 = v2.unwrap();

        match t{
            Type::ADD => Some(v1 + v2),
            Type::SUB => Some(v1 - v2),
            Type::MUL => Some(v1 * v2),
            Type::DIV => {
                match v2 {
                    0 => None, 
                    _ => Some(v1 / v2)
                }
            },
            Type::MIN => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;
                
                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut min = i32::MAX; 
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        min = min.min(spreadsheet_ref.get_cell_value(y,x).unwrap_or(i32::MAX));
                    }
                }
                Some(min)
            },
            Type::MAX => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;
                
                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut max = i32::MIN; 
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        max = max.max(spreadsheet_ref.get_cell_value(y,x).unwrap_or(i32::MIN));
                    }
                }
                Some(max)
            },

            Type::SUM => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;
                
                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut sum = 0; 
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        sum += spreadsheet_ref.get_cell_value(y,x).unwrap_or(0);
                    }
                }
                Some(sum)
            },
            Type::AVG => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut count = 0;
                let mut sum = 0; 
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        let v = spreadsheet_ref.get_cell_value(y,x);
                        sum += v.unwrap_or(0);
                        count += if v.is_some() {1} else {0};
                    }
                }
                Some(sum/count)
            },
            Type::DEV => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut count = 0;
                let mut sum = 0; 
                let mut sq = 0;
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        let v = spreadsheet_ref.get_cell_value(y,x);
                        sum += v.unwrap_or(0);
                        count += if v.is_some() {1} else {0};
                        sq += v.unwrap_or(0)*v.unwrap_or(0);
                    }
                }
                let mean = sum as f64 / count as f64;
                let mean_sq = sq as f64 / count as f64;
                let std = (mean_sq - mean*mean).sqrt();
                Some(std as i32)
            },
            _ => {
                panic!("Unsupported operation to process equation");
            }
        }
    }
    
    pub fn process_equation(&self, spreadsheet_ref: &SpreadSheet) -> Option<i32> {
        // println!("Processing equation: ");
        let t = self.t;
        if t == Type::SLP {
            let c = self.operands[0].borrow().get_value();

            if c.is_none() {
                return None;
            }
            assert!(c.unwrap() >= 0, "Invalid negative sleep time");
            // do nothing for c seconds
            sleep(Duration::from_secs(c.unwrap() as u64));
            return c;
        }

        let operands = &self.operands;
        let v1 = operands[0].borrow().get_value();
        if v1.is_none() {
            return None;
        }
        let v1 = v1.unwrap();

        let v2 = operands[1].borrow().get_value();
        if v2.is_none() {
            return None;
        }
        let v2 = v2.unwrap();

        match t{
            Type::ADD => Some(v1 + v2),
            Type::SUB => Some(v1 - v2),
            Type::MUL => Some(v1 * v2),
            Type::DIV => {
                match v2 {
                    0 => None, 
                    _ => Some(v1 / v2)
                }
            },
            Type::MIN => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;
                
                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut min = i32::MAX; 
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        min = min.min(spreadsheet_ref.get_cell_value(y,x).unwrap_or(i32::MAX));
                    }
                }
                Some(min)
            },
            Type::MAX => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;
                
                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut max = i32::MIN; 
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        max = max.max(spreadsheet_ref.get_cell_value(y,x).unwrap_or(i32::MIN));
                    }
                }
                Some(max)
            },

            Type::SUM => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;
                
                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut sum = 0; 
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        sum += spreadsheet_ref.get_cell_value(y,x).unwrap_or(0);
                    }
                }
                Some(sum)
            },
            Type::AVG => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut count = 0;
                let mut sum = 0; 
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        let v = spreadsheet_ref.get_cell_value(y,x);
                        sum += v.unwrap_or(0);
                        count += if v.is_some() {1} else {0};
                    }
                }
                Some(sum/count)
            },
            Type::DEV => {

                let y1 = operands[0].borrow().get_coordinate().0;
                let x1 = operands[0].borrow().get_coordinate().1;
                let y2 = operands[1].borrow().get_coordinate().0;
                let x2 = operands[1].borrow().get_coordinate().1;

                assert!(x1<=x2 && y1<=y2, "Invalid range!");
                let mut count = 0;
                let mut sum = 0; 
                let mut sq = 0;
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        let v = spreadsheet_ref.get_cell_value(y,x);
                        sum += v.unwrap_or(0);
                        count += if v.is_some() {1} else {0};
                        sq += v.unwrap_or(0)*v.unwrap_or(0);
                    }
                }
                let mean = sum as f64 / count as f64;
                let mean_sq = sq as f64 / count as f64;
                let std = (mean_sq - mean*mean).sqrt();
                Some(std as i32)
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
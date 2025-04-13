use super::utils::Type;
use super::operand::SharedOperand;
use super::utils::Coordinate;

use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Clone)]
pub struct Equation {
    coordinate: Coordinate,
    t: Type,
    
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

    pub fn process_equation(&self) -> i32 {
        let t = self.t;
        let operands = &self.operands;
        match t{
            Type::ADD => {
                assert!(operands.len() == 2, "ADD operation requires exactly 2 operands");
                operands[0].borrow().get_value() + operands[1].borrow().get_value()
            }
            Type::SUB => {
                assert!(operands.len() == 2, "SUB operation requires exactly 2 operands");
                operands[0].borrow().get_value() - operands[1].borrow().get_value()
            }
            Type::MUL => {
                assert!(operands.len() == 2, "MUL operation requires exactly 2 operands");
                operands[0].borrow().get_value() * operands[1].borrow().get_value()
            }
            Type::DIV => {
                assert!(operands.len() == 2, "DIV operation requires exactly 2 operands");
                operands[0].borrow().get_value() / operands[1].borrow().get_value()
            }
            Type::MIN => {
                assert!(operands.len() == 2, "MIN operation requires exactly 2 operands");
                operands[0].borrow().get_value().min(operands[1].borrow().get_value())
            }
            Type::MAX => {
                assert!(operands.len() == 2, "MAX operation requires exactly 2 operands");
                operands[0].borrow().get_value().max(operands[1].borrow().get_value())
            }
            _ => {
                panic!("Unsupported operation to process equation");
            }
        }
    }

    pub fn print(&self){
        if self.t!=Type::NUL {
            let coord0 = self.operands[0].borrow();
            let coord1 = self.operands[1].borrow();

            let str0 = if coord0.is_cell() {
                format!("({},{})", coord0.get_coordinate().0, coord0.get_coordinate().1)
            } else {
                format!("{}", coord0.get_value())
            };

            let str1 = if coord1.is_cell() {
                format!("({},{})", coord1.get_coordinate().0, coord1.get_coordinate().1)
            } else {
                format!("{}", coord1.get_value())
            };

            print!("Equation: {} {} {}", str0, self.t.to_str(), str1);
        }
        else {
            print!("Equation: ({},{}) NUL",self.coordinate.0,self.coordinate.1);
        }   
    }  
    
}
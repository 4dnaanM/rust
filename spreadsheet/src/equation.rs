use super::utils::Type;
use super::operand::SharedOperand;
use super::utils::Coordinate;

use std::hash::{Hash, Hasher};
use std::ops::{Add,Sub,Mul,Div};

#[derive(Eq, PartialEq, Clone)]
pub struct Equation<T> {
    coordinate: Coordinate,
    t: Type,
    
    // each equation should own its list of operands. When equation changes for a cell, construct a whole new one
    operands: Vec<SharedOperand<T>>, // References to operands
    // when the equation is dropped, each reference is droped, decreasing the ref count
}

impl<T> Hash for Equation<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinate.hash(state);
    }
}


impl <T: Clone + Copy + From<i32> + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T>>
    Equation<T> {
    pub fn new(coordinate: Coordinate, t: Option<Type>, operands: Option<Vec<SharedOperand<T>>>) -> Self {
        
        let t = t.unwrap_or(Type::NUL);
        let operands = if t == Type::NUL {
            Vec::<SharedOperand<T>>::new()
        } else {
            operands.expect("Operands cannot be None when Type is not NUL")
        };

        Equation {
            coordinate,
            t,
            operands,
        }
    }

    
    pub fn get_operands(&self) -> &Vec<SharedOperand<T>> {
        &self.operands
    }

    pub fn process_equation(&self) -> T {
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
            _ => {
                panic!("Unsupported operation to process equation");
            }
        }
    }
}
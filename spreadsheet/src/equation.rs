use super::utils::{Coordinate, Type};
use super::operand::Operand;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

// equation used to have reference to cell but now it has reference to operand
// this means cell and equation are meaningless without a spreadsheet
// if possible use a ref to a cell (some clone issue)
pub struct Equation<'a,T> {
    coordinate: Coordinate, 
    t: Type, 
    operands: Vec<&'a Operand<'a,T>>
}

impl<'a,T: Clone + Copy + From<i32> + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T>> Equation<'a,T> {
    pub fn new(coord: Coordinate, t: Option<Type>, operands: Option<Vec<&'a Operand<'a,T>>>) -> Self {
        
        let t = t.unwrap_or(Type::NUL);
        let operands = if t == Type::NUL {
            Vec::<&Operand<T>>::new()
        } else {
            operands.expect("Operands cannot be None when Type is not NUL")
        };

        Equation {
            coordinate: coord,
            t,
            operands,
        }
    }

    pub fn process_equation(&self) -> T {
        let t = &self.t;
        let operands = &self.operands.iter().collect::<Vec<_>>();
        match t{
            Type::ADD => {
                assert!(operands.len() == 2, "ADD operation requires exactly 2 operands");
                operands[0].get_value() + operands[1].get_value()
            }
            Type::SUB => {
                assert!(operands.len() == 2, "SUB operation requires exactly 2 operands");
                operands[0].get_value() - operands[1].get_value()
            }
            Type::MUL => {
                assert!(operands.len() == 2, "MUL operation requires exactly 2 operands");
                operands[0].get_value() * operands[1].get_value()
            }
            Type::DIV => {
                assert!(operands.len() == 2, "DIV operation requires exactly 2 operands");
                operands[0].get_value() / operands[1].get_value()
            }
            _ => {
                panic!("Unsupported operation to process equation");
            }
        }
    }
}
use super::utils::{Coordinate, Type};
use super::operand::Operand;

// equation used to have reference to cell but now it has reference to operand
// this means cell and equation are meaningless without a spreadsheet
// if possible use a ref to a cell (some clone issue)
pub struct Equation<T> {
    coordinate: Coordinate, 
    t: Type, 
    operands: Vec<Operand<T>>
}

impl<T: From<i32>> Equation<T> {
    pub fn new(coord: Coordinate, t: Option<Type>, operands: Option<Vec<Operand<T>>>) -> Self {
        
        let t = t.unwrap_or(Type::NUL);
        let operands = if t == Type::NUL {
            Vec::<Operand<T>>::new()
        } else {
            operands.expect("Operands cannot be None when Type is not NUL")
        };

        Equation {
            coordinate: coord,
            t,
            operands,
        }
    }

    fn process_cell_equation(t: Type, operands: Vec<Operand<T>>) -> T {
        T::from(0)
    }
}
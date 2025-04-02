use super::utils::{Coordinate, Type};
use super::cell::Operand;
pub struct Equation<T> {
    coordinate: Coordinate, 
    t: Type, 
    operands: Vec<Operand<T>>
}

impl<T: From<i32>> Equation<T> {
    pub fn new(coord: Coordinate) -> Self {
        Equation {
            coordinate: coord, 
            t: Type::NUL, 
            operands: Vec::<Operand<T>>::new()
        }
    }

    fn process_cell_equation(t: Type, operands: Vec<Operand<T>>) -> T {
        T::from(0)
    }
}
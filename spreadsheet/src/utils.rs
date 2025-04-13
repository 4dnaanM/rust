#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Coordinate (pub usize, pub usize);
impl From<(usize, usize)> for Coordinate {
    fn from((row, col): (usize, usize)) -> Self {
        Coordinate(row, col)
    }
}
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Type {
    NUL,
    ADD, 
    SUB, 
    MUL, 
    DIV,
    MIN, 
    MAX
}

impl Type {
    pub fn to_str(&self) -> &str {
        match self {
            Type::ADD => "ADD",
            Type::SUB => "SUB",
            Type::MUL => "MUL",
            Type::DIV => "DIV",
            Type::NUL => "NUL",
            Type::MIN => "MIN",
            Type::MAX => "MAX"
        }
    }
}
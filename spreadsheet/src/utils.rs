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
    MAX,
    SUM,
    AVG,
    DEV,
    SLP
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
            Type::MAX => "MAX",
            Type::SUM => "SUM",
            Type::AVG => "AVG",
            Type::DEV => "DEV",
            Type::SLP => "SLP",
        }
    }
    pub fn from_str(s: &str) -> Type {
        match s {
            "+" => Type::ADD,
            "-" => Type::SUB,
            "*" => Type::MUL,
            "/" => Type::DIV,
            "NUL" => Type::NUL,
            "MIN" => Type::MIN,
            "MAX" => Type::MAX,
            "SUM" => Type::SUM,
            "AVG" => Type::AVG,
            "STDEV" => Type::DEV,
            "SLP" => Type::SLP,
            _ => panic!("Unknown type: {}", s)
        }
    }
}

pub enum Status {
    OK,
    ERR
}
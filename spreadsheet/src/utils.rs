use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SerializableRcRefCell<T>(pub Rc<RefCell<T>>);

impl<T> SerializableRcRefCell<T> {
    pub fn new(value: T) -> Self {
        SerializableRcRefCell(Rc::new(RefCell::new(value)))
    }

    // Add the ptr_eq method
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        Rc::ptr_eq(&this.0, &other.0)
    }

    // Add the as_ptr method
    pub fn as_ptr(&self) -> *const RefCell<T> {
        Rc::as_ptr(&self.0)
    }
}

impl<T> Serialize for SerializableRcRefCell<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.borrow().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for SerializableRcRefCell<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(SerializableRcRefCell(Rc::new(RefCell::new(value))))
    }
}

#[derive(
    Clone, Copy, Hash, Eq, PartialEq, Debug, serde_derive::Serialize, serde_derive::Deserialize,
)]
pub struct Coordinate(pub usize, pub usize);
impl From<(usize, usize)> for Coordinate {
    fn from((row, col): (usize, usize)) -> Self {
        Coordinate(row, col)
    }
}
#[derive(Eq, PartialEq, Clone, Copy, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
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
    SLP,
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
            "SLEEP" => Type::SLP,
            _ => panic!("Unknown type: {}", s),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Status {
    OK,
    ERR,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for Coordinate struct
    #[test]
    fn test_coordinate_creation() {
        let coord = Coordinate(3, 5);
        assert_eq!(coord.0, 3);
        assert_eq!(coord.1, 5);
    }

    #[test]
    fn test_coordinate_from_tuple() {
        let coord: Coordinate = (4, 6).into();
        assert_eq!(coord.0, 4);
        assert_eq!(coord.1, 6);
    }

    // Tests for Type enum
    #[test]
    fn test_type_to_str() {
        assert_eq!(Type::ADD.to_str(), "ADD");
        assert_eq!(Type::SUB.to_str(), "SUB");
        assert_eq!(Type::MUL.to_str(), "MUL");
        assert_eq!(Type::DIV.to_str(), "DIV");
        assert_eq!(Type::NUL.to_str(), "NUL");
        assert_eq!(Type::MIN.to_str(), "MIN");
        assert_eq!(Type::MAX.to_str(), "MAX");
        assert_eq!(Type::SUM.to_str(), "SUM");
        assert_eq!(Type::AVG.to_str(), "AVG");
        assert_eq!(Type::DEV.to_str(), "DEV");
        assert_eq!(Type::SLP.to_str(), "SLP");
    }

    #[test]
    fn test_type_from_str() {
        assert_eq!(Type::from_str("+"), Type::ADD);
        assert_eq!(Type::from_str("-"), Type::SUB);
        assert_eq!(Type::from_str("*"), Type::MUL);
        assert_eq!(Type::from_str("/"), Type::DIV);
        assert_eq!(Type::from_str("NUL"), Type::NUL);
        assert_eq!(Type::from_str("MIN"), Type::MIN);
        assert_eq!(Type::from_str("MAX"), Type::MAX);
        assert_eq!(Type::from_str("SUM"), Type::SUM);
        assert_eq!(Type::from_str("AVG"), Type::AVG);
        assert_eq!(Type::from_str("STDEV"), Type::DEV);
        assert_eq!(Type::from_str("SLEEP"), Type::SLP);
    }

    #[test]
    #[should_panic(expected = "Unknown type: INVALID")]
    fn test_type_from_str_invalid() {
        Type::from_str("INVALID");
    }
}

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
    Nul,
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Sum,
    Avg,
    Dev,
    Slp,
}

impl Type {
    pub fn from_str(s: &str) -> Type {
        match s {
            "+" => Type::Add,
            "-" => Type::Sub,
            "*" => Type::Mul,
            "/" => Type::Div,
            "NUL" => Type::Nul,
            "MIN" => Type::Min,
            "MAX" => Type::Max,
            "SUM" => Type::Sum,
            "AVG" => Type::Avg,
            "STDEV" => Type::Dev,
            "SLEEP" => Type::Slp,
            _ => panic!("Unknown type: {}", s),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Status {
    Ok,
    Err,
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

    #[test]
    fn test_type_from_str() {
        assert_eq!(Type::from_str("+"), Type::Add);
        assert_eq!(Type::from_str("-"), Type::Sub);
        assert_eq!(Type::from_str("*"), Type::Mul);
        assert_eq!(Type::from_str("/"), Type::Div);
        assert_eq!(Type::from_str("NUL"), Type::Nul);
        assert_eq!(Type::from_str("MIN"), Type::Min);
        assert_eq!(Type::from_str("MAX"), Type::Max);
        assert_eq!(Type::from_str("SUM"), Type::Sum);
        assert_eq!(Type::from_str("AVG"), Type::Avg);
        assert_eq!(Type::from_str("STDEV"), Type::Dev);
        assert_eq!(Type::from_str("SLEEP"), Type::Slp);
    }

    #[test]
    #[should_panic(expected = "Unknown type: INVALID")]
    fn test_type_from_str_invalid() {
        Type::from_str("INVALID");
    }
}

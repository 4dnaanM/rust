#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Coordinate (pub usize, pub usize);
impl From<(usize, usize)> for Coordinate {
    fn from((row, col): (usize, usize)) -> Self {
        Coordinate(row, col)
    }
}
#[derive(Eq, PartialEq)]
pub enum Type {
    NUL,
    ADD, 
    SUB, 
    MUL, 
    DIV
}
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Coordinate (pub usize, pub usize);
pub enum Type {
    NUL,
    ADD, 
    SUB, 
    MUL, 
    DIV
}
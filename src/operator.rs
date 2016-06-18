use std::fmt;

#[derive(Debug,Copy,Clone)]
pub enum Operator {
    ConditionalMove([u8; 3]),
    ArrayIndex([u8; 3]),
    ArrayAmendment([u8; 3]),
    Addition([u8; 3]),
    Multiplication([u8; 3]),
    Division([u8; 3]),
    NotAnd([u8; 3]),
    //
    Orthography(u8, u32)
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

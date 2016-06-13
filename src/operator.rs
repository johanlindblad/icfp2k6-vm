#[derive(Debug)]
pub enum Operator {
    ConditionalMove([u8; 3]),
    ArrayIndex([u8; 3]),
    Addition([u8; 3]),
    Multiplication([u8; 3]),
    Division([u8; 3]),
    NotAnd([u8; 3]),
    //
    Orthography(u8, u32)
}

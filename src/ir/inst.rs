use slotmap::new_key_type;

use super::value::Value;

new_key_type! {
pub struct Inst;
}

pub enum BinaryOpcode {
    Add,
    UAdd,
    Sub,
    Mul,
    Div
}

pub struct Binary {
    pub opcode: BinaryOpcode, 
    pub args: [Value; 2],
}

pub enum InstData {
    Binary(Binary)
}
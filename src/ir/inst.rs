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

pub enum InstData {
    Binary{opcode: BinaryOpcode, args: [Value; 2]}
}
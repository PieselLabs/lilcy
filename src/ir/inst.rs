use crate::impl_key;

use super::value::Value;

impl_key! {
    pub struct Inst(u32);
}

pub enum BinaryOpcode {
    Add,
    UAdd,
    Sub,
    Mul,
    Div,
}

pub struct Binary {
    pub opcode: BinaryOpcode,
    pub args: [Value; 2],
}

pub enum InstData {
    Binary(Binary),
}

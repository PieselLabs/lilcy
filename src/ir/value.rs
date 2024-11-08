use slotmap::new_key_type;

use super::{inst::Inst, types::Type};


new_key_type! {
pub struct Value;
}

pub enum ValueData {
    Inst {typ: Type, inst: Inst},
    Arg {typ: Type, idx: u16},
}

impl ValueData {
    pub fn get_type(&self) -> Type {
        match self {
            ValueData::Inst { typ, .. } => *typ,
            ValueData::Arg { typ, .. } => *typ,
        }
    }
}
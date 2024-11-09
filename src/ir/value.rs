use crate::impl_key;

use super::{inst::Inst, types::Type};

impl_key! {
    pub struct Value(u32);
}

pub enum ValueData {
    Inst { typ: Type, inst: Inst },
    Arg { typ: Type, idx: u16 },
}

impl ValueData {
    pub fn get_type(&self) -> Type {
        match self {
            ValueData::Inst { typ, .. } => *typ,
            ValueData::Arg { typ, .. } => *typ,
        }
    }
}

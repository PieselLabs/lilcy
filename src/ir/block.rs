use std::collections::LinkedList;

use crate::impl_key;

use super::inst::Inst;

impl_key! {
    pub struct Block(u16);
}

#[derive(Default)]
pub struct BlockData {
    pub insts: LinkedList<Inst>,
}

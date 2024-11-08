use std::collections::LinkedList;

use slotmap::new_key_type;

use super::inst::Inst;

new_key_type! {
    pub struct Block;
}

#[derive(Default)]
pub struct BlockData {
    pub insts: LinkedList<Inst>
}

use slotmap::{new_key_type, DefaultKey, SlotMap};
use std::collections::LinkedList;
use std::fmt::{Display, Formatter};

pub mod x64;

pub enum RegType {
    Int,
    FP,
    Vec,
}

pub trait TargetReg: Display + Sized + Copy {}

#[derive(Copy, Clone)]
pub enum Reg<T: TargetReg> {
    Fixed(T),
    Virtual(i16),
}

pub enum GenericInstruction {
    BR { block_idx: i16 },
}

impl<T: TargetReg> Display for Reg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Reg::Fixed(r) => r.fmt(f),
            Reg::Virtual(id) => write!(f, "$vreg{id}"),
        }
    }
}

pub trait TargetInst: Display + Sized + Copy {}

pub enum Inst<I: TargetInst> {
    Generic(GenericInstruction),
    Target(I),
}

pub struct Block {
    pub instructions: LinkedList<InstId>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            instructions: LinkedList::new(),
        }
    }

    pub fn add_instruction(&mut self, inst: InstId) {
        self.instructions.push_back(inst)
    }
}

new_key_type! {
pub struct InstId;
pub struct BlockId;
}

pub struct Func<I: TargetInst> {
    pub blocks: LinkedList<BlockId>,

    pub instructions_data: SlotMap<InstId, Inst<I>>,
    pub blocks_data: SlotMap<BlockId, Block>,
}

impl<I: TargetInst> Func<I> {
    pub fn new() -> Self {
        Self {
            blocks: LinkedList::new(),
            instructions_data: SlotMap::with_key(),
            blocks_data: SlotMap::with_key(),
        }
    }

    pub fn add_instruction(&mut self, inst: Inst<I>) -> InstId {
        self.instructions_data.insert(inst)
    }

    pub fn add_block(&mut self) -> (BlockId, &mut Block) {
        let key = self.blocks_data.insert(Block::new());
        self.blocks.push_back(key);
        (key, &mut self.blocks_data[key])
    }
}

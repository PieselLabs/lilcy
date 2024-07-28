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

pub struct Block<I: TargetInst> {
    pub instructions: Vec<I>,
}

impl<I: TargetInst> Block<I> {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
}

pub struct Func<I: TargetInst> {
    pub blocks: Vec<Block<I>>,
}

impl<I: TargetInst> Func<I> {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
}

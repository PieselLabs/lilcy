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
    Concrete(T),
    Virtual(i32),
}

impl<T: TargetReg> Display for Reg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Reg::Concrete(r) => r.fmt(f),
            Reg::Virtual(id) => write!(f, "%{id}"),
        }
    }
}

pub trait TargetInstr: Display + Sized + Copy {}

pub struct Block<I: TargetInstr> {
    pub instructions: Vec<I>,
}

impl<I: TargetInstr> Block<I> {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
}

pub struct Module<I: TargetInstr> {
    pub blocks: Vec<Block<I>>,
}

impl<I: TargetInstr> Module<I> {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
}

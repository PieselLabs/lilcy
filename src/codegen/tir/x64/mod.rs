pub mod emit_mc;

use crate::codegen::tir::{BlockId, TargetInst, TargetReg};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum X64Reg {
    AX,
    BX,
    CX,
    DX,

    SI,
    DI,
    BP,
    SP,

    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    EFLAGS,
}
impl Display for X64Reg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            X64Reg::AX => write!(f, "$ax"),
            X64Reg::BX => write!(f, "$bx"),
            X64Reg::CX => write!(f, "$cx"),
            X64Reg::DX => write!(f, "$dx"),
            X64Reg::SI => write!(f, "$si"),
            X64Reg::DI => write!(f, "$di"),
            X64Reg::BP => write!(f, "$bp"),
            X64Reg::SP => write!(f, "$sp"),
            X64Reg::R8 => write!(f, "$r8"),
            X64Reg::R9 => write!(f, "$r9"),
            X64Reg::R10 => write!(f, "$r10"),
            X64Reg::R11 => write!(f, "$r11"),
            X64Reg::R12 => write!(f, "$r12"),
            X64Reg::R13 => write!(f, "$r13"),
            X64Reg::R14 => write!(f, "$r14"),
            X64Reg::R15 => write!(f, "$r15"),
            X64Reg::EFLAGS => write!(f, "$eflags"),
        }
    }
}
impl TargetReg for X64Reg {}

type Reg = super::Reg<X64Reg>;

#[derive(Copy, Clone)]
pub struct Mem {
    pub reg: Reg,
    index: Option<Reg>,
    scale: u8,
    disp: i32,
}

impl Display for Mem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let reg = self.reg;
        let scale = self.scale;
        let disp = self.disp;

        write!(f, "[{reg}")?;
        if let Some(idx) = self.index {
            write!(f, "+{idx}*{scale}")?;
        }
        write!(f, "]")?;

        if disp != 0 {
            write!(f, "+{disp}")?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum JumpTarget {
    BB(BlockId),
}

#[derive(Copy, Clone)]
pub enum X64Inst {
    MOV64rr { src: Reg, dst: Reg },
    MOV64rm { src: Reg, dst: Mem },
    CMP64rr { lhs: Reg, rhs: Reg },

    JLE { target: JumpTarget },
    JE { target: JumpTarget },
    JL { target: JumpTarget },
}

impl TargetInst for X64Inst {}

impl Display for X64Inst {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            X64Inst::MOV64rr { src, dst } => write!(f, "{dst} = MOV64rr {src}"),
            X64Inst::MOV64rm { src, dst } => write!(f, "{dst} = MOV64rm {src}"),
            X64Inst::CMP64rr { lhs, rhs } => write!(f, "$eflags = CMP64rr {lhs} {rhs}"),
            X64Inst::JLE { target: _ } => write!(f, "JLE $eflags"),
            X64Inst::JE { target: _ } => write!(f, "JEQ $eflags"),
            X64Inst::JL { target: _ } => write!(f, "JLT $eflags"),
        }
    }
}

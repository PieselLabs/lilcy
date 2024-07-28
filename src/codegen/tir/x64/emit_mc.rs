use crate::codegen::tir::x64::{JumpTarget, X64Inst, X64Reg};
use crate::codegen::tir::x64::{Mem, Reg};
use crate::codegen::tir::{Block, Func};
use iced_x86::code_asm::*;
use std::collections::HashMap;

pub struct X64AsmPrinter {
    asm: CodeAssembler,
    block_labels: HashMap<i32, CodeLabel>,
}

impl X64AsmPrinter {
    pub fn new() -> Self {
        Self {
            asm: CodeAssembler::new(64).unwrap(),
            block_labels: HashMap::new(),
        }
    }
}
pub trait EmitMC {
    fn emit(&self, p: &mut X64AsmPrinter) -> Result<(), IcedError>;
}

fn convert_to_iced_reg64(r: Reg) -> AsmRegister64 {
    if let Reg::Concrete(r) = r {
        match r {
            X64Reg::AX => rax,
            X64Reg::BX => rbx,
            X64Reg::CX => rcx,
            X64Reg::DX => rdx,
            X64Reg::SI => rsi,
            X64Reg::DI => rdi,
            X64Reg::BP => rbp,
            X64Reg::SP => rsp,
            X64Reg::R8 => r8,
            X64Reg::R9 => r9,
            X64Reg::R10 => r10,
            X64Reg::R11 => r11,
            X64Reg::R12 => r12,
            X64Reg::R13 => r13,
            X64Reg::R14 => r14,
            X64Reg::R15 => r15,

            X64Reg::EFLAGS => unreachable!(),
        }
    } else {
        panic!()
    }
}

fn convert_to_iced_mem64(r: &Mem) -> AsmMemoryOperand {
    let mut res = qword_ptr(convert_to_iced_reg64(r.reg));

    if let Some(idx) = r.index {
        res = res + convert_to_iced_reg64(idx) * r.scale;
    }

    if r.disp != 0 {
        res = res + r.disp;
    }

    res
}

impl EmitMC for X64Inst {
    fn emit(&self, p: &mut X64AsmPrinter) -> Result<(), IcedError> {
        match self {
            X64Inst::MOV64rr { src, dst } => {
                let src = convert_to_iced_reg64(*src);
                let dst = convert_to_iced_reg64(*dst);
                p.asm.mov(dst, src)
            }
            X64Inst::MOV64rm { src, dst } => {
                let src = convert_to_iced_reg64(*src);
                let dst = convert_to_iced_mem64(dst);
                p.asm.mov(dst, src)
            }
            X64Inst::CMP64rr { lhs, rhs } => {
                let lhs = convert_to_iced_reg64(*lhs);
                let rhs = convert_to_iced_reg64(*rhs);
                p.asm.cmp(lhs, rhs)
            }
            X64Inst::JLE {
                target: JumpTarget::BB(block),
            } => p.asm.jle(p.block_labels[block]),
            X64Inst::JE {
                target: JumpTarget::BB(block),
            } => p.asm.je(p.block_labels[block]),
            X64Inst::JL {
                target: JumpTarget::BB(block),
            } => p.asm.jl(p.block_labels[block]),
        }
    }
}

impl EmitMC for Block<X64Inst> {
    fn emit(&self, asm: &mut X64AsmPrinter) -> Result<(), IcedError> {
        for inst in &self.instructions {
            inst.emit(asm)?;
        }
        Ok(())
    }
}

impl EmitMC for Func<X64Inst> {
    fn emit(&self, p: &mut X64AsmPrinter) -> Result<(), IcedError> {
        for (id, _) in self.blocks.iter().enumerate() {
            let label = p.asm.create_label();
            p.block_labels.insert(id as i32, label);
        }

        for (id, block) in self.blocks.iter().enumerate() {
            p.asm
                .set_label(p.block_labels.get_mut(&(id as i32)).unwrap())?;
            block.emit(p)?;
        }
        Ok(())
    }
}

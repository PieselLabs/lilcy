use crate::codegen::tir::x64::{Inst, X64Reg};
use crate::codegen::tir::x64::{Mem, Reg};
use iced_x86::code_asm::*;

pub trait EmitMC {
    fn emit(&self, asm: &mut CodeAssembler) -> Result<(), IcedError>;
}

fn convert_to_iced_reg64(r: &Reg) -> AsmRegister64 {
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
        }
    } else {
        panic!()
    }
}

fn convert_to_iced_mem64(r: &Mem) -> AsmMemoryOperand {
    let mut res = qword_ptr(convert_to_iced_reg64(&r.reg));

    if let Some(idx) = r.index {
        res = res + convert_to_iced_reg64(&idx) * r.scale;
    }

    if r.disp != 0 {
        res = res + r.disp;
    }

    res
}

impl EmitMC for Inst {
    fn emit(&self, asm: &mut CodeAssembler) -> Result<(), IcedError> {
        match self {
            Inst::MOV64rr { src, dst } => {
                let src = convert_to_iced_reg64(src);
                let dst = convert_to_iced_reg64(dst);

                asm.mov(dst, src)
            }
            Inst::MOV64rm { src, dst } => {
                let src = convert_to_iced_reg64(src);
                let dst = convert_to_iced_mem64(dst);

                asm.mov(dst, src)
            }
        }
    }
}

use crate::codegen::tir::x64::{JumpTarget, X64Inst, X64Reg};
use crate::codegen::tir::x64::{Mem, Reg};
use crate::codegen::tir::{Block, BlockId, Func, Inst};
use iced_x86::code_asm::*;
use slotmap::SecondaryMap;

fn convert_to_iced_reg64(r: Reg) -> AsmRegister64 {
    if let Reg::Fixed(r) = r {
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

fn emit_inst_mc(
    inst: &X64Inst,
    asm: &mut CodeAssembler,
    block_labels: &SecondaryMap<BlockId, CodeLabel>,
) -> Result<(), IcedError> {
    match inst {
        X64Inst::MOV64rr { src, dst } => {
            let src = convert_to_iced_reg64(*src);
            let dst = convert_to_iced_reg64(*dst);
            asm.mov(dst, src)
        }
        X64Inst::MOV64rm { src, dst } => {
            let src = convert_to_iced_reg64(*src);
            let dst = convert_to_iced_mem64(dst);
            asm.mov(dst, src)
        }
        X64Inst::CMP64rr { lhs, rhs } => {
            let lhs = convert_to_iced_reg64(*lhs);
            let rhs = convert_to_iced_reg64(*rhs);
            asm.cmp(lhs, rhs)
        }
        X64Inst::JLE {
            target: JumpTarget::BB(block),
        } => asm.jle(block_labels[*block]),
        X64Inst::JE {
            target: JumpTarget::BB(block),
        } => asm.je(block_labels[*block]),
        X64Inst::JL {
            target: JumpTarget::BB(block),
        } => asm.jl(block_labels[*block]),
    }
}

fn emit_block_mc(
    block: &Block,
    f: &Func<X64Inst>,
    asm: &mut CodeAssembler,
    block_labels: &SecondaryMap<BlockId, CodeLabel>,
) -> Result<(), IcedError> {
    for &id in &block.instructions {
        let inst = &f.instructions_data[id];
        if let Inst::Target(inst) = inst {
            emit_inst_mc(inst, asm, block_labels)?;
        } else {
            todo!("return error")
        }
    }
    Ok(())
}

pub fn emit_mc(f: &Func<X64Inst>) -> Result<(), IcedError> {
    let mut asm = CodeAssembler::new(64)?;

    let mut block_labels = SecondaryMap::new();

    for &id in &f.blocks {
        let label = asm.create_label();
        block_labels.insert(id, label);
    }

    for &id in &f.blocks {
        asm.set_label(block_labels.get_mut(id).unwrap())?;
        let block = &f.blocks_data[id];
        emit_block_mc(block, f, &mut asm, &block_labels)?;
    }
    Ok(())
}

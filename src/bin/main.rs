use lilcy::codegen::tir::x64::emit_mc::emit_mc;
use lilcy::codegen::tir::x64::{X64Inst, X64Reg};
use lilcy::codegen::tir::{x64, Func, Inst, InstId, Reg};
use std::mem::size_of;

fn main() {
    let mut f = Func::<X64Inst>::new();

    let inst = f.add_instruction(Inst::Target(X64Inst::MOV64rr {
        src: Reg::Fixed(X64Reg::AX),
        dst: Reg::Fixed(X64Reg::AX),
    }));

    let (_, block) = f.add_block();

    block.add_instruction(inst);

    emit_mc(&f).unwrap();

    println!("{}", size_of::<X64Inst>());
    println!("{}", size_of::<InstId>());
}

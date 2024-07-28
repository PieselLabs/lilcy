use iced_x86::code_asm::CodeAssembler;
use lilcy::codegen::tir::x64::emit_mc::{EmitMC, X64AsmPrinter};
use lilcy::codegen::tir::{x64, Block, Func, Reg};
use std::mem::size_of;

fn main() {
    let mut m = Func::<x64::X64Inst>::new();

    let mut b = Block::<x64::X64Inst>::new();

    b.instructions.push(x64::X64Inst::MOV64rr {
        src: Reg::Concrete(x64::X64Reg::AX),
        dst: Reg::Concrete(x64::X64Reg::AX),
    });

    m.blocks.push(b);

    // println!("{m}");

    let mut p = X64AsmPrinter::new();
    m.emit(&mut p).unwrap();

    println!("{}", size_of::<x64::X64Inst>());
}

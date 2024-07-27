use iced_x86::code_asm::CodeAssembler;
use lilcy::codegen::tir::x64::emit_mc::EmitMC;
use lilcy::codegen::tir::{x64, Block, Module, Reg};

fn main() {
    let mut m = Module::<x64::Inst>::new();

    let mut b = Block::<x64::Inst>::new();

    b.instructions.push(x64::Inst::MOV64rr {
        src: Reg::Concrete(x64::X64Reg::AX),
        dst: Reg::Concrete(x64::X64Reg::AX),
    });

    m.blocks.push(b);

    // println!("{m}");

    let mut a = CodeAssembler::new(64).unwrap();
    m.emit(&mut a).unwrap();
    let instrs = a.take_instructions();
}

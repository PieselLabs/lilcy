use crate::codegen::tir::{RegClass, TargetInst, TargetReg};

pub trait RegisterAllocator<R: TargetReg> {
    fn alloc_reg(&mut self, cls: RegClass) -> R;
}

pub fn allocate_registers<I: TargetInst, R: TargetReg, RA: RegisterAllocator<R>>(f: &mut Func<I>, ra: &mut RA) {
    for &block in f.blocks {

    }
}
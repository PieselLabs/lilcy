use slotmap::{SecondaryMap, SlotMap};

use super::{block::{Block, BlockData}, inst::{Inst, InstData}, types::Type, value::{Value, ValueData}};

pub struct Signature {
    pub args: Vec<Type>,
    pub ret: Option<Type>,
}

pub struct Func {
    insts: SlotMap<Inst, InstData>,
    args: Vec<Value>,
    pub (super) blocks: SlotMap<Block, BlockData>,
    values: SlotMap<Value, ValueData>,
    inst_results: SecondaryMap<Inst, Value>,
    pub entry: Option<Block>,
    sig: Signature,
    name: String,
}

impl Func {
    pub fn new(name: String, sig: Signature) -> Self {
        let mut values =  SlotMap::with_key();

        let args = sig.args.iter().enumerate()
            .map(|(i, &typ)| values.insert(ValueData::Arg {idx: i as u16, typ }))
            .collect();

        Self {
            name,
            sig,
            args,
            values,
            entry: Default::default(),
            insts: SlotMap::with_key(),
            blocks: SlotMap::with_key(),
            inst_results: SecondaryMap::new(),
        }
    }

    pub (super) fn add_void_inst(&mut self, block: Block, inst: InstData) -> Inst {
        let inst = self.insts.insert(inst);
        self.blocks[block].insts.push_back(inst);
        inst
    }

    pub (super) fn add_inst(&mut self, block: Block, inst: InstData, res_type: Type) -> Inst {
        let inst = self.insts.insert(inst);
        self.blocks[block].insts.push_back(inst);
        let value = self.values.insert(ValueData::Inst { typ: res_type, inst });
        self.inst_results.insert(inst, value);
        inst
    }

    pub (super) fn add_block(&mut self) -> Block {
        let block = self.blocks.insert(BlockData::default());
        if self.entry.is_none() {
            self.entry = Some(block);
        }
        block
    }

    pub (super) fn get_inst_result(&self, inst: Inst) -> Option<Value> {
        if let Some(val) = self.inst_results.get(inst) {
            Some(*val)
        } else {
            None
        }
    }

    pub fn value_data(&self, value: Value) -> &ValueData {
        &self.values[value]
    }

    pub fn get_arg(&self, idx: usize) -> Value {
        self.args[idx]
    }
}


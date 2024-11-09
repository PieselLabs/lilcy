use super::{
    block::Block,
    func::Func,
    inst::{Binary, BinaryOpcode, InstData},
    types::Type,
    value::Value,
};

pub struct Builder<'f> {
    func: &'f mut Func,
    // TODO: should be possible to insert in the middle of the block
    insert_point: Option<Block>,
}

impl<'f> Builder<'f> {
    pub fn new(func: &'f mut Func) -> Self {
        Self {
            func,
            insert_point: None,
        }
    }

    pub fn add_block(&mut self) -> Block {
        self.func.add_block()
    }

    pub fn set_insert_point(&mut self, block: Block) {
        self.insert_point = Some(block);
    }

    fn emit_binary_inst(
        &mut self,
        opcode: BinaryOpcode,
        res_type: Type,
        args: [Value; 2],
    ) -> Value {
        let inst = self.func.add_inst(
            self.insert_point.unwrap(),
            InstData::Binary(Binary { opcode, args }),
            res_type,
        );
        self.func.get_inst_result(inst)
    }

    fn emit_binary_inst_same_types(&mut self, opcode: BinaryOpcode, args: [Value; 2]) -> Value {
        let lhs_type = self.func.value_data(args[0]).get_type();
        let rhs_type = self.func.value_data(args[1]).get_type();
        assert_eq!(lhs_type, rhs_type);
        self.emit_binary_inst(opcode, lhs_type, args)
    }

    pub fn get_arg(&self, idx: usize) -> Value {
        self.func.get_arg(idx)
    }

    pub fn add(&mut self, lhs: Value, rhs: Value) -> Value {
        self.emit_binary_inst_same_types(BinaryOpcode::Add, [lhs, rhs])
    }

    pub fn uadd(&mut self, lhs: Value, rhs: Value) -> Value {
        self.emit_binary_inst_same_types(BinaryOpcode::UAdd, [lhs, rhs])
    }
}

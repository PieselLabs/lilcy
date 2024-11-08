use lilcy::ir::{builder::Builder, func::{Func, Signature}, types::Type};

fn main() {
    let mut func = Func::new("func".to_string(), Signature{args: vec![Type::I32, Type::I32], ret: Some(Type::I32) });
    let mut builder = Builder::new(&mut func);

    let entry = builder.add_block();
    builder.set_insert_point(entry);

    builder.add(builder.get_arg(0), builder.get_arg(1));
}

use crate::runtime::parser::Instruction;
use std::env;

mod runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args.get(1);
    assert!(
        !path.is_none(),
        "No file specified.  I can not Ni without the sacred text!"
    );

    let instructions: Vec<Instruction> = runtime::parser::parse_input_file(path.unwrap());
    let mut interpreter = runtime::interpreter::Interpreter::new(
        runtime::storage::Storage::new(1024 * 16, false),
        instructions,
    );
    interpreter.run();
}

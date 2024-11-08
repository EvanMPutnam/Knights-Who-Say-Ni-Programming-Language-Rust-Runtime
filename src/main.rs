use crate::runtime::parser::Instruction;
use std::env;

mod runtime;

const TOTAL_STORAGE_DEFAULT: usize = 1024 * 16;

struct EnvironmentVariables {
    storage: usize,
    debug: bool,
}

fn parse_environment_vars() -> EnvironmentVariables {
    let debug = env::var("NI_DEBUG");
    let debug = if debug.is_ok() && debug.unwrap().to_lowercase().eq("true") {
        true
    } else {
        false
    };

    let total_storage_bytes = env::var("NI_STORAGE");
    let mut total_storage = TOTAL_STORAGE_DEFAULT;
    if total_storage_bytes.is_ok() {
        let storage = total_storage_bytes.unwrap().parse::<usize>();
        if storage.is_ok() {
            total_storage = storage.unwrap();
        }
    }
    EnvironmentVariables {
        debug,
        storage: total_storage,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args.get(1);
    assert!(
        !path.is_none(),
        "No file specified.  I can not Ni without the sacred text!"
    );

    let environment_vars = parse_environment_vars();

    let instructions: Vec<Instruction> = runtime::parser::parse_input_file(path.unwrap());
    let mut interpreter = runtime::interpreter::Interpreter::new(
        runtime::storage::Storage::new(environment_vars.storage, environment_vars.debug),
        instructions,
    );
    interpreter.run();
}

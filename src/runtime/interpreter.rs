use crate::runtime::parser::Instruction;
use crate::runtime::storage::Storage;
use std::io::{stdin, Read};

pub struct Interpreter {
    storage: Storage,
    program: Vec<Instruction>,
    instruction_pointer: usize,
}

// TODO - Write interpreter for the language.
// https://martin-ueding.de/posts/creating-a-brainfuck-interpreter/
// https://thesharperdev.com/how-to-write-a-brainfuck-interpreter-in-c/

impl Interpreter {
    pub fn new(storage: Storage, instructions: Vec<Instruction>) -> Self {
        Interpreter {
            storage,
            program: instructions,
            instruction_pointer: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.program.get(self.instruction_pointer);
            if instruction.is_none() {
                // Program has finished!
                return;
            }
            let instruction = instruction.unwrap();
            let increment_instruction = true;
            // TODO --> Let instructions be "interface" with an exec method.
            match instruction {
                Instruction::IncrementDp => {
                    self.storage.increment_pointer();
                }
                Instruction::DecrementDp => {
                    self.storage.decrement_pointer();
                }
                Instruction::IncrementByteAtDp => {
                    let current_dp = self.storage.get_current_data_pointer();
                    let incremented_byte = self.storage.get(current_dp).overflowing_add(1);
                    self.storage.set(current_dp, incremented_byte.0);
                }
                Instruction::DecrementByteAtDp => {
                    let current_dp = self.storage.get_current_data_pointer();
                    let incremented_byte = self.storage.get(current_dp).overflowing_sub(1);
                    self.storage.set(current_dp, incremented_byte.0);
                }
                Instruction::OutputByteAtDp => {
                    self.storage.output_byte_at_pointer();
                }
                Instruction::AcceptByteAndStoreAtDp => {
                    let mut character = [0];
                    let current_dp = self.storage.get_current_data_pointer();
                    let read_char = stdin().read(&mut character);
                    assert!(read_char.is_ok(), "Cannot read character");
                    self.storage.set(current_dp, character[0]);
                }
                Instruction::JumpForwardIfDpZero => {
                    let current_dp = self.storage.get_current_data_pointer();
                    let current_byte = self.storage.get(current_dp);
                    if current_byte == 0 {
                        self.jump(true);
                    }
                }
                Instruction::JumpBackIfDpIsNonZero => {
                    let current_dp = self.storage.get_current_data_pointer();
                    let current_byte = self.storage.get(current_dp);
                    if current_byte != 0 {
                        self.jump(false);
                    }
                }
            }
            if increment_instruction {
                self.instruction_pointer += 1;
            }
        }
    }

    fn jump(&mut self, forward: bool) {
        let bracket = if forward {
            Instruction::JumpForwardIfDpZero
        } else {
            Instruction::JumpBackIfDpIsNonZero
        };
        let target = if forward {
            Instruction::JumpBackIfDpIsNonZero
        } else {
            Instruction::JumpForwardIfDpZero
        };

        let mut bracket_counter = 0;
        loop {
            let instruction = self.program.get(self.instruction_pointer).unwrap();
            if *instruction == bracket {
                bracket_counter += 1;
            } else if *instruction == target {
                bracket_counter -= 1;
                if bracket_counter == 0 {
                    break;
                }
            }
            self.instruction_pointer = if forward {
                self.instruction_pointer.overflowing_add(1).0
            } else {
                self.instruction_pointer.overflowing_sub(1).0
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let mut instructions: Vec<Instruction> = Vec::new();
        // 2
        instructions.push(Instruction::IncrementByteAtDp);
        instructions.push(Instruction::IncrementByteAtDp);

        // Next
        instructions.push(Instruction::IncrementDp);

        // 5
        instructions.push(Instruction::IncrementByteAtDp);
        instructions.push(Instruction::IncrementByteAtDp);
        instructions.push(Instruction::IncrementByteAtDp);
        instructions.push(Instruction::IncrementByteAtDp);
        instructions.push(Instruction::IncrementByteAtDp);

        // Adding loop
        instructions.push(Instruction::JumpForwardIfDpZero);
        instructions.push(Instruction::DecrementDp);
        instructions.push(Instruction::IncrementByteAtDp);
        instructions.push(Instruction::IncrementDp);
        instructions.push(Instruction::DecrementByteAtDp);
        instructions.push(Instruction::JumpBackIfDpIsNonZero);

        let mut interpreter = Interpreter::new(Storage::new(1024, false), instructions);
        interpreter.run();
        let v1 = interpreter.storage.get(0);
        let v2 = interpreter.storage.get(1);
        assert_eq!(v1, 7);
        assert_eq!(v2, 0);
    }
}

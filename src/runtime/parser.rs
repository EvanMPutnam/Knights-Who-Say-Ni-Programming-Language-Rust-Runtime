use regex_tokenizer::tokenizer;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    IncrementDp,
    DecrementDp,
    IncrementByteAtDp,
    DecrementByteAtDp,
    OutputByteAtDp,
    AcceptByteAndStoreAtDp,
    JumpForwardIfDpZero,
    JumpBackIfDpIsNonZero,
}

impl From<NiTokenizer_types> for Instruction {
    fn from(token_type: NiTokenizer_types) -> Self {
        match token_type {
            NiTokenizer_types::IncrementDp => Instruction::IncrementDp,
            NiTokenizer_types::DecrementDp => Instruction::DecrementDp,
            NiTokenizer_types::IncrementByteAtDp => Instruction::IncrementByteAtDp,
            NiTokenizer_types::DecrementByteAtDp => Instruction::DecrementByteAtDp,
            NiTokenizer_types::OutputByteAtDp => Instruction::OutputByteAtDp,
            NiTokenizer_types::AcceptByteAndStoreAtDp => Instruction::AcceptByteAndStoreAtDp,
            NiTokenizer_types::JumpForwardIfDpZero => Instruction::JumpForwardIfDpZero,
            NiTokenizer_types::JumpBackIfDpIsNonZero => Instruction::JumpBackIfDpIsNonZero,
            _ => panic!(),
        }
    }
}

tokenizer! {
    NiTokenizer
    r"Niiii+" => Error1
    r"niiii+" => Error2
    r"Niii" => JumpForwardIfDpZero
    r"niii" => JumpBackIfDpIsNonZero
    r"Nii" => OutputByteAtDp
    r"nii" => AcceptByteAndStoreAtDp
    r"Ni!" => IncrementByteAtDp
    r"ni!" => DecrementByteAtDp
    r"Ni" => IncrementDp
    r"ni" => DecrementDp
    r"\s+" => WhiteSpace
    r"\S+" => Other
}

fn validate_instructions(instructions: &Vec<Instruction>) {
    let l_vals = instructions
        .iter()
        .filter(|&x| *x == Instruction::JumpForwardIfDpZero)
        .count();
    let r_vals = instructions
        .iter()
        .filter(|&x| *x == Instruction::JumpBackIfDpIsNonZero)
        .count();
    assert_eq!(l_vals, r_vals, "You are not Ni-ing correctly!");
}

fn parse_str(script: &str) -> Vec<Instruction> {
    let tokenizer = NiTokenizer::new();
    let mut instructions = Vec::new();
    let mut tokens = tokenizer.tokenize(script);
    loop {
        let token = tokens.next();
        if token.is_none() {
            break;
        }
        let token = token.unwrap();
        if token.type_ == NiTokenizer_types::WhiteSpace {
            continue;
        }

        let is_error_token = token.type_ == NiTokenizer_types::Other
            || token.type_ == NiTokenizer_types::Error1
            || token.type_ == NiTokenizer_types::Error2;
        assert!(
            !is_error_token,
            "Invalid token {}.  You must correctly ni!",
            token.value
        );

        let instruction = Instruction::from(token.type_);
        instructions.push(instruction);
    }
    validate_instructions(&instructions);
    instructions
}
pub fn parse_input_file(file_path: &str) -> Vec<Instruction> {
    assert!(
        file_path.ends_with(".ni") || file_path.ends_with(".nii"),
        "Ni! Use the right file extension for {}",
        file_path
    );
    let file_text = fs::read_to_string(&file_path);
    assert!(file_text.is_ok(), "File not found: {}.", file_path);
    parse_str(file_text.unwrap().as_str())
}

#[cfg(test)]
mod tests {
    use crate::runtime::parser::{parse_str, Instruction};

    #[test]
    fn test_parse_str() {
        let instructions = parse_str("Nii Ni");
        assert_eq!(instructions.len(), 2);
        assert_eq!(*instructions.get(0).unwrap(), Instruction::OutputByteAtDp);
        assert_eq!(*instructions.get(1).unwrap(), Instruction::IncrementDp);
    }
}

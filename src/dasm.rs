pub const UNINITIALIZED_OPCODE:u32 = 0;
pub const VALID_OPCODE:u32 = 1;
pub const UNDOC_OPCODE:u32 = 1 << 2;
pub const BRANCH_OPCODE:u32 = 1 << 3;
pub const SUBROUTINE_OPCODE:u32 = 1 << 4;
pub const JMP_OPCODE:u32 = 1 << 5;
pub const INVALID_OPCODE:u32 = 1 << 6;
pub const DATA_FLAG:u32 = 1 << 7;
pub const LOAD_FLAG:u32 = 1 << 8;
pub const STORE_FLAG:u32 = 1 << 9;

#[derive(Debug, Default)]
pub enum LineType {
    #[default] None,
    Instruction,
    UnknownInstruction,
    Comment,
}

#[derive(Debug, Default)]
pub struct DisassembledLine {
    pub line_type: LineType,
    pub instr_size: u8,
    pub address: u32,
    pub address_ref: u32,
    offset: u32,
    return_address: u32,
    pub output: String,
    label: String,
    comment: String,
    pub flags: u32
}

impl DisassembledLine {
    pub fn new() -> Self {
        DisassembledLine {
            line_type: LineType::None,
            instr_size: 0,
            address: 0,
            address_ref: 0,
            offset: 0,
            return_address: 0,
            output: String::from(""),
            label: String::from(""),
            comment: String::from(""),
            flags: UNINITIALIZED_OPCODE
        }
    }

    pub fn from(line_type:LineType, address:u32, offset: u32, label: &str) -> Self {
        DisassembledLine {
            line_type,
            instr_size: 0,
            address,
            address_ref: 0,
            offset,
            return_address: 0,
            output: String::from(""),
            label: String::from(label),
            comment: String::from(""),
            flags: UNINITIALIZED_OPCODE
        }
    }

}

pub trait Disassembler {
    fn disassemble(&self, offset:usize, code: &[u8]) -> Vec<DisassembledLine>;
}

#[cfg(test)]
mod dasm_test {
    use crate::dasm::DisassembledLine;

    #[test]
    fn test_line() {
        let line = DisassembledLine::new();
        println!("{}",line.address);
        assert_eq!(0, line.address);
    }
}


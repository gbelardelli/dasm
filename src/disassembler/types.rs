use super::UNINITIALIZED_OPCODE;

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
    pub address: u64,
    pub address_ref: u64,
    offset: u64,
    return_address: u64,
    pub byte_code: [u8;16],
    pub opcode: String,
    pub operand: String,
    pub operand_address: u64,
    label: String,
    pub comment: String,
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
            operand_address: 0,
            operand: String::from(""),
            opcode: String::from(""),
            byte_code: [0; 16],
            label: String::from(""),
            comment: String::from(""),
            flags: UNINITIALIZED_OPCODE
        }
    }
}

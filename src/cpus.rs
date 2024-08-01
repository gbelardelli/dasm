use crate::{dasm::DisassembledLine, memory::BinaryBuffer};



pub mod mos65xx;



type Mnemonic = &'static str;

const UNINITIALIZED_OPCODE:u32 = 0;
const VALID_OPCODE:u32 = 1;
const UNDOC_OPCODE:u32 = 1 << 2;
const BRANCH_OPCODE:u32 = 1 << 3;
const SUBROUTINE_OPCODE:u32 = 1 << 4;
const JMP_OPCODE:u32 = 1 << 5;
const INVALID_OPCODE:u32 = 1 << 6;
const DATA_FLAG:u32 = 1 << 7;
const LOAD_FLAG:u32 = 1 << 8;
const STORE_FLAG:u32 = 1 << 9;

pub trait CpuTrait {
    fn fetch_and_decode(&mut self) -> DisassembledLine;
}

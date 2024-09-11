use std::collections::HashMap;

use types::DisassembledLine;
use crate::cpus::CpuTrait;

pub mod types;
mod dasm;

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
pub const INDIRECT_FLAG:u32 = 1 << 10;

pub struct Dasm {
    cpu: Box<dyn CpuTrait>,
    labels_map: HashMap<u32, String>,
    lines: Vec<DisassembledLine>,
    relative_label_id: u32,
    subroutine_label_id: u32,
    jump_label_id: u32,
    start_pc:u32
}

pub trait DisassemblerTrait {
    fn disassemble_next(&mut self) -> Option<DisassembledLine>;
}


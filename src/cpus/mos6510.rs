use addressing::AddressingMode;
use opcodes6510::OPCODES_TABLE;
use crate::dasm::{SUBROUTINE_OPCODE, UNDOC_OPCODE};
use super::CpuTrait;
use crate::{dasm::{DisassembledLine, LineType}, memory::BinaryBuffer};

mod opcodes6510;
mod addressing;
mod disassembler;

#[derive(Debug,Default)]
pub struct Cpu6510 {
    pc: u16,
    memory: BinaryBuffer
}


impl CpuTrait for Cpu6510 {
    fn set_pc(&mut self, pc:u32) {
        self.pc = pc as u16;
    }
}


impl Cpu6510 {
    pub fn new(memory: BinaryBuffer) -> Self {
        Self {
            pc: memory.get_loaded_address() as u16,
            memory,
        }
    }
}

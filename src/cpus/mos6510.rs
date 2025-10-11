

use super::CpuTrait;
use crate::memory::BinaryBuffer;

mod opcodes6510;
mod addressing;
mod disassembler;

#[derive(Debug,Default)]
pub struct Cpu6510 {
    pc: u64,
    memory: BinaryBuffer
}


impl CpuTrait for Cpu6510 {
    fn set_program_counter(&mut self, pc:u64) -> Result<i32,i32> {
        if pc <= u16::MAX as u64{
            self.pc = pc;
            return Ok(0);
        }
        Err(1)
    }
}


impl Cpu6510 {
    pub fn new(memory: BinaryBuffer) -> Self {
        Self {
            pc: memory.get_loaded_address(),
            memory,
        }
    }
}

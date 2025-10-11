use crate::disassembler::DisassemblerTrait;

pub mod mos6510;
pub mod mc680x0;

type Mnemonic = &'static str;

pub trait CpuTrait : DisassemblerTrait {
    fn set_program_counter(&mut self, pc:u64) -> Result<i32,i32>;
}

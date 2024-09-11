use crate::disassembler::DisassemblerTrait;

pub mod mos6510;

type Mnemonic = &'static str;

pub trait CpuTrait : DisassemblerTrait {
    fn set_pc(&mut self, pc:u32);
}

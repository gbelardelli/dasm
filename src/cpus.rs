use crate::{dasm::DisassembledLine, memory::BinaryBuffer};



pub mod mos6510;




type Mnemonic = &'static str;




pub trait CpuTrait {
    fn fetch_and_decode(&mut self) -> DisassembledLine;
    fn set_pc(&mut self, pc:u32);
}

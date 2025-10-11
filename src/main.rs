use disassembler::Dasm;
use memory::BinaryBuffer;
use cpus::{mos6510::Cpu6510, CpuTrait};

pub mod utils;
mod disassembler;
mod cpus;
mod memory;


fn main() {
    let bytes = std::fs::read("./kernal.901486-07-vic20.bin").unwrap();
    let memory:BinaryBuffer = BinaryBuffer::new(bytes, 0xE000);
    let cpu: Box<dyn CpuTrait> = Box::new(Cpu6510::new(memory));

    let mut dasm = Dasm::new(cpu,0x0);
    dasm.run();
}

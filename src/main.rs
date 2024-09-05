#![allow(dead_code)]


use memory::BinaryBuffer;
use cpus::{mos6510::Cpu6510, CpuTrait};

pub mod utils;
mod dasm;
mod cpus;
mod memory;





struct MyStruct {
    cpu: Box<dyn CpuTrait>,
    start_offset:u32
}

impl MyStruct {
    pub fn new(cpu: Box<dyn CpuTrait>, start_offset:u32) -> Self {
        MyStruct {
            cpu,
            start_offset
        }
    }

    pub fn run(&mut self) {
        self.cpu.set_pc(self.start_offset);

        for n in 0..63 {
            let line = self.cpu.disassemble_next();

            let mut output_line= "".to_owned();

            output_line.push_str(&format!("{:04X}    ",line.address));
            
            for i in 0..line.instr_size+1 {
                output_line.push_str(&format!("{:02X} ",line.byte_code[i as usize]));
            }
            if line.instr_size == 0 {
                output_line.push_str("      ");
            }else if line.instr_size == 1 {
                output_line.push_str("   ");
            }
            output_line.push_str(&format!("{} ", line.opcode));
            output_line.push_str(&format!("{} ", line.operand));

            println!("{}", output_line);
        }
    }    
}


fn main() {
    let bytes = std::fs::read("./basic-901226-01.bin").unwrap();
    let memory:BinaryBuffer = BinaryBuffer::new(bytes, 0xA000);
    //let mut cpu = Cpu6510::new(memory);

    //let mut dasm:Disassembler = Disassembler::new(&mut cpu, 0x38a);
    //dasm.run();
    let cpu: Box<dyn CpuTrait> = Box::new(Cpu6510::new(memory));
    let mut dasm = MyStruct::new(cpu,0x38a);

    dasm.run();

    //let cpu:Cpu65xx = Cpu65xx::new(memory);
    //cpu.fetch_and_decode();

    /*let mut t = Instant::now();
    let refs:AsciiReferences = extract_ascii_references(&bytes, 4);
    println!("Elapsed: {:?}", t.elapsed());


    let string1 = String::from("Stringa1");
    let string2 = String::from("Stringa2");
    {
        let string3 = String::from("Stringa3");
        let result = longest(string1.as_str(), string3.as_str());
        println!("The longest string is {}", result);
    }*/

}
/*
fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else{
        y
    }
}*/

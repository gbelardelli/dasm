use std::{result, time::Instant};
use dasm::{DasmTrait, DisassembledLine};
use memory::BinaryBuffer;
use utils::{AsciiReferences, extract_ascii_references};
use cpus::{mos65xx::Cpu65xx, CpuTrait};

pub mod utils;
mod dasm;
mod cpus;
mod memory;

struct Disassembler<'a> {
    cpu: &'a mut dyn CpuTrait,
}

impl<'a> Disassembler<'a> {
    pub fn new(cpu: &'a mut dyn CpuTrait) -> Self {
        Disassembler {
            cpu
        }
    }

    pub fn run(&mut self) {
        self.cpu.fetch_and_decode();
    }
}
fn main() {
    let line = DisassembledLine::new();
    let line2 = DisassembledLine::default();
    let line3 = DisassembledLine::from(dasm::LineType::Comment, 0x33, 0x2, "");
    
    println!("line: {:?}",line);
    println!("line2: {:?}",line2);
    println!("line3: {:?}",line3);


    let bytes = std::fs::read("./MANIC MINER.prg").unwrap();
    let memory:BinaryBuffer = BinaryBuffer::new(bytes, 0);
    let mut cpu = Cpu65xx::new(memory);

    let mut cazzo:Disassembler = Disassembler::new(&mut cpu);
    cazzo.run();

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

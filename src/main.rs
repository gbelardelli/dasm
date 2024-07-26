use std::time::Instant;
use dasm::{DasmTrait, DisassembledLine};
use utils::{AsciiReferences, extract_ascii_references};
use cpus::{mos65xx::Cpu65xx};

pub mod utils;
mod dasm;

mod cpus;

fn main() {
    let line = DisassembledLine::new();
    let line2 = DisassembledLine::default();
    let line3 = DisassembledLine::from(dasm::LineType::Comment, 0x33, 0x2, "");
    
    println!("line: {:?}",line);
    println!("line2: {:?}",line2);
    println!("line3: {:?}",line3);


    let bytes = std::fs::read("./ter.prg").unwrap();

    let cpu:Cpu65xx = Cpu65xx::new();
    cpu.fetch_and_decode(&bytes);

    let mut t = Instant::now();
    let refs:AsciiReferences = extract_ascii_references(&bytes, 4);
    println!("Elapsed: {:?}", t.elapsed());

    /*for refee in refs {
        println!("Elapsed: {:?}", refee);
    }*/
}


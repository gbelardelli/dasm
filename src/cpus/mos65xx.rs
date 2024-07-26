
#[derive(Debug,Default)]
pub struct Cpu65xx {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,

    pc: u16,

    n: bool,
    v: bool,
    d: bool,
    i: bool,
    z: bool,
    c: bool,
}

const UNINITIALIZED_OPCODE:u32 = 0;
const VALID_OPCODE:u32 = 1;
const UNDOC_OPCODE:u32 = 1 << 2;
const BRANCH_OPCODE:u32 = 1 << 3;
const SUBROUTINE_OPCODE:u32 = 1 << 4;
const JMP_OPCODE:u32 = 1 << 5;
const INVALID_OPCODE:u32 = 1 << 6;
const DATA_FLAG:u32 = 1 << 7;
const LOAD_FLAG:u32 = 1 << 8;
const STORE_FLAG:u32 = 1 << 9;

enum AddressingMode {
    AddrImplied,
    AddrAccumulator,
    AddrImmediate,
    AddrZeroPage,
    AddrZeroPageX,
    AddrZeroPageY,
    AddrAbsolute,
    AddrAbsoluteX,
    AddrAbsoluteY,
    AddrIndirectX,
    AddrIndirectY,
    AddrRelative,
    AddrDirect,
    AddrIndirect
}
struct Opcode {
    //opcode: u8,
    addressing: AddressingMode,
    mnemonic: Mnemonic,
    flags: u32
}

use super::{Mnemonic};
use crate::dasm::{DasmTrait, DisassembledLine};

const OPCODES_TABLE:&'static [Opcode] = &[
    Opcode{ addressing:AddressingMode::AddrAbsolute, mnemonic:"XOR", flags:VALID_OPCODE}
];

impl DasmTrait for Cpu65xx {
    fn fetch_and_decode(&self, bytes: &Vec<u8>) -> DisassembledLine {
        let fetchedOpcode:u8 = bytes[self.pc as usize];
        let opcode=&OPCODES_TABLE[0];
        println!("tadaaaaa: {}",self.pc);
        DisassembledLine::default()
    }
}

impl Cpu65xx {
    pub fn new() -> Cpu65xx {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0x0000,
            n: false,
            v: false,
            d: false,
            i: false,
            z: false,
            c: false,
        }
    }
}

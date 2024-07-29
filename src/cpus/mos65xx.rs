
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

    memory: BinaryBuffer,
}


#[derive(Debug)]
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
#[derive(Debug)]
struct Opcode {
    //opcode: u8,
    addressing: AddressingMode,
    mnemonic: Mnemonic,
    flags: u32
}

use super::{Mnemonic, BRANCH_OPCODE, INVALID_OPCODE, JMP_OPCODE, SUBROUTINE_OPCODE, UNDOC_OPCODE, VALID_OPCODE};
use crate::{dasm::{DasmTrait, DisassembledLine}, memory::BinaryBuffer};

const OPCODES_TABLE:&'static [Opcode] = &[
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"BRK", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE},
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"SLO", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"NOP", flags:UNDOC_OPCODE},
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"ASL", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"SLO", flags:UNDOC_OPCODE  },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"PHP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAccumulator,mnemonic:"ASL", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"ANC", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"ASL", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"SLO", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrRelative,   mnemonic:"BPL", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"SLO", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ASL", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"SLO", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"CLC", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"SLO", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ASL", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"SLO", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"JSR", flags:VALID_OPCODE|SUBROUTINE_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"RLA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"BIT", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"ROL", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"RLA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"PLP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"AND", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrAccumulator,mnemonic:"ROL", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"ANC", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"BIT", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"ROL", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"RLA", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrRelative,   mnemonic:"BMI", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"RLA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ROL", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"RLA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"SEC", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"RLA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ROL", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"RLA", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"RTI", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"SRE", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"LSR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"SRE", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"PHA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAccumulator,mnemonic:"LSR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"ALR", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"JMP", flags:VALID_OPCODE|JMP_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"LSR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"SRE", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrRelative,   mnemonic:"BVC", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"SRE", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"LSR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"SRE", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"CLI", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"SRE", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"LSR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"SRE", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"RTS", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"RRA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"ROR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"RRA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"PLA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAccumulator,mnemonic:"ROR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"ARR", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirect,   mnemonic:"JMP", flags:VALID_OPCODE|JMP_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"ROR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"RRA", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrRelative,   mnemonic:"BVS", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"RRA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ROR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"RRA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"SEI", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"RRA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ROR", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"RRA", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"NOP", flags:INVALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"SAX", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"STY", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"STX", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"SAX", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"DEY", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"TXA", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"ANE", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"STY", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"STX", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"SAX", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrRelative,   mnemonic:"BCC", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"SHA", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"STY", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageY,  mnemonic:"STX", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageY,  mnemonic:"SAX", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"TYA", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"TXS", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"TAS", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"SHY", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"SHX", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"SHA", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"LDY", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"LDX", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"LAX", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"LDY", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"LDX", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"LAX", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"TAY", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"LDA", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"TAX", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"LAX", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"LDY", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"LDX", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"LAX", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrRelative,   mnemonic:"BCS", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"LAX", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"LDY", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageY,  mnemonic:"LDX", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageY,  mnemonic:"LAX", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"CLV", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"TSX", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"LAS", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"LDY", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"LDX", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"LAX", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"CPY", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"DCP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"CPY", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"DEC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"DCP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"INY", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"DEX", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"SBX", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"CPY", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"DEC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"DCP", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrRelative,   mnemonic:"BNE", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"DCP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"DEC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"DCP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"CLD", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"DCP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"DEC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"DCP", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"CPX", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectX,  mnemonic:"ISC", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"CPX", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"INC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPage,   mnemonic:"ISC", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"INX", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImmediate,  mnemonic:"SBC", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"CPX", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"INC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsolute,   mnemonic:"ISC", flags:UNDOC_OPCODE },

    Opcode{ addressing:AddressingMode::AddrRelative,   mnemonic:"BEQ", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrIndirectY,  mnemonic:"ISC", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"INC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ISC", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"SED", flags:VALID_OPCODE }, //
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"ISC", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"INC", flags:VALID_OPCODE },
    Opcode{ addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ISC", flags:UNDOC_OPCODE }

];

impl DasmTrait for Cpu65xx {
    fn fetch_and_decode(&self) -> DisassembledLine {
        let fetched_opcode:u8 = self.memory.read_byte(self.pc as u32);
        println!("fetched: {}",fetched_opcode);
        let opcode=&OPCODES_TABLE[fetched_opcode as usize];
        println!("tadaaaaa: {:#?}",opcode);
        DisassembledLine::default()
    }
}

impl Cpu65xx {
    pub fn new(memory: BinaryBuffer) -> Cpu65xx {
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
            memory
        }
    }
}

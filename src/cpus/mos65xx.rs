
#[derive(Debug,Default)]
pub struct Cpu65xx {
    pc: u16,
    memory: Option<BinaryBuffer>,
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
pub struct Opcode {
    opcode: u32,
    addressing: AddressingMode,
    mnemonic: Mnemonic,
    flags: u32
}
use super::{CpuTrait, Mnemonic, BRANCH_OPCODE, INVALID_OPCODE, JMP_OPCODE, SUBROUTINE_OPCODE, UNDOC_OPCODE, VALID_OPCODE};
use crate::{dasm::{DisassembledLine, Disassembler}, memory::BinaryBuffer};

const OPCODES_TABLE:&'static [Opcode] = &[
    Opcode{ opcode: 0x00, addressing:AddressingMode::AddrImplied,    mnemonic:"BRK", flags:VALID_OPCODE },
    Opcode{ opcode: 0x01, addressing:AddressingMode::AddrIndirectX,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x02, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x03, addressing:AddressingMode::AddrIndirectX,  mnemonic:"SLO", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x04, addressing:AddressingMode::AddrZeroPage,   mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x05, addressing:AddressingMode::AddrZeroPage,   mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x06, addressing:AddressingMode::AddrZeroPage,   mnemonic:"ASL", flags:VALID_OPCODE },
    Opcode{ opcode: 0x07, addressing:AddressingMode::AddrZeroPage,   mnemonic:"SLO", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x08, addressing:AddressingMode::AddrImplied,    mnemonic:"PHP", flags:VALID_OPCODE },
    Opcode{ opcode: 0x09, addressing:AddressingMode::AddrImmediate,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x0A, addressing:AddressingMode::AddrAccumulator,mnemonic:"ASL", flags:VALID_OPCODE },
    Opcode{ opcode: 0x0B, addressing:AddressingMode::AddrImmediate,  mnemonic:"ANC", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x0C, addressing:AddressingMode::AddrAbsolute,   mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x0D, addressing:AddressingMode::AddrAbsolute,   mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x0E, addressing:AddressingMode::AddrAbsolute,   mnemonic:"ASL", flags:VALID_OPCODE },
    Opcode{ opcode: 0x0F, addressing:AddressingMode::AddrAbsolute,   mnemonic:"SLO", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0x10, addressing:AddressingMode::AddrRelative,   mnemonic:"BPL", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ opcode: 0x11, addressing:AddressingMode::AddrIndirectY,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x12, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x13, addressing:AddressingMode::AddrIndirectY,  mnemonic:"SLO", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x14, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x15, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x16, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ASL", flags:VALID_OPCODE },
    Opcode{ opcode: 0x17, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"SLO", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x18, addressing:AddressingMode::AddrImplied,    mnemonic:"CLC", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0x19, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x1A, addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x1B, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"SLO", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x1C, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x1D, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ORA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x1E, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ASL", flags:VALID_OPCODE },
    Opcode{ opcode: 0x1F, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"SLO", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0x20, addressing:AddressingMode::AddrAbsolute,   mnemonic:"JSR", flags:VALID_OPCODE|SUBROUTINE_OPCODE },
    Opcode{ opcode: 0x21, addressing:AddressingMode::AddrIndirectX,  mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ opcode: 0x22, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x23, addressing:AddressingMode::AddrIndirectX,  mnemonic:"RLA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x24, addressing:AddressingMode::AddrZeroPage,   mnemonic:"BIT", flags:VALID_OPCODE },
    Opcode{ opcode: 0x25, addressing:AddressingMode::AddrZeroPage,   mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ opcode: 0x26, addressing:AddressingMode::AddrZeroPage,   mnemonic:"ROL", flags:VALID_OPCODE },
    Opcode{ opcode: 0x27, addressing:AddressingMode::AddrZeroPage,   mnemonic:"RLA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x28, addressing:AddressingMode::AddrImplied,    mnemonic:"PLP", flags:VALID_OPCODE },
    Opcode{ opcode: 0x29, addressing:AddressingMode::AddrImmediate,  mnemonic:"AND", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0x2A, addressing:AddressingMode::AddrAccumulator,mnemonic:"ROL", flags:VALID_OPCODE },
    Opcode{ opcode: 0x2B, addressing:AddressingMode::AddrImmediate,  mnemonic:"ANC", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x2C, addressing:AddressingMode::AddrAbsolute,   mnemonic:"BIT", flags:VALID_OPCODE },
    Opcode{ opcode: 0x2D, addressing:AddressingMode::AddrAbsolute,   mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ opcode: 0x2E, addressing:AddressingMode::AddrAbsolute,   mnemonic:"ROL", flags:VALID_OPCODE },
    Opcode{ opcode: 0x2F, addressing:AddressingMode::AddrAbsolute,   mnemonic:"RLA", flags:UNDOC_OPCODE },
					
    Opcode{ opcode: 0x30, addressing:AddressingMode::AddrRelative,   mnemonic:"BMI", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ opcode: 0x31, addressing:AddressingMode::AddrIndirectY,  mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ opcode: 0x32, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x33, addressing:AddressingMode::AddrIndirectY,  mnemonic:"RLA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x34, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x35, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ opcode: 0x36, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ROL", flags:VALID_OPCODE },
    Opcode{ opcode: 0x37, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"RLA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x38, addressing:AddressingMode::AddrImplied,    mnemonic:"SEC", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0x39, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ opcode: 0x3A, addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x3B, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"RLA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x3C, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x3D, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"AND", flags:VALID_OPCODE },
    Opcode{ opcode: 0x3E, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ROL", flags:VALID_OPCODE },
    Opcode{ opcode: 0x3F, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"RLA", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0x40, addressing:AddressingMode::AddrImplied,    mnemonic:"RTI", flags:VALID_OPCODE },
    Opcode{ opcode: 0x41, addressing:AddressingMode::AddrIndirectX,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x42, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x43, addressing:AddressingMode::AddrIndirectX,  mnemonic:"SRE", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x44, addressing:AddressingMode::AddrZeroPage,   mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x45, addressing:AddressingMode::AddrZeroPage,   mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x46, addressing:AddressingMode::AddrZeroPage,   mnemonic:"LSR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x47, addressing:AddressingMode::AddrZeroPage,   mnemonic:"SRE", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x48, addressing:AddressingMode::AddrImplied,    mnemonic:"PHA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x49, addressing:AddressingMode::AddrImmediate,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x4A, addressing:AddressingMode::AddrAccumulator,mnemonic:"LSR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x4B, addressing:AddressingMode::AddrImmediate,  mnemonic:"ALR", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x4C, addressing:AddressingMode::AddrAbsolute,   mnemonic:"JMP", flags:VALID_OPCODE|JMP_OPCODE },
    Opcode{ opcode: 0x4D, addressing:AddressingMode::AddrAbsolute,   mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x4E, addressing:AddressingMode::AddrAbsolute,   mnemonic:"LSR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x4F, addressing:AddressingMode::AddrAbsolute,   mnemonic:"SRE", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0x50, addressing:AddressingMode::AddrRelative,   mnemonic:"BVC", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ opcode: 0x51, addressing:AddressingMode::AddrIndirectY,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x52, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x53, addressing:AddressingMode::AddrIndirectY,  mnemonic:"SRE", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x54, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x55, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x56, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"LSR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x57, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"SRE", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x58, addressing:AddressingMode::AddrImplied,    mnemonic:"CLI", flags:VALID_OPCODE },
    Opcode{ opcode: 0x59, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x5A, addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x5B, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"SRE", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x5C, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x5D, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"EOR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x5E, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"LSR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x5F, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"SRE", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0x60, addressing:AddressingMode::AddrImplied,    mnemonic:"RTS", flags:VALID_OPCODE },
    Opcode{ opcode: 0x61, addressing:AddressingMode::AddrIndirectX,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ opcode: 0x62, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x63, addressing:AddressingMode::AddrIndirectX,  mnemonic:"RRA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x64, addressing:AddressingMode::AddrZeroPage,   mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x65, addressing:AddressingMode::AddrZeroPage,   mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ opcode: 0x66, addressing:AddressingMode::AddrZeroPage,   mnemonic:"ROR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x67, addressing:AddressingMode::AddrZeroPage,   mnemonic:"RRA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x68, addressing:AddressingMode::AddrImplied,    mnemonic:"PLA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x69, addressing:AddressingMode::AddrImmediate,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ opcode: 0x6A, addressing:AddressingMode::AddrAccumulator,mnemonic:"ROR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x6B, addressing:AddressingMode::AddrImmediate,  mnemonic:"ARR", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x6C, addressing:AddressingMode::AddrIndirect,   mnemonic:"JMP", flags:VALID_OPCODE|JMP_OPCODE },
    Opcode{ opcode: 0x6D, addressing:AddressingMode::AddrAbsolute,   mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ opcode: 0x6E, addressing:AddressingMode::AddrAbsolute,   mnemonic:"ROR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x6F, addressing:AddressingMode::AddrAbsolute,   mnemonic:"RRA", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0x70, addressing:AddressingMode::AddrRelative,   mnemonic:"BVS", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ opcode: 0x71, addressing:AddressingMode::AddrIndirectY,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ opcode: 0x72, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x73, addressing:AddressingMode::AddrIndirectY,  mnemonic:"RRA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x74, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x75, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ opcode: 0x76, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ROR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x77, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"RRA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x78, addressing:AddressingMode::AddrImplied,    mnemonic:"SEI", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0x79, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ opcode: 0x7A, addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x7B, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"RRA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x7C, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x7D, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ADC", flags:VALID_OPCODE },
    Opcode{ opcode: 0x7E, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ROR", flags:VALID_OPCODE },
    Opcode{ opcode: 0x7F, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"RRA", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0x80, addressing:AddressingMode::AddrImmediate,  mnemonic:"NOP", flags:INVALID_OPCODE },
    Opcode{ opcode: 0x81, addressing:AddressingMode::AddrIndirectX,  mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x82, addressing:AddressingMode::AddrImmediate,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x83, addressing:AddressingMode::AddrIndirectX,  mnemonic:"SAX", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x84, addressing:AddressingMode::AddrZeroPage,   mnemonic:"STY", flags:VALID_OPCODE },
    Opcode{ opcode: 0x85, addressing:AddressingMode::AddrZeroPage,   mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x86, addressing:AddressingMode::AddrZeroPage,   mnemonic:"STX", flags:VALID_OPCODE },
    Opcode{ opcode: 0x87, addressing:AddressingMode::AddrZeroPage,   mnemonic:"SAX", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x88, addressing:AddressingMode::AddrImplied,    mnemonic:"DEY", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0x89, addressing:AddressingMode::AddrImmediate,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x8A, addressing:AddressingMode::AddrImplied,    mnemonic:"TXA", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0x8B, addressing:AddressingMode::AddrImmediate,  mnemonic:"ANE", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x8C, addressing:AddressingMode::AddrAbsolute,   mnemonic:"STY", flags:VALID_OPCODE },
    Opcode{ opcode: 0x8D, addressing:AddressingMode::AddrAbsolute,   mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x8E, addressing:AddressingMode::AddrAbsolute,   mnemonic:"STX", flags:VALID_OPCODE },
    Opcode{ opcode: 0x8F, addressing:AddressingMode::AddrAbsolute,   mnemonic:"SAX", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0x90, addressing:AddressingMode::AddrRelative,   mnemonic:"BCC", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ opcode: 0x91, addressing:AddressingMode::AddrIndirectY,  mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x92, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x93, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"SHA", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x94, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"STY", flags:VALID_OPCODE },
    Opcode{ opcode: 0x95, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x96, addressing:AddressingMode::AddrZeroPageY,  mnemonic:"STX", flags:VALID_OPCODE },
    Opcode{ opcode: 0x97, addressing:AddressingMode::AddrZeroPageY,  mnemonic:"SAX", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x98, addressing:AddressingMode::AddrImplied,    mnemonic:"TYA", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0x99, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x9A, addressing:AddressingMode::AddrImplied,    mnemonic:"TXS", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0x9B, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"TAS", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x9C, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"SHY", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x9D, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"STA", flags:VALID_OPCODE },
    Opcode{ opcode: 0x9E, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"SHX", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0x9F, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"SHA", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0xA0, addressing:AddressingMode::AddrImmediate,  mnemonic:"LDY", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xA1, addressing:AddressingMode::AddrIndirectX,  mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ opcode: 0xA2, addressing:AddressingMode::AddrImmediate,  mnemonic:"LDX", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xA3, addressing:AddressingMode::AddrIndirectX,  mnemonic:"LAX", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xA4, addressing:AddressingMode::AddrZeroPage,   mnemonic:"LDY", flags:VALID_OPCODE },
    Opcode{ opcode: 0xA5, addressing:AddressingMode::AddrZeroPage,   mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ opcode: 0xA6, addressing:AddressingMode::AddrZeroPage,   mnemonic:"LDX", flags:VALID_OPCODE },
    Opcode{ opcode: 0xA7, addressing:AddressingMode::AddrZeroPage,   mnemonic:"LAX", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xA8, addressing:AddressingMode::AddrImplied,    mnemonic:"TAY", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xA9, addressing:AddressingMode::AddrImmediate,  mnemonic:"LDA", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xAA, addressing:AddressingMode::AddrImplied,    mnemonic:"TAX", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xAB, addressing:AddressingMode::AddrImmediate,  mnemonic:"LAX", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xAC, addressing:AddressingMode::AddrAbsolute,   mnemonic:"LDY", flags:VALID_OPCODE },
    Opcode{ opcode: 0xAD, addressing:AddressingMode::AddrAbsolute,   mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ opcode: 0xAE, addressing:AddressingMode::AddrAbsolute,   mnemonic:"LDX", flags:VALID_OPCODE },
    Opcode{ opcode: 0xAF, addressing:AddressingMode::AddrAbsolute,   mnemonic:"LAX", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0xB0, addressing:AddressingMode::AddrRelative,   mnemonic:"BCS", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ opcode: 0xB1, addressing:AddressingMode::AddrIndirectY,  mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ opcode: 0xB2, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xB3, addressing:AddressingMode::AddrIndirectY,  mnemonic:"LAX", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xB4, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"LDY", flags:VALID_OPCODE },
    Opcode{ opcode: 0xB5, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ opcode: 0xB6, addressing:AddressingMode::AddrZeroPageY,  mnemonic:"LDX", flags:VALID_OPCODE },
    Opcode{ opcode: 0xB7, addressing:AddressingMode::AddrZeroPageY,  mnemonic:"LAX", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xB8, addressing:AddressingMode::AddrImplied,    mnemonic:"CLV", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xB9, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ opcode: 0xBA, addressing:AddressingMode::AddrImplied,    mnemonic:"TSX", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xBB, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"LAS", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xBC, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"LDY", flags:VALID_OPCODE },
    Opcode{ opcode: 0xBD, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"LDA", flags:VALID_OPCODE },
    Opcode{ opcode: 0xBE, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"LDX", flags:VALID_OPCODE },
    Opcode{ opcode: 0xBF, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"LAX", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0xC0, addressing:AddressingMode::AddrImmediate,  mnemonic:"CPY", flags:VALID_OPCODE },
    Opcode{ opcode: 0xC1, addressing:AddressingMode::AddrIndirectX,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ opcode: 0xC2, addressing:AddressingMode::AddrImmediate,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xC3, addressing:AddressingMode::AddrIndirectX,  mnemonic:"DCP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xC4, addressing:AddressingMode::AddrZeroPage,   mnemonic:"CPY", flags:VALID_OPCODE },
    Opcode{ opcode: 0xC5, addressing:AddressingMode::AddrZeroPage,   mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ opcode: 0xC6, addressing:AddressingMode::AddrZeroPage,   mnemonic:"DEC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xC7, addressing:AddressingMode::AddrZeroPage,   mnemonic:"DCP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xC8, addressing:AddressingMode::AddrImplied,    mnemonic:"INY", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xC9, addressing:AddressingMode::AddrImmediate,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ opcode: 0xCA, addressing:AddressingMode::AddrImplied,    mnemonic:"DEX", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xCB, addressing:AddressingMode::AddrImmediate,  mnemonic:"SBX", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xCC, addressing:AddressingMode::AddrAbsolute,   mnemonic:"CPY", flags:VALID_OPCODE },
    Opcode{ opcode: 0xCD, addressing:AddressingMode::AddrAbsolute,   mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ opcode: 0xCE, addressing:AddressingMode::AddrAbsolute,   mnemonic:"DEC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xCF, addressing:AddressingMode::AddrAbsolute,   mnemonic:"DCP", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0xD0, addressing:AddressingMode::AddrRelative,   mnemonic:"BNE", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ opcode: 0xD1, addressing:AddressingMode::AddrIndirectY,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ opcode: 0xD2, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xD3, addressing:AddressingMode::AddrIndirectY,  mnemonic:"DCP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xD4, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xD5, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ opcode: 0xD6, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"DEC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xD7, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"DCP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xD8, addressing:AddressingMode::AddrImplied,    mnemonic:"CLD", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xD9, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ opcode: 0xDA, addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xDB, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"DCP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xDC, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xDD, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"CMP", flags:VALID_OPCODE },
    Opcode{ opcode: 0xDE, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"DEC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xDF, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"DCP", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0xE0, addressing:AddressingMode::AddrImmediate,  mnemonic:"CPX", flags:VALID_OPCODE },
    Opcode{ opcode: 0xE1, addressing:AddressingMode::AddrIndirectX,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xE2, addressing:AddressingMode::AddrImmediate,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xE3, addressing:AddressingMode::AddrIndirectX,  mnemonic:"ISC", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xE4, addressing:AddressingMode::AddrZeroPage,   mnemonic:"CPX", flags:VALID_OPCODE },
    Opcode{ opcode: 0xE5, addressing:AddressingMode::AddrZeroPage,   mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xE6, addressing:AddressingMode::AddrZeroPage,   mnemonic:"INC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xE7, addressing:AddressingMode::AddrZeroPage,   mnemonic:"ISC", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xE8, addressing:AddressingMode::AddrImplied,    mnemonic:"INX", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xE9, addressing:AddressingMode::AddrImmediate,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xEA, addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:VALID_OPCODE },
    Opcode{ opcode: 0xEB, addressing:AddressingMode::AddrImmediate,  mnemonic:"SBC", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xEC, addressing:AddressingMode::AddrAbsolute,   mnemonic:"CPX", flags:VALID_OPCODE },
    Opcode{ opcode: 0xED, addressing:AddressingMode::AddrAbsolute,   mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xEE, addressing:AddressingMode::AddrAbsolute,   mnemonic:"INC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xEF, addressing:AddressingMode::AddrAbsolute,   mnemonic:"ISC", flags:UNDOC_OPCODE },

    Opcode{ opcode: 0xF0, addressing:AddressingMode::AddrRelative,   mnemonic:"BEQ", flags:VALID_OPCODE|BRANCH_OPCODE },
    Opcode{ opcode: 0xF1, addressing:AddressingMode::AddrIndirectY,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xF2, addressing:AddressingMode::AddrImplied,    mnemonic:"JAM", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xF3, addressing:AddressingMode::AddrIndirectY,  mnemonic:"ISC", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xF4, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xF5, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xF6, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"INC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xF7, addressing:AddressingMode::AddrZeroPageX,  mnemonic:"ISC", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xF8, addressing:AddressingMode::AddrImplied,    mnemonic:"SED", flags:VALID_OPCODE }, //
    Opcode{ opcode: 0xF9, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xFA, addressing:AddressingMode::AddrImplied,    mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xFB, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"ISC", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xFC, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"NOP", flags:UNDOC_OPCODE },
    Opcode{ opcode: 0xFD, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"SBC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xFE, addressing:AddressingMode::AddrAbsoluteY,  mnemonic:"INC", flags:VALID_OPCODE },
    Opcode{ opcode: 0xFF, addressing:AddressingMode::AddrAbsoluteX,  mnemonic:"ISC", flags:UNDOC_OPCODE }
];

impl CpuTrait for Cpu65xx {
    fn fetch_and_decode(&mut self) -> DisassembledLine {
        let fetched_opcode:u8 = self.memory.as_ref().unwrap().read_byte(self.pc as u32);
        println!("fetched: {}",fetched_opcode);
        let opcode=&OPCODES_TABLE[fetched_opcode as usize];
        let mut dasm_line = DisassembledLine::new();

        dasm_line.address = self.pc as u32;

        if opcode.flags & UNDOC_OPCODE != 0 {
            dasm_line.output = String::from("???");
            self.pc+=1;
            return dasm_line;
        }

        dasm_line.output = opcode.mnemonic.to_owned();

        let pc_inc = self.get_pc_inc_from_addressing(&opcode.addressing) as u16;
        let mut address:u16 = 0;

        match opcode.addressing {
            AddressingMode::AddrImplied   | 
            AddressingMode::AddrAccumulator => self.pc += pc_inc,
            AddressingMode::AddrImmediate |
            AddressingMode::AddrZeroPage  | 
            AddressingMode::AddrZeroPageX |
            AddressingMode::AddrZeroPageY |
            AddressingMode::AddrIndirectX |
            AddressingMode::AddrIndirectY => address = self.memory.as_ref().unwrap().read_byte((self.pc+1) as u32) as u16,
            AddressingMode::AddrRelative => address = self.pc + (self.memory.as_ref().unwrap().read_byte((self.pc+1) as u32) as u16)+2,
            _ => address = self.memory.as_ref().unwrap().read_word_le((self.pc+1) as u32) as u16
        }
        
        println!("tadaaaaa: {:#?}",dasm_line);
        
        dasm_line
    }    
}

impl Disassembler for Cpu65xx {
    fn disassemble(&self, offset:usize, code: &[u8]) -> Vec<DisassembledLine> {
        let mut dasm_lines:Vec<DisassembledLine> = Vec::new();
        let mut idx=offset;
        while idx < code.len() {
            let opcode=&OPCODES_TABLE[ code[idx] as usize];
        }

        dasm_lines
    }
}
impl Cpu65xx {
    pub fn new(memory: BinaryBuffer) -> Self {
        Self {
            pc: 0x0000,
            memory: Some(memory)
        }
    }

    fn build_dasm_line(opcode:Opcode, address:usize) -> DisassembledLine {
        let mut dasm_line = DisassembledLine::new();
        dasm_line.address = address as u32;
        
        if opcode.flags & UNDOC_OPCODE != 0 {
            dasm_line.output = String::from("???");
            return dasm_line;
        }


        dasm_line
    }


    fn get_pc_inc_from_addressing(&self, addressing:&AddressingMode) -> u8 {
        match addressing {
            AddressingMode::AddrImplied   | 
            AddressingMode::AddrAccumulator => 1,
            AddressingMode::AddrImmediate |
            AddressingMode::AddrZeroPage  | 
            AddressingMode::AddrZeroPageX |
            AddressingMode::AddrZeroPageY |
            AddressingMode::AddrIndirectX |
            AddressingMode::AddrIndirectY |
            AddressingMode::AddrRelative => 2,
            _ => 3
        }
    }
}

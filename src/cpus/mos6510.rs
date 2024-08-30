use opcodes6510::{format_string_from_addressing, get_pc_inc_from_addressing, AddressingMode, Opcode6510, OPCODES_TABLE};
use crate::dasm::UNDOC_OPCODE;
use super::CpuTrait;
use crate::{dasm::{DisassembledLine, Disassembler, LineType}, memory::BinaryBuffer};

mod opcodes6510;

#[derive(Debug,Default)]
pub struct Cpu6510 {
    pc: u16,
    memory: BinaryBuffer
}


impl CpuTrait for Cpu6510 {
    fn set_pc(&mut self, pc:u32) {
        self.pc = pc as u16;
    }

    fn fetch_and_decode(&mut self) -> DisassembledLine {
        let current_address:u32 = self.pc as u32 + self.memory.get_loaded_address();
        let fetched_opcode:u8 = self.memory.read_byte(self.pc as u32);
        println!("fetched: {}",fetched_opcode);
        let opcode=&OPCODES_TABLE[fetched_opcode as usize];
        let mut dasm_line = DisassembledLine::new();

        dasm_line.address = current_address;
        dasm_line.flags = opcode.flags;

        if opcode.flags & UNDOC_OPCODE != 0 {
            dasm_line.output = String::from("???");
            // In questo caso è un istruzione non documentata
            // e potrebbe avere senso una LineType::ToBeExamine.
            dasm_line.line_type = LineType::UnknownInstruction;
            self.pc+=1;
            return dasm_line;
        }

        dasm_line.output = opcode.mnemonic.to_owned();

        let pc_inc = get_pc_inc_from_addressing(&opcode.addressing);
        dasm_line.instr_size = pc_inc-1;

        let mut address:u16 = 0;

        if dasm_line.instr_size != 0 {
            if dasm_line.instr_size == 1 {
                if opcode.addressing != AddressingMode::AddrRelative {
                    address = self.memory.read_byte((self.pc+1) as u32) as u16;
                }else{
                    let tmp=self.memory.read_signed_byte((self.pc+1) as u32);
                    address = current_address as u16 - (self.memory.read_signed_byte((self.pc+1) as u32) as i16)+2;
                }
                dasm_line.output.push_str(format_string_from_addressing(&opcode.addressing, address).as_str());
            }else if dasm_line.instr_size == 2 {
                address = self.memory.read_word_le((self.pc+1) as u32) as u16;
            }else{
                panic!("Instruction size is not 1 or 2");
            }

            if opcode.addressing != AddressingMode::AddrImmediate {
                dasm_line.address_ref = address as u32;
            }
        }

        println!("tadaaaaa: {:#?}",dasm_line);
        self.pc += pc_inc as u16;
        dasm_line
    }    
}

impl Disassembler for Cpu6510 {
    fn disassemble(&self, offset:usize, code: &[u8]) -> Vec<DisassembledLine> {
        let mut dasm_lines:Vec<DisassembledLine> = Vec::new();
        let mut idx=offset;
        while idx < code.len() {
            let opcode=&OPCODES_TABLE[ code[idx] as usize];
        }

        dasm_lines
    }
}
impl Cpu6510 {
    pub fn new(memory: BinaryBuffer) -> Self {
        Self {
            pc: memory.get_loaded_address() as u16,
            memory,
        }
    }

    fn build_dasm_line(opcode:Opcode6510, address:usize) -> DisassembledLine {
        let mut dasm_line = DisassembledLine::new();
        dasm_line.address = address as u32;
        
        if opcode.flags & UNDOC_OPCODE != 0 {
            dasm_line.output = String::from("???");
            return dasm_line;
        }


        dasm_line
    }



}

use crate::dasm::{DisassembledLine, DisassemblerTrait, LineType, SUBROUTINE_OPCODE, UNDOC_OPCODE};

use super::{addressing::AddressingMode, opcodes6510::OPCODES_TABLE, Cpu6510};


impl DisassemblerTrait for Cpu6510 {
    fn disassemble_next(&mut self) -> DisassembledLine {
        let current_address:u32 = self.pc as u32 + self.memory.get_loaded_address();
        let fetched_opcode:u8 = self.memory.read_byte(self.pc as u32);

        let opcode=&OPCODES_TABLE[fetched_opcode as usize];
        let mut dasm_line = DisassembledLine::new();

        dasm_line.address = current_address;
        dasm_line.flags = opcode.flags;

        if opcode.flags & UNDOC_OPCODE != 0 {
            dasm_line.opcode = String::from("???");
            // In questo caso è un istruzione non documentata
            // e potrebbe avere senso una LineType::ToBeExamine.
            dasm_line.line_type = LineType::UnknownInstruction;
            self.pc+=1;
            return dasm_line;
        }

        let pc_inc = AddressingMode::get_pc_inc(&opcode.addressing);
        dasm_line.instr_size = pc_inc-1;

        let mut address:u16 = 0;

        dasm_line.opcode = opcode.mnemonic.to_owned();
        dasm_line.byte_code[0] = opcode.opcode;

        if dasm_line.instr_size != 0 {
            if dasm_line.instr_size == 1 {
                if opcode.addressing != AddressingMode::AddrRelative {
                    let byte=self.memory.read_byte((self.pc+1) as u32);
                    dasm_line.byte_code[1] = byte;
                    address = byte as u16;
                }else{
                    // TODO: Questa cosa non mi entusiasma affatto!
                    let byte=self.memory.read_signed_byte((self.pc+1) as u32);
                    dasm_line.byte_code[1] = byte as u8;

                    if byte < 0 {
                        address = (current_address - byte.abs() as u32 + 2) as u16;
                    }else{
                        address = (current_address + byte.abs() as u32 + 2) as u16;
                        dasm_line.address_ref = address as u32;
                    }
                }

                dasm_line.operand.push_str(AddressingMode::format_string(&opcode.addressing, address).as_str());
            }else if dasm_line.instr_size == 2 {
                address = self.memory.read_word_le((self.pc+1) as u32) as u16;
                dasm_line.byte_code[1] = (address & 0x00FF) as u8;
                dasm_line.byte_code[2] = ((address & 0xFF00) >> 8) as u8;

                if opcode.opcode == 0x20 || opcode.opcode == 0x4C {
                    if (opcode.flags & SUBROUTINE_OPCODE) != 0 {
                        dasm_line.operand.push_str("SUB__");
                        dasm_line.operand.push_str(AddressingMode::format_string(&opcode.addressing, address).as_str());
                    }else{
                        dasm_line.operand.push_str("JUMP__");
                        dasm_line.operand.push_str(AddressingMode::format_string(&opcode.addressing, address).as_str());
                    }
                }else{
                    dasm_line.operand.push_str(AddressingMode::format_string(&opcode.addressing, address).as_str());
                }
            }else{
                panic!("Instruction size is not 1 or 2");
            }

            if opcode.addressing != AddressingMode::AddrImmediate {
                dasm_line.address_ref = address as u32;
            }
        }

        self.pc += pc_inc as u16;
        dasm_line
    }
}

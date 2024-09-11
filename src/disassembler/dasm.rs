use std::collections::HashMap;
use crate::{cpus::CpuTrait, disassembler::{BRANCH_OPCODE, JMP_OPCODE, SUBROUTINE_OPCODE}};
use super::{types::DisassembledLine, Dasm, INDIRECT_FLAG};

impl Dasm {
    pub fn new(cpu: Box<dyn CpuTrait>, start_pc:u32) -> Self {
        Dasm {
            cpu,
            start_pc,
            labels_map:HashMap::new(),
            lines:Vec::new(),
            relative_label_id: 0,
            jump_label_id: 0,
            subroutine_label_id: 0
        }
    }

    pub fn run(&mut self) {
        self.cpu.set_pc(self.start_pc);

        self.pass1();
        self.pass2();
        self.print_dasm();
    }

    fn pass1(&mut self) {
        loop {
            let res = self.cpu.disassemble_next();
            if res.is_none() {
                return;
            }
            let mut line=res.unwrap();

            if line.flags & BRANCH_OPCODE != 0 || line.flags & SUBROUTINE_OPCODE != 0 || (line.flags & JMP_OPCODE != 0 && line.flags & INDIRECT_FLAG == 0) {
                self.generate_label(&mut line);
            }

            self.lines.push(line);
        }
    }

    fn pass2(&mut self) {

    }

    pub fn print_dasm(&self) {
        let mut output_line= String::new();
        for dasm_line in &self.lines {
            output_line.push_str(&format!("{:04X}    ",dasm_line.address));

            for i in 0..dasm_line.instr_size+1 {
                output_line.push_str(&format!("{:02X} ",dasm_line.byte_code[i as usize]));
            }
            if dasm_line.instr_size == 0 {
                output_line.push_str("      ");
            }else if dasm_line.instr_size == 1 {
                output_line.push_str("   ");
            }
            output_line.push_str(&format!("{} ", dasm_line.opcode));
            output_line.push_str(&format!("{} ", dasm_line.operand));

            if dasm_line.comment.len() > 0 {
                output_line.push_str(&format!("   ; {}", dasm_line.comment));
            }

            println!("{}", output_line);
            output_line="".to_owned();
        }
    }
    fn generate_label(&mut self, dasm_line: &mut DisassembledLine) {
        if self.labels_map.contains_key(&(dasm_line.address as u32)) {
            dasm_line.operand = self.labels_map.get(&(dasm_line.address as u32)).unwrap().to_string();
        }else{
            if dasm_line.flags & BRANCH_OPCODE != 0 {
                dasm_line.operand = format!("LABEL{:0>8}", self.relative_label_id);
                self.relative_label_id+=1;
            }else if dasm_line.flags & SUBROUTINE_OPCODE != 0 {
                dasm_line.operand = format!("SUBRT{:0>8}", self.subroutine_label_id);
                self.subroutine_label_id+=1;
            }else{
                dasm_line.operand = format!("JUMP_{:0>8}", self.jump_label_id);
                self.jump_label_id+=1;
            }

            self.labels_map.insert(dasm_line.address as u32, dasm_line.operand.clone());
            let sign = if dasm_line.address_ref > dasm_line.address { "+" } else { "-" };
            dasm_line.comment=format!("${:04X} {}", dasm_line.address_ref, sign);
        }
    }
}

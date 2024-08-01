
#[derive(Debug, Default)]
pub enum LineType {
    #[default] None,
    Instruction,
    Comment,
}

#[derive(Debug, Default)]
pub struct DisassembledLine {
    line_type: LineType,
    pub address: u32,
    offset: u32,
    return_address: u32,
    output: String,
    label: String,
    comment: String
}

impl DisassembledLine {
    pub fn new() -> Self {
        DisassembledLine {
            line_type: LineType::None,
            address: 0,
            offset: 0,
            return_address: 0,
            output: String::from(""),
            label: String::from(""),
            comment: String::from("")
        }
    }

    pub fn from(line_type:LineType, address:u32, offset: u32, label: &str) -> Self {
        DisassembledLine {
            line_type,
            address,
            offset,
            return_address: 0,
            output: String::from(""),
            label: String::from(label),
            comment: String::from("")
        }
    }
}

pub trait DasmTrait {

    fn fetch_and_decode(&mut self) -> DisassembledLine;


}
#[cfg(test)]
mod dasm_test {
    use crate::dasm::DisassembledLine;

    #[test]
    fn test_line() {
        let line = DisassembledLine::new();
        println!("{}",line.address);
        assert_eq!(0, line.address);
    }
}


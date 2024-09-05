
#[derive(Debug,PartialEq)]
pub enum AddressingMode {
    AddrImplied,    // ""
    AddrAccumulator,// ""
    AddrImmediate,  // "#$%2.2X"
    AddrZeroPage,   // "$%2.2X"
    AddrZeroPageX,  // "$%2.2X,X"
    AddrZeroPageY,  // "$%2.2X,Y"
    AddrAbsolute,   // "$%2.2X%2.2X" -> 16bit
    AddrAbsoluteX,  // "$%2.2X%2.2X,X" -> 16bit
    AddrAbsoluteY,  // "$%2.2X%2.2X,Y" -> 16bit
    AddrIndirectX,  // "($%2.2X,X)"
    AddrIndirectY,  // "($%2.2X),Y"
    AddrRelative,   // "$%4.4X"
    AddrIndirect    // "($%2.2X%2.2X)"
}

impl AddressingMode {
    pub fn format_string(addressing:&AddressingMode, value: u16) -> String {
        match addressing {
            AddressingMode::AddrImplied   | 
            AddressingMode::AddrAccumulator => String::from(""),
            AddressingMode::AddrImmediate => String::from(format!("#${:02X}",value)),
            AddressingMode::AddrZeroPage  => String::from(format!("${:02X}",value)),
            AddressingMode::AddrZeroPageX => String::from(format!("${:02X},X",value)),
            AddressingMode::AddrZeroPageY => String::from(format!("${:02X},Y",value)),
            AddressingMode::AddrAbsolute => String::from(format!("${:04X}",value)),
            AddressingMode::AddrAbsoluteX => String::from(format!("${:04X},X",value)),
            AddressingMode::AddrAbsoluteY => String::from(format!("${:04X},Y",value)),
            AddressingMode::AddrIndirectX => String::from(format!("(${:02X},X)",value)),
            AddressingMode::AddrIndirectY => String::from(format!("(${:02X}),Y",value)),
            AddressingMode::AddrRelative => String::from(format!("${:04X}",value)),
            AddressingMode::AddrIndirect => String::from(format!("(${:04X})",value)),
        }
    }

    pub fn get_pc_inc(addressing:&AddressingMode) -> u8 {
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

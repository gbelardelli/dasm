

pub struct MC680x0 {
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

impl DasmTrait for MC680x0 {
    fn dasm(bytes: &Vec<u8>) {

    }
}
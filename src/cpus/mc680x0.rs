use super::CpuTrait;



pub struct MC680x0 {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,

    pc: u32,

    n: bool,
    v: bool,
    d: bool,
    i: bool,
    z: bool,
    c: bool,
}
/*
impl CpuTrait for MC680x0 {
    fn set_pc(&mut self, pc:u32) -> Result<i32,i32> {
        if pc <= 0xFFFFFF {
            self.pc = pc;
            return Ok(0);
        }
        Err(1)
    }
}
    */
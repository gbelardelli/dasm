
// Il modulo è privato così che nessuno al di fuori di questo crate può
// implementarlo
mod sealed {
    pub trait Sealed {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

pub trait BusSize: sealed::Sealed + Into<u64> {}

impl BusSize for u16 {}
impl BusSize for u32 {}
impl BusSize for u64 {}

pub trait SignedWrap {
    type Signed;
    fn to_signed(self) -> Self::Signed;
}

impl SignedWrap for u8  { type Signed = i8;  fn to_signed(self) -> i8  { self as i8 } }
impl SignedWrap for u16 { type Signed = i16; fn to_signed(self) -> i16 { self as i16 } }
impl SignedWrap for u32 { type Signed = i32; fn to_signed(self) -> i32 { self as i32 } }
impl SignedWrap for u64 { type Signed = i64; fn to_signed(self) -> i64 { self as i64 } }


#[derive(Debug,Default)]
pub struct BinaryBuffer {
    data: Vec<u8>,
    loaded_address: u64,
}

impl BinaryBuffer {
    pub fn new(data: Vec<u8>, loaded_address: u64) -> Self {
        BinaryBuffer {
            data,
            loaded_address,
        }
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }
    pub fn get_loaded_address(&self) -> u64 {
        self.loaded_address
    }

    /// Restituisce l’indirizzo di memoria assoluto corrispondente 
    /// a un offset
    pub fn address_of(&self, offset: u64) -> u64 {
        self.loaded_address + offset
    }

    /// Restituisce l’offset all'interno del buffer da
    /// un indirizzo assoluto
    pub fn offset_of(&self, addr: u64) -> Option<u64> {
        if addr < self.loaded_address {
            return None;
        }

        let offset = addr - self.loaded_address;
        if (offset as usize) < self.data.len() {
            Some(offset)
        } else {
            None
        }
    }

    pub fn read_byte<T: BusSize>(&self, offset: T) -> u8 {
        let offset64 = offset.into() as usize;
        self.data[offset64]
    }

    pub fn read_word_le(&self, offset: u32) -> u16 {
        let mut word:u16 = self.data[(offset+1) as usize] as u16;
        word<<=8;
        word|=self.data[offset as usize] as u16;
        word
    }

    fn read_word_be(&self, offset: u32) -> u16 {
        let mut word:u16 = self.data[offset as usize] as u16;
        word|=((self.data[(offset+1) as usize] as u16) << 8) as u16;
        word
    }
    // Potrebbe essere un trait il fatto di leggere del buffer in LE o BE
}

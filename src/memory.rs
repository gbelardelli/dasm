

#[derive(Debug,Default)]
pub struct BinaryBuffer {
    data: Vec<u8>,
    loaded_address: u32,
    buffer_size: usize,
}

impl BinaryBuffer {
    pub fn new(data: Vec<u8>, loaded_address: u32) -> Self {
        BinaryBuffer {
            buffer_size: data.len(),
            data,
            loaded_address,
        }
    }

    pub fn get_loaded_address(&self) -> u32 {
        self.loaded_address
    }

    pub fn read_byte(&self, offset: u32) -> u8 {
        self.data[offset as usize]
    }
    pub fn read_signed_byte(&self, offset: u32) -> i8 {
        self.data[offset as usize] as i8
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

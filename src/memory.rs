
trait BusSize {}

impl BusSize for u16 {}
impl BusSize for u32 {}


#[derive(Debug,Default)]
pub struct BinaryBuffer {
    data: Vec<u8>,
    loaded_address: u32,
    buffer_size: usize,
}

// TODO: Valutare l'idea di fare BinaryBuffer<T>
// per avere offset a 16, 32 o 64bit. Questo servirebbe per evitare di fare
// troppe conversioni as type nel codice e renderlo anche più leggibile.
// Il <T> dovrebbe essere limitato ai tipi u16,u32 e u64. Se non è possibile
// tale limitazione si può fare con i trait ad es:
// pub trait Bus16 {
//    fn read_byte(&self, offset: u16) -> u8;
//    fn read_signed_byte(&self, offset: u32) -> i8;
//    fn read_word_le(&self, offset: u16) -> u16;
// }

impl BinaryBuffer {
    pub fn new(data: Vec<u8>, loaded_address: u32) -> Self {
        BinaryBuffer {
            buffer_size: data.len(),
            data,
            loaded_address,
        }
    }

    pub fn get_size(&self) -> usize {
        self.buffer_size
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

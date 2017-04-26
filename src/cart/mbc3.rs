use cart::ByteIO;

pub struct Mbc3 {

}

impl Mbc3 {
    pub fn new(bytes: Vec<u8>) -> Mbc3 {
        Mbc3 {}
    }
}

impl ByteIO for Mbc3 {

    fn read_byte(&self, addr:u16) -> u8 {
        panic!("invalid mbc3 read: 0x{:04x}",addr)
    }

    fn write_byte(&mut self, addr:u16, val:u8) {
        panic!("invalid mbc3 write: 0x{:04x}",addr)
    }

}

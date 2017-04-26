use cart::ByteIO;

pub struct Mbc1 {

}

impl Mbc1 {
    pub fn new(bytes: Vec<u8>) -> Mbc1 {
        Mbc1 {}
    }
}

impl ByteIO for Mbc1 {

    fn read_byte(&self, addr:u16) -> u8 {
        panic!("invalid mbc1 read: 0x{:04x}",addr)
    }

    fn write_byte(&mut self, addr:u16, val:u8) {
        panic!("invalid mbc1 write: 0x{:04x}",addr)
    }

}

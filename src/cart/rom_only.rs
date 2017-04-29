use cart::ByteIO;

pub struct RomOnly {
    bytes: Vec<u8>,
}

impl RomOnly {
    pub fn new(bytes: Vec<u8>) -> RomOnly {
        RomOnly {bytes: bytes}
    }
}

impl ByteIO for RomOnly {

    fn read_byte(&self, addr:u16) -> u8 {
        let idx = addr as usize;
        match addr {
            0x0000...0x7FFF => self.bytes[idx],
            _ => {panic!("invalid rom_only read: 0x{:04x}",addr)}
        }
    }

    fn write_byte(&mut self, addr:u16, val:u8) {

        //panic!("invalid rom_only write: 0x{:04x}",addr)
    }

}

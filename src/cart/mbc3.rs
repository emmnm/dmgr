//!
//! mbc3.rs - memory bank controller version 3.
//!
//! See: http://gbdev.gg8.se/wiki/articles/MBC3
//!
//!
use cart::ByteIO;

pub struct Mbc3 {
    bytes: Vec<u8>,

    /// from 0 to 127.
    rom_bank: usize,
    ram: Vec<u8>,

    ram_bank: usize,
    ram_rtc_enable: bool,
}

impl Mbc3 {

    pub fn new(bytes: Vec<u8>) -> Mbc3 {
        Mbc3 {
            bytes:bytes,
            rom_bank: 0x01,
            ram: vec![0; 0x8000],
            ram_bank: 0x00,
            ram_rtc_enable: false,
        }
    }
}

impl ByteIO for Mbc3 {

    fn read_byte(&self, addr:u16) -> u8 {
        let idx = addr as usize;
        match addr {
            0x0000...0x3FFF => {self.bytes[idx]},
            0x4000...0x7FFF => {self.bytes[(idx - 0x4000) + 0x4000 * self.rom_bank] }, // here!
            0xA000...0xBFFF => { self.ram[(idx-0xA000) + 0x2000 * self.ram_bank] },
            _ => {panic!("Invalid mbc3 read location! 0x{:04X}",addr)},
        }
    }

    fn write_byte(&mut self, addr:u16, val:u8) {
        let idx = addr as usize;
        match addr {
            0x0000...0x1FFF => {
                self.ram_rtc_enable = (0x0A & val) > 0x00;
            },
            0x2000...0x3FFF => { //use 7 bits.
                if val > 127 {
                    panic!("bank bad");
                }
                self.rom_bank = if val == 0x00 {
                    0x01
                } else {
                    0x7F & val
                } as usize;
            },
            0xA000...0xBFFF => {
                self.ram[(idx - 0xA000) + 0x2000 * self.ram_bank] = val;
            },
            0x4000...0x5FFF => {
                if val < 0x04 {
                    self.ram_bank = (0x03 & val) as usize;
                } else {
                    panic!("RTC Not supported!");
                }
            },
            0x6000...0x7FFF => {
                //rtc stuff.
                //{},
            },
            _ => {panic!("Invalid mbc3 write location! 0x{:04X}",addr)},
        }
    }

}

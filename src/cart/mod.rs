use std::rc::Rc;

use std::fmt;
use std::fmt::Debug;
use std::string::String;
use constants::read_bytes_from_file;

mod rom_only;
mod mbc1;
mod mbc3;

pub use cart::mbc1::Mbc1;
pub use cart::mbc3::Mbc3;
pub use cart::rom_only::RomOnly;

pub trait ByteIO {
    fn read_byte(&self, addr:u16) -> u8;
    fn write_byte(&mut self, addr:u16, val:u8);
}

pub struct Cartridge {
    title: String,
    mbc_type: String,
    rom_size: String,
    ram_size: String,
    destination: String,

    mbc: Box<ByteIO>,
}

impl Cartridge {

    pub fn new(file_path: String) -> Cartridge {
        let bytes = read_bytes_from_file(file_path);
        let title = (&bytes[0x0134..0x0144]).into_iter()
                        .take_while(|x| **x > 0x00)
                        .cloned()
                        .collect::<Vec<_>>();
        let mbc_type = match bytes[0x0147] {
            0x00 => "NONE",
            0x01...0x03 => "MBC1",
            0x05...0x06 => "MBC2",
            0x0F...0x13 => "MBC3",
            0x19...0x1E => "MBC5",
            _ => "OTHER"
        };
        let destination = match bytes[0x014A] {
            0x00 => "JAPAN",
            _ => "WORLD",
        };
        Cartridge {
            title: String::from_utf8(title).unwrap(),
            mbc_type: String::from(mbc_type),
            rom_size: String::from("N/A"),
            ram_size: String::from("N/A"),
            destination: String::from(destination),
            mbc: match bytes[0x0147] {
                0x00 => Box::new(RomOnly::new(bytes)),
                0x01...0x03 => Box::new(Mbc1::new(bytes)),
                // 0x05...0x06 => "MBC2",
                0x0F...0x13 => Box::new(Mbc3::new(bytes)),
                // 0x19...0x1E => "MBC5",
                val => {panic!("mbc not supported 0x{:2x}",val)}
            }
        }
    }
}

impl ByteIO for Cartridge {
    fn read_byte(&self, addr:u16) -> u8 {
        self.mbc.read_byte(addr)
    }

    fn write_byte(&mut self, addr:u16, val:u8) {
        self.mbc.write_byte(addr,val)
    }
}



impl Debug for Cartridge {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{{ title={}, mbc={}, rom={}, ram={}, region={} }}",
            self.title, self.mbc_type, self.rom_size, self.ram_size, self.destination)
    }

}

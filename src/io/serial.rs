//!
//! link cable implementations.
//!
use cart::ByteIO;

/// Stub implementation, returns values
/// as if no link is connected.
pub struct Serial {
    control: u8,
}

impl Serial {
    pub fn new() -> Serial {
        Serial {control: 0}
    }
}

impl ByteIO for Serial {

    fn read_byte(&self, addr:u16) -> u8 {
        match addr {
            0xFF01 => {0},
            0xFF02 => {self.control}
            _ => {panic!("Invalid Serial read address: 0x{:04X}",addr)}
        }
    }

    fn write_byte(&mut self, addr:u16, val:u8) {
        match addr {
            0xFF01 => {},
            0xFF02 => {self.control = val},
            _ => {panic!("Invalid Serial write address: 0x{:04X}",addr)}
        }
    }

}

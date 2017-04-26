use cart::ByteIO;
use context::Context;

pub struct Joypad {

}

impl Joypad {

}

impl ByteIO for Joypad {
    fn read_byte(&self, addr:u16) -> u8 {
        0
    }
    fn write_byte(&mut self, addr:u16, val:u8) {

    }
}

impl Joypad {
    pub fn step(ctx:&mut Context, cycles: usize) {
    }
}

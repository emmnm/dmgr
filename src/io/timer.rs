use cart::ByteIO;
use context::Context;

pub struct Timer {

}

impl Timer {
    pub fn new() -> Timer {
        Timer {}
    }
}

impl ByteIO for Timer {
    fn read_byte(&self, addr:u16) -> u8 {
        0
    }
    fn write_byte(&mut self, addr:u16, val:u8) {

    }
}


impl Timer {
    pub fn step(ctx:&mut Context, cycles: usize) {
    }
}

use cart::ByteIO;
use context::Context;

pub struct Sound {

}

impl Sound {
    pub fn new() -> Sound {
        Sound {}
    }
}

impl ByteIO for Sound {
    fn read_byte(&self, addr:u16) -> u8 {
        0
    }
    fn write_byte(&mut self, addr:u16, val:u8) {

    }
}


impl Sound {
    pub fn step(ctx:&mut Context, cycles: usize) {
    }
}

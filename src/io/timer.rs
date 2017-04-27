//! Timer module.
//!
use cart::ByteIO;
use context::Context;

pub struct Timer {

    ///control. Sets speed of counter.
    control: u8,

    /// clock value.
    counter: u8,

    /// when the counter overflows, it is reset to modulo.
    modulo: u8,

    /// always counts up, can't be disabled.
    divider: u8,

    // divider real time,
    internal_divider: usize,
    internal_counter: usize,
}

impl Timer {

    pub fn new() -> Timer {
        Timer {control:0,counter:0,modulo:0,divider:0,internal_divider:0,internal_counter:0}
    }

    pub fn step(ctx:&mut Context, num_cycles:usize) {
        let mut flag = false;
        {
            let timer = ctx.timer();
            //divider increases by one every 256 instructions.
            timer.internal_divider += num_cycles;
            if timer.internal_divider > 0xFF {
                timer.divider = timer.divider.wrapping_add(1);
                timer.internal_divider = 0;
            }

            // counter goes up at variable speed.
            if timer.control & 0x04 > 0x00 {
                timer.internal_counter += num_cycles;
                if timer.internal_counter > timer.clock_period() {
                    timer.counter = timer.counter.wrapping_add(1);
                    timer.internal_counter = 0;

                    if timer.counter == 0x00 {
                        flag = true;
                    }
                }
            }
        }
        if flag {
            ctx.ints().request(0x04);
        }
    }

    /// Calculates the number of cycles per Interrupt
    /// at the specified clock speed.
    fn clock_period(&self) -> usize {
        match self.control & 0x03 {
            0b00 => { 1024 }, // freq 4096 / cpu speed.
            0b01 => { 16 }, // freq 262144
            0b10 => { 64 }, // freq 65536
            0b11 => { 256 }, // freq 16382
            _ => panic!("Invalid clock register")
        }
    }
}

impl ByteIO for Timer {

        fn read_byte(&self, addr:u16) -> u8 {
            match addr {
                0xFF04 => {self.divider},
                0xFF05 => {self.counter},
                0xFF06 => {self.modulo},
                0xFF07 => {self.control},
                _ => {panic!("Invalid timer read address: 0x{:04X}",addr)},
            }
        }

        fn write_byte(&mut self, addr:u16, val:u8) {
            match addr {
                0xFF04 => {self.divider = 0},
                0xFF05 => {self.counter = val},
                0xFF06 => {self.modulo = val},
                0xFF07 => {self.control = 0x07 & val }
                _ => {panic!("Invalid timer write address: 0x{:04X}",addr)},
            }
        }

}

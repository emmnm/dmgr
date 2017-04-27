//!
//! joypad.rs - Joypad implementation.
//!
//! Uses Context to set registers appropriately.
//!
use sdl2::EventPump;
use context::Context;
use cart::ByteIO;


/// Structure to store current key presses and store
/// whether the directional pad or buttons are selected.
pub struct Joypad {

    /// the top nibble stores the buttons, the bottom nibble stores the dpad.
    all_keys:u8,

    /// selection bit mask.
    selection:u8,
}

impl Joypad {

    /// Create a new Joypad.
    pub fn new() -> Joypad {
        Joypad {all_keys:0xFF, selection:0x00}
    }

    /// Update the bit values of `all_keys` based off the buttons
    /// detected by the Context.
    pub fn step(ctx:&mut Context, pump: &mut EventPump, num_cycles: usize) {
        let joy = ctx.joypad();

        for event in pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit { .. } => {panic!("DONE") },
                KeyUp { keycode, .. } => match keycode {
                    Some(W) => {joy.all_keys |= 0x04},
                    Some(A) => {joy.all_keys |= 0x02},
                    Some(S) => {joy.all_keys |= 0x08},
                    Some(D) => {joy.all_keys |= 0x01},
                    Some(J) => {joy.all_keys |= 0x20},
                    Some(K) => {joy.all_keys |= 0x10},
                    Some(Quote) => {joy.all_keys |= 0x40},
                    Some(Return) => {joy.all_keys |= 0x80},
                    _ => {},
                },
                KeyDown {keycode, .. } => match keycode {
                    Some(W) => {joy.all_keys &= !0x04},
                    Some(A) => {joy.all_keys &= !0x02},
                    Some(S) => {joy.all_keys &= !0x08},
                    Some(D) => {joy.all_keys &= !0x01},
                    Some(J) => {joy.all_keys &= !0x20},
                    Some(K) => {joy.all_keys &= !0x10},
                    Some(Quote) => {joy.all_keys &= !0x40},
                    Some(Return) => {joy.all_keys &= !0x80},
                    _ => {},
                },
                _ => (),
            }
        }
        //println!("{:08b}",joy.all_keys);
    }
}

impl ByteIO for Joypad {

    /// Read byte representing the keys pressed.
    fn read_byte(&self, addr:u16) -> u8 {
        match addr {
            0xFF00 => {
                if (self.selection & 0x10 > 0) { //direction keys
                    0x20 | (0x0f & self.all_keys)
                } else if ( self.selection & 0x20 > 0) { //button keys.
                    0x10 | (self.all_keys >> 4)
                } else { //nothing selected.
                    0x3f
                }
            }
            _ => {panic!("invalid joypad read address: 0x{:04X}",addr)}
        }
    }

    /// Write a byte, which selects the dpad or regular buttons.
    fn write_byte(&mut self, addr:u16, val:u8) {
        match addr {
            0xFF00 => {self.selection = !val}
            _ => {panic!("invalid joypad write address: 0x{:04X}",addr)}
        }
    }
}

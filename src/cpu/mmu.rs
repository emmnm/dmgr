extern crate sdl2;
use sdl2::render::Renderer;
use sdl2::render::Texture;

use context::Context;
use io::{Lcd,Gpu,Timer,Joypad,Sound};
use cart::ByteIO;

pub fn step(ctx:&mut Context, renderer:&mut Renderer, texture:&mut Texture, cycles: usize) {
    // Gpu::step(ctx,cycles);
    // Lcd::step(ctx,renderer,texture,cycles);
    // Timer::step(ctx,cycles);
    // Joypad::step(ctx,cycles);
    //Sound::step(ctx,cycles);
}

pub fn read_byte(ctx:&mut Context, addr: u16) -> u8 {
    let idx = addr as usize;
    match addr {
        0x0000...0x7FFF => {
            if ctx.in_bios() && addr < 0x0100 {
                ctx.bios()[idx]
            } else {
                ctx.cart().read_byte(addr)
            }
        },
        0x8000...0x9FFF => ctx.gpu().read_byte(addr),
        0xA000...0xBFFF => ctx.cart().read_byte(addr),
        0xC000...0xCFFF => ctx.wb0()[idx-0xC000],
        0xD000...0xDFFF => ctx.wb1()[idx-0xD000],
        0xE000...0xEFFF => ctx.wb0()[idx-0xE000],
        0xF000...0xFDFF => ctx.wb1()[idx-0xF000],
        0xFE00...0xFE9F => ctx.gpu().read_byte(addr),
        0xFF00 => ctx.joypad().read_byte(addr),
        0xFF01...0xFF02 => ctx.serial().read_byte(addr),
        0xFF04...0xFF07 => ctx.timer().read_byte(addr),
        0xFF0F => ctx.ints().read_flag(),
        0xFF10...0xFF3F => ctx.sound().read_byte(addr),
        0xFEA0...0xFEFF => {panic!("Unusable memory")},
        // FF00-FF7F   I/O Ports
        0xFF40...0xFF4B => ctx.gpu().read_byte(addr),
        0xFF80...0xFFFE => ctx.hram()[idx-0xFF80],
        0xFFFF => ctx.ints().read_enable(),
        _ => panic!("invalid mmu read addr 0x{:04X}",addr)
    }
}

pub fn read_word(ctx: &mut Context, addr:u16) -> u16 {
    let low = read_byte(ctx,addr) as u16;
    let high = read_byte(ctx,addr+1) as u16;
    (high << 8) | low
}

pub fn write_byte(ctx:&mut Context, addr:u16, val:u8) {
    let idx = addr as usize;
    match addr {
        0x0000...0x7FFF => {
            if ctx.in_bios() && addr < 0x0100 {
                panic!("Writing to bios")
            } else {
                ctx.cart().write_byte(addr,val)
            }
        },
        0x8000...0x9FFF => ctx.gpu().write_byte(addr,val),
        0xA000...0xBFFF => ctx.cart().write_byte(addr,val),
        0xC000...0xCFFF => ctx.wb0()[idx-0xC000] = val,
        0xD000...0xDFFF => ctx.wb1()[idx-0xD000] = val,
        0xE000...0xEFFF => ctx.wb0()[idx-0xE000] = val,
        0xF000...0xFDFF => ctx.wb1()[idx-0xF000] = val,
        0xFE00...0xFE9F => ctx.gpu().write_byte(addr,val),
        0xFEA0...0xFEFF => {}, //unusable
        0xFF00 => ctx.joypad().write_byte(addr,val),
        0xFF01...0xFF02 => ctx.serial().write_byte(addr,val),
        0xFF04...0xFF07 => ctx.timer().write_byte(addr,val),
        0xFF0F => ctx.ints().write_flag(val),
        0xFF10...0xFF3F => ctx.sound().write_byte(addr,val),
        0xFF46 => { //dma write.
            let mut data = vec![0;0xA0];
            for i in 0x00..0xA0 { data[i] = read_byte(ctx,
                ((val as u16) << 8) + i as u16) }
            ctx.gpu().write_dma(&data);
        },
        0xFF40...0xFF4B => ctx.gpu().write_byte(addr,val),
        0xFF50 => {println!("Leaving bios"); ctx.leave_bios()},
        //0xFF50 => panic!("Left bios"),
        0xFF4C...0xFF7F => {}, //empty io.
        0xFF80...0xFFFE => ctx.hram()[idx-0xFF80] = val,
        0xFFFF => ctx.ints().write_enable(val),
        _ => panic!("invalid mmu write addr 0x{:04X}",addr),
    }
}

pub fn write_word(ctx: &mut Context, addr:u16, val:u16) {
    write_byte(ctx,addr,val as u8);
    write_byte(ctx,addr+1,(val >> 8) as u8);
}

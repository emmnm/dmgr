extern crate sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;

use std::time::{Duration, Instant};
use std::thread;
use std::env;

mod constants;
mod context;
mod cart;
mod cpu;
mod io;

use constants::{WINDOW_SCALE,CYCLES_PER_SECOND};
use context::Context;
use cpu::mmu;
use io::{Gpu,Timer,Lcd,Joypad};

fn main() {

    let argv = env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        println!("Usage: {} <rom_file>",argv[0]);
        return;
    }

    let sdl_handle = sdl2::init().unwrap();

    let video_handle = sdl_handle.video().unwrap();
    let window = video_handle.window("RGB Emulator",WINDOW_SCALE * 160, WINDOW_SCALE * 144)
         .position_centered()
         .opengl()
         .build().unwrap();
    let mut renderer = window.renderer()
         .build().unwrap();
    let mut texture = renderer.create_texture_streaming(
        PixelFormatEnum::RGB24, 160, 144).unwrap();

    let mut ctx = Context::new(argv[1].clone());
    let mut event_pump = sdl_handle.event_pump().unwrap();
    println!("read cartridge: {:?}",ctx.cart());

    ctx.reset();
    loop {
        let start = Instant::now();

        /* Run number of cycles required */
        let mut cycle_count = 0;
        while cycle_count < (CYCLES_PER_SECOND) {
            //println!("reg {:?}",ctx.reg());
            let cycles = cpu::step(&mut ctx);
            //mmu::step(&mut ctx,&mut renderer, &mut texture, cycles);
            Gpu::step(&mut ctx,cycles);
            Lcd::step(&mut ctx,&mut renderer,&mut texture,cycles);
            Timer::step(&mut ctx,cycles);
            Joypad::step(&mut ctx,&mut event_pump, cycles);

            cpu::handle_interrupts(&mut ctx);
            cycle_count += cycles;
        }

        let elapsed = start.elapsed();
        let loop_time = Duration::from_millis(1000);
        if loop_time > elapsed {
            let sleep_time = loop_time - elapsed;

            let seconds : f64 =
                elapsed.as_secs() as f64 +
                1e-9 * (elapsed.subsec_nanos() as f64);

            println!("Finished in {:.08}", seconds);
            thread::sleep(sleep_time);
        } else {
            println!("Taking long!");
        }

    }
}

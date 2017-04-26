extern crate sdl2;
use std::time::{Duration, Instant};
use std::thread;
use std::env;

mod constants;
mod context;
mod cart;
mod cpu;
mod io;

use constants::CYCLES_PER_SECOND;
use context::Context;
use cart::Cartridge;
use cpu::mmu;

fn main() {

    let argv = env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        println!("Usage: {} <rom_file>",argv[0]);
        return;
    }

    let mut ctx = Context::new(argv[1].clone());
    println!("read cartridge: {:?}",ctx.cart());


    ctx.reset();
    loop {
        let start = Instant::now();

        /* Run number of cycles required */
        let mut cycle_count = 0;
        while cycle_count < CYCLES_PER_SECOND {
            //println!("reg {:?}",ctx.reg());
            let cycles = cpu::step(&mut ctx);
            mmu::step(&mut ctx,cycles);
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
        }

    }
}

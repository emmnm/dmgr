use std::string::String;
use cart::Cartridge;
use cpu::{Interrupts,Registers};
use io::{Serial,Lcd,Timer,Gpu,Sound,Joypad};
use constants::read_bytes_from_file;

pub struct Context {
    //store io structs.
    lcd: Lcd,
    gpu: Gpu,
    sound: Sound,
    serial: Serial,
    timer: Timer,
    joypad: Joypad,
    cartridge: Cartridge,
    registers: Registers,
    interrupts: Interrupts,
    inbios: bool,
    bios: Vec<u8>,
    work_bank_0: Vec<u8>,
    work_bank_1: Vec<u8>,
    hram: Vec<u8>,

    pub suspend: bool,
}

impl Context {

    pub fn new(file_path: String) -> Context {
        Context {
            bios: read_bytes_from_file(String::from("roms/DMG_ROM.bin")),
            cartridge: Cartridge::new(file_path),
            registers: Registers::new(),
            interrupts: Interrupts::new(),
            serial: Serial::new(),
            sound: Sound::new(),
            joypad: Joypad::new(),
            gpu: Gpu::new(),
            lcd: Lcd::new(),
            timer: Timer::new(),
            suspend: false,
            work_bank_0: vec![0; 0x1000],
            work_bank_1: vec![0; 0x1000],
            hram: vec![0; 0x2000],
            inbios: true,
        }
    }

    pub fn reset(&mut self) {
        self.registers.reset();
        self.interrupts.reset();
        self.inbios = true;
    }

    pub fn cart(&mut self) -> &mut Cartridge {
        &mut self.cartridge
    }

    pub fn reg(&mut self) -> &mut Registers {
        &mut self.registers
    }

    pub fn ints(&mut self) -> &mut Interrupts {
        &mut self.interrupts
    }

    pub fn serial(&mut self) -> &mut Serial {
        &mut self.serial
    }

    pub fn sound(&mut self) -> &mut Sound {
        &mut self.sound
    }
    pub fn gpu(&mut self) -> &mut Gpu {
        &mut self.gpu
    }
    pub fn lcd(&mut self) -> &mut Lcd {
        &mut self.lcd
    }
    pub fn timer(&mut self) -> &mut Timer {
        &mut self.timer
    }
    pub fn joypad(&mut self) -> &mut Joypad {
        &mut self.joypad
    }

    pub fn wb0(&mut self) -> &mut Vec<u8> {
        &mut self.work_bank_0
    }
    pub fn wb1(&mut self) -> &mut Vec<u8> {
        &mut self.work_bank_1
    }
    pub fn hram(&mut self) -> &mut Vec<u8> {
        &mut self.hram
    }

    pub fn bios(&mut self) -> &mut Vec<u8> {
        &mut self.bios
    }

    pub fn in_bios(&mut self) -> bool {
        self.inbios
    }

    pub fn leave_bios(&mut self) {
        self.inbios = false
    }

}

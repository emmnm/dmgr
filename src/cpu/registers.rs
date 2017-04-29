use context::Context;
use cpu::mmu;
use std::fmt;

use self::ByteRegister::{A,C,IMM};

pub struct Registers {
    a: u8, f:u8,
    b: u8, c:u8,
    d: u8, e:u8,
    h: u8, l:u8,
    pc: u16,
    sp: u16,
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ af: {:02X}{:02X}, bc: {:02X}{:02X}, de: {:02X}{:02X}, hl: {:02X}{:02X}, sp: {:04X}, pc: {:04X} }}",
            self.a,self.f,self.b,self.c,self.d,self.e,self.h,self.l,self.sp,self.pc )
    }
}

#[derive(Copy,Clone,Debug)]
pub enum ByteRegister {
    A,F,    B,C,
    D,E,    H,L,
    IMM,
    MEM(WordRegister),
    MEMDEC(WordRegister),
    MEMINC(WordRegister),
    MEM_FF00_C,
    MEM_FF00_IMM,
}

#[derive(Copy,Clone,Debug)]
pub enum WordRegister {
    AF, BC, DE, HL,
    SP, PC, DIMM,
}

#[derive(Copy,Clone,Debug)]
pub enum FlagType {
    Zf, //bit 7
    Nf, //bit 6
    Hf, //bit 5
    Cf, //bit 4.
}

impl Registers {

    pub fn new() -> Registers {
        Registers {
            a: 0, f: 0,     b:0, c:0,
            d: 0, e: 0,     h:0, l:0,
            pc: 0, sp: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0x0000;
    }
}

pub fn getb(ctx:&mut Context, r:ByteRegister) -> u8 {
    match r {
        ByteRegister::A => ctx.reg().a,
        ByteRegister::F => ctx.reg().f,
        ByteRegister::B => ctx.reg().b,
        ByteRegister::C => ctx.reg().c,
        ByteRegister::D => ctx.reg().d,
        ByteRegister::E => ctx.reg().e,
        ByteRegister::H => ctx.reg().h,
        ByteRegister::L => ctx.reg().l,
        ByteRegister::IMM => {let pc = ctx.reg().pc; ctx.reg().pc += 1; mmu::read_byte(ctx,pc) },
        ByteRegister::MEM(wr) => {
            let addr = getw(ctx,wr);
            mmu::read_byte(ctx,addr)
        }
        ByteRegister::MEMINC(wr) => {
            let addr = getw(ctx,wr);
            setw(ctx,wr,addr+1);
            mmu::read_byte(ctx,addr)
        }
        ByteRegister::MEMDEC(wr) => {
            let addr = getw(ctx,wr);
            setw(ctx,wr,addr-1);
            mmu::read_byte(ctx,addr)
        }
        ByteRegister::MEM_FF00_C => {
            let addr = 0xFF00u16 + getb(ctx,C) as u16;
            mmu::read_byte(ctx,addr)
        }
        ByteRegister::MEM_FF00_IMM => {
            let addr = 0xFF00u16 + getb(ctx,IMM) as u16;
            mmu::read_byte(ctx,addr)
        }
        //_ => {panic!("Not supported {:?}",r)}
    }
}

pub fn setb(ctx:&mut Context, r:ByteRegister,val:u8) {
    match r {
        ByteRegister::A => ctx.reg().a = val,
        ByteRegister::F => ctx.reg().f = val,
        ByteRegister::B => ctx.reg().b = val,
        ByteRegister::C => ctx.reg().c = val,
        ByteRegister::D => ctx.reg().d = val,
        ByteRegister::E => ctx.reg().e = val,
        ByteRegister::H => ctx.reg().h = val,
        ByteRegister::L => ctx.reg().l = val,
        ByteRegister::MEM(wr) => {
            let addr = getw(ctx,wr);
            mmu::write_byte(ctx,addr,val);
        }
        ByteRegister::MEMINC(wr) => {
            let addr = getw(ctx,wr);
            mmu::write_byte(ctx,addr,val);
            setw(ctx,wr,addr+1);
        }
        ByteRegister::MEMDEC(wr) => {
            let addr = getw(ctx,wr);
            mmu::write_byte(ctx,addr,val);
            setw(ctx,wr,addr-1);
        }
        ByteRegister::MEM_FF00_C => {
            let addr = 0xFF00u16 + getb(ctx,C) as u16;
            mmu::write_byte(ctx,addr,val);
        }
        ByteRegister::MEM_FF00_IMM => {
            let addr = 0xFF00u16 + getb(ctx,IMM) as u16;
            mmu::write_byte(ctx,addr,val);
        }
        _ => {panic!("Not supported {:?}",r)}
    }
}

pub fn getf(ctx:&mut Context, f:FlagType) -> bool {
    let n = match f {
        FlagType::Zf => 7,
        FlagType::Nf => 6,
        FlagType::Hf => 5,
        FlagType::Cf => 4,
    };
    ctx.reg().f & (0x01 << n) > 0x00
}

pub fn setf(ctx:&mut Context, f:FlagType, val: bool) {
    let n = match f {
        FlagType::Zf => 7,
        FlagType::Nf => 6,
        FlagType::Hf => 5,
        FlagType::Cf => 4,
    };
    let reg = ctx.reg();
    let bit = if val {0x01} else {0x00};
    reg.f = !(0x01 << n) & reg.f | (0x01 & bit) << n
}

pub fn getw(ctx:&mut Context, r:WordRegister) -> u16 {
    match r {
        WordRegister::AF => {
            ((ctx.reg().a as u16) << 8) | (ctx.reg().f as u16)
        }
        WordRegister::BC => {
            ((ctx.reg().b as u16) << 8) | (ctx.reg().c as u16)
        }
        WordRegister::DE => {
            ((ctx.reg().d as u16) << 8) | (ctx.reg().e as u16)
        }
        WordRegister::HL => {
            ((ctx.reg().h as u16) << 8) | (ctx.reg().l as u16)
        }
        WordRegister::PC => ctx.reg().pc,
        WordRegister::SP => ctx.reg().sp,
        WordRegister::DIMM => {
            let addr = ctx.reg().pc;
            ctx.reg().pc += 2;
            mmu::read_word(ctx,addr)
        },
    }
}

pub fn setw(ctx:&mut Context, r:WordRegister, val:u16) {
    let low = val as u8;
    let high = (val >> 8) as u8;
    match r {
        WordRegister::AF => { ctx.reg().a = high; ctx.reg().f = 0xF0 & low; }
        WordRegister::BC => { ctx.reg().b = high; ctx.reg().c = low; }
        WordRegister::DE => { ctx.reg().d = high; ctx.reg().e = low; }
        WordRegister::HL => { ctx.reg().h = high; ctx.reg().l = low; }
        WordRegister::PC => ctx.reg().pc = val,
        WordRegister::SP => ctx.reg().sp = val,

        _ => panic!("not implemented {:?}",r)
    }
}

pub fn from_stack(ctx: &mut Context) -> u16 {
    let sp = ctx.reg().sp;
    let val = mmu::read_word(ctx,sp);
    ctx.reg().sp += 2;
    val
}

pub fn to_stack(ctx: &mut Context, val:u16) {
    ctx.reg().sp -= 2;
    let sp = ctx.reg().sp;
    mmu::write_word(ctx,sp,val);
}

use context::Context;
use cpu::mmu;
use cpu::{OPS,CBOPS};
use cpu::registers::{getb,setb,getw,setw,getf,setf,from_stack,to_stack,ByteRegister,WordRegister,FlagType};
use cpu::registers::ByteRegister::*;
use cpu::registers::WordRegister::*;
use cpu::registers::FlagType::*;

pub fn step(ctx: &mut Context) -> usize {
    let mut pc = getw(ctx,PC);
    setw(ctx,PC,pc+1);
    let mut opcode = mmu::read_byte(ctx,pc);
    print!("0x{:04X}\t",pc);

    let (time, func) = if opcode == 0xCB {
        print!("0xCB");
        opcode = mmu::read_byte(ctx,pc+1);
        setw(ctx,PC,pc+2);
        CBOPS[opcode as usize]
    } else {
        print!("0x");
        OPS[opcode as usize]
    };
    println!("{:02X}",opcode);
    let actual = func(ctx);
    if actual > time {
        actual
    } else {
        time
    }
}

pub fn handle_interrupts(ctx:&mut Context) {
    //panic!("NEED TO HANDLE INTERRUPTS");
}

pub fn fail(ctx: &mut Context) -> usize {
    panic!("Supposed to fail!");
}

pub fn halt(ctx: &mut Context) -> usize {
    panic!("halt!");
    1
}

pub fn nop(ctx: &mut Context) -> usize {
    1
}

pub fn ldb(ctx: &mut Context, to:ByteRegister, from:ByteRegister) -> usize {
    let val = getb(ctx,from);
    setb(ctx,to,val);
    0
}

pub fn ldw(ctx: &mut Context, to:WordRegister,from:WordRegister) -> usize {
    let val = getw(ctx,from);
    setw(ctx,to,val);
    0
}

pub fn incb(ctx: &mut Context, r: ByteRegister) -> usize {
    let val = getb(ctx,r) as u16;
    setb(ctx,r,(val + 1) as u8);
    setf(ctx,Zf,val == 0xFF);
    setf(ctx,Nf,false);
    setf(ctx,Hf,0x0F & val + 1 > 0x0F);
    1
}

pub fn decb(ctx:&mut Context, r:ByteRegister) -> usize {
    let val = getb(ctx,r);
    let new = val as i32 - 1;
    setb(ctx,r,new as u8);
    setf(ctx,Zf,new == 0x00);
    setf(ctx,Nf,true);
    setf(ctx,Hf,((0x0f & val) as i32 - 1) < 0);
    1
}

pub fn incw(ctx:&mut Context, r:WordRegister) -> usize {
    let val = getw(ctx,r) as i32;
    setw(ctx,r,(val+1) as u16);
    2
}
pub fn decw(ctx:&mut Context, r:WordRegister) -> usize {
    let val = getw(ctx,r) as i32;
    setw(ctx,r,(val-1) as u16);
    2
}

pub fn add(ctx:&mut Context, r:ByteRegister) -> usize {
    panic!("Add not implemented");
    1
}
pub fn adc(ctx:&mut Context, r:ByteRegister) -> usize {
    panic!("Adc not implemented");
    1
}

pub fn sub(ctx:&mut Context, r:ByteRegister) -> usize {
    let (av,rv) = (getb(ctx,A),getb(ctx,r));
    let res = av.wrapping_sub(rv);
    setb(ctx,A,res);
    setf(ctx,Zf,res == 0x00);
    setf(ctx,Nf,true);
    setf(ctx,Hf, (av & 0x0f) < (rv & 0xf));
    setf(ctx,Cf, av < rv);
    1
}

pub fn cp(ctx: &mut Context, r:ByteRegister) -> usize {
    let a_val = getb(ctx,A);
    let r_val = getb(ctx,r);
    setf(ctx,Zf,a_val == r_val);
    setf(ctx,Nf,true);
    setf(ctx,Hf,(a_val & 0x0f) < (r_val & 0x0f));
    setf(ctx,Cf,a_val < r_val);
    1
}





pub fn xor(ctx: &mut Context, r: ByteRegister) -> usize {
    let a_val = getb(ctx,A);
    let r_val = getb(ctx,r);
    setb(ctx,A,a_val ^ r_val);
    setb(ctx,F,0x00);
    setf(ctx,Zf,a_val == r_val);
    0
}

pub fn and(ctx: &mut Context, r: ByteRegister) -> usize {
    let a_val = getb(ctx,A);
    let r_val = getb(ctx,r);
    setb(ctx,A,a_val & r_val);
    setb(ctx,F,0x00);
    setf(ctx,Hf,true);
    setf(ctx,Zf,a_val == r_val);
    0
}

pub fn bit(ctx: &mut Context, idx:usize, r: ByteRegister) -> usize {
    let r_val = getb(ctx,r);
    setf(ctx,Zf,(r_val & (0x01 << idx)) == 0x00);
    setf(ctx,Nf,false);
    setf(ctx,Hf,true);
    0
}

pub fn rla(ctx:&mut Context) -> usize {
    let low = if getf(ctx,Cf) {1} else {0};
    let value = getb(ctx,A);
    let result = (value << 1) | low;
    setb(ctx,A,result);
    setf(ctx,Zf,false);
    setf(ctx,Nf,false);
    setf(ctx,Hf,false);
    setf(ctx,Cf,(value & 0x80) > 0x00);
    1
}

pub fn rl(ctx: &mut Context, r:ByteRegister) -> usize {
    let val = getb(ctx,r);
    let low = if getf(ctx,Cf) {1} else {0};
    let res = (val << 1) + low;
    setb(ctx,r,res);
    setf(ctx,Zf,res == 0x00);
    setf(ctx,Nf,false);
    setf(ctx,Hf,false);
    setf(ctx,Cf,0x80 & val > 0x00);
    2
}

pub fn jr(ctx: &mut Context) -> usize {
    let imm = (getb(ctx,IMM) as i8) as i32;
    let signed_pc = getw(ctx,PC) as i32;
    setw(ctx,PC,(signed_pc + imm) as u16);
    3
}

pub fn jrf(ctx: &mut Context, f:FlagType, cond:bool) -> usize {
    let imm = (getb(ctx,IMM) as i8) as i32;
    if getf(ctx,f) == cond {
        let signed_pc = getw(ctx,PC) as i32;
        setw(ctx,PC,(signed_pc + imm) as u16);
        3
    } else {
        2
    }
}

pub fn push(ctx: &mut Context, r:WordRegister) -> usize {
    let val = getw(ctx,r);
    to_stack(ctx,val);
    4
}

pub fn pop(ctx: &mut Context, r:WordRegister) -> usize {
    let val = from_stack(ctx);
    setw(ctx,r,val);
    3
}

pub fn call(ctx: &mut Context) -> usize {
    let imm = getw(ctx,DIMM);
    let pc = getw(ctx,PC);
    to_stack(ctx,pc);
    setw(ctx,PC,imm);
    6
}

pub fn callf(ctx: &mut Context, f:FlagType, cond:bool) -> usize {
    panic!("not implemented")
}

pub fn ret(ctx: &mut Context) -> usize {
    let addr = from_stack(ctx);
    setw(ctx,PC,addr);
    4
}

pub fn retf(ctx: &mut Context, f:FlagType, cond:bool) -> usize {
    panic!("not implemented");
    2
}

use context::Context;
use cpu::mmu;
use cpu::{OPS,CBOPS};
use cpu::registers::{getb,setb,getw,setw,getf,setf,from_stack,to_stack,ByteRegister,WordRegister,FlagType};
use cpu::registers::ByteRegister::*;
use cpu::registers::WordRegister::*;
use cpu::registers::FlagType::*;

static mut CURR_OP: u8 = 0;

pub fn step(ctx: &mut Context) -> usize {
    if ctx.suspend {
        return 1;
    }
    let mut pc = getw(ctx,PC);
    setw(ctx,PC,pc+1);
    let mut opcode = mmu::read_byte(ctx,pc);

    let (time, func) = if opcode == 0xCB {
        opcode = mmu::read_byte(ctx,pc+1);
        setw(ctx,PC,pc+2);
        CBOPS[opcode as usize]
    } else {
        OPS[opcode as usize]
    };

    unsafe {
        CURR_OP = opcode;

    }
    let actual = func(ctx);
    if actual > time {
        actual
    } else {
        time
    }
}

pub fn handle_interrupts(ctx:&mut Context) {
    if ctx.ints().is_master_enabled() {
        let filtered = ctx.ints().read_filtered();
        if filtered > 0x00 {
            ctx.suspend = false;
        }

        if 0x01 & filtered > 0 {
            ctx.ints().disable();
            ctx.ints().clear(0x01);
            rst(ctx,0x40);
        } else if 0x02 & filtered > 0 {
            ctx.ints().disable();
            ctx.ints().clear(0x02);
            rst(ctx,0x48);
        } else if 0x04 & filtered > 0 {
            ctx.ints().disable();
            ctx.ints().clear(0x04);
            rst(ctx,0x50);
        } else if 0x08 & filtered > 0 {
            ctx.ints().disable();
            ctx.ints().clear(0x08);
            rst(ctx,0x58);
        } else if 0x10 & filtered > 0 {
            ctx.ints().disable();
            ctx.ints().clear(0x10);
            rst(ctx,0x60);
        }
    }
}

pub fn fail(ctx: &mut Context) -> usize {
    unsafe {
        panic!("Supposed to fail! 0x{:02X}",CURR_OP);
    }
}

pub fn invalid(ctx: &mut Context) -> usize {
    panic!("Invalid instruction");
}

pub fn stop(ctx: &mut Context) -> usize {
    panic!("Turning off machine");
}

pub fn halt(ctx: &mut Context) -> usize {
    ctx.suspend = true;
    1
}

pub fn nop(ctx: &mut Context) -> usize {
    1
}

pub fn daa(ctx:&mut Context) -> usize {
    let mut a = getb(ctx,A) as i32;
    if getf(ctx,Nf) {
        if getf(ctx,Hf) { a = (a - 0x06) & 0xFF }
        if getf(ctx,Cf) { a -= 0x60 }
    } else {
        if getf(ctx,Hf) || (a & 0x0F) > 9 { a += 0x06 }
        if getf(ctx,Cf) || a > 0x9F { a += 0x60 }
    }
    let mut newflags = !(0x80 | 0x20);
    if (a & 0x100) == 0x100 {
        newflags |= 0x10;
    }
    setb(ctx,A,a as u8);
    setf(ctx,Zf,(a as u8) == 0x00);
    setf(ctx,Hf,false);
    setf(ctx,Cf,a > 0x100);
    1
}

pub fn di(ctx: &mut Context) -> usize {
    ctx.ints().disable();
    1
}

pub fn ei(ctx: &mut Context) -> usize {
    ctx.ints().enable();
    1
}

pub fn ldb(ctx: &mut Context, to:ByteRegister, from:ByteRegister) -> usize {
    let val = getb(ctx,from);
    setb(ctx,to,val);
    1
}

pub fn ldw(ctx: &mut Context, to:WordRegister,from:WordRegister) -> usize {
    let val = getw(ctx,from);
    setw(ctx,to,val);
    0
}

pub fn ldimm_sp(ctx:&mut Context) -> usize {
    let addr = getw(ctx,DIMM);
    let sp = getw(ctx,SP);
    mmu::write_byte(ctx,addr+0,sp as u8);
    mmu::write_byte(ctx,addr+1,(sp >> 8) as u8);
    5
}

//TODO: Correct these flags!
pub fn ldhl_sp_imm(ctx:&mut Context) -> usize {
    let sp = getw(ctx,SP) as i32;
    let imm = getb(ctx,IMM) as i32;
    let result = (sp + imm) as u16;
    setw(ctx,HL,result);
    setb(ctx,F,0);
    //setf(ctx,Hf,false);
    //setf(ctx,Cf,(0xFF & sp));
    3
}

pub fn incb(ctx: &mut Context, r: ByteRegister) -> usize {
    let val = getb(ctx,r) as u16;
    setb(ctx,r,(val + 1) as u8);
    setf(ctx,Zf,val == 0xFF);
    setf(ctx,Nf,false);
    setf(ctx,Hf,(0x0F & val) + 1 > 0x0F);
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
    let (tv, fv) = (getb(ctx,A),getb(ctx,r));
    let res = (tv as u16) + (fv as u16);
    setb(ctx,A,res as u8);
    setf(ctx,Zf,(res as u8) == 0);
    setf(ctx,Nf,false);
    setf(ctx,Hf,((tv & 0x0f) + (fv & 0x0f)) > 0x0f);
    setf(ctx,Cf,(res & 0xff00) > 0);
    1
}

pub fn addw(ctx:&mut Context, to:WordRegister, from:WordRegister) -> usize {
    let (tv,fv) = (getw(ctx,to) as u32, getw(ctx,from) as u32);
    let result = tv + fv;
    setw(ctx,to,result as u16);
    setf(ctx,Nf,false);
    setf(ctx,Hf,(tv & 0x0fff) + (fv & 0x0fff) > 0x0fff);
    setf(ctx,Cf,result & 0xffff0000 > 0);
    2
}

pub fn adc(ctx:&mut Context, r:ByteRegister) -> usize {
    let (av,rv) = (getb(ctx,A),getb(ctx,r));
    let cv = if getf(ctx,Cf) {1} else {0};
    let res = (av as u16) + (rv as u16) + (cv as u16);
    setb(ctx,A,res as u8);
    setf(ctx,Zf,res as u8 == 0x00);
    setf(ctx,Nf,false);
    setf(ctx,Hf,((av & 0x0f) + (rv & 0x0f) + (cv & 0x0f)) > 0x0f);
    setf(ctx,Cf,res & 0xff00 > 0);
    1
}

pub fn add_sp_imm(ctx:&mut Context) -> usize {
    let imm = getb(ctx,IMM) as i32;
    let sp = getw(ctx,SP) as i32;
    let result = (sp + imm) as u16;
    setw(ctx,SP,result);
    setb(ctx,F,0);
    setf(ctx,Hf,(0x0FFF & sp) + (0x0FFF & imm) > 0x0FFF);
    setf(ctx,Cf,sp + imm > 0x0FFF);
    4
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

pub fn sbc(ctx:&mut Context, r:ByteRegister) -> usize {
    let av = getb(ctx,A) as i32;
    let rv = getb(ctx,r) as i32;
    let cv = if getf(ctx,Cf) {1} else {0} as i32;
    let result = (av - rv - cv) as u8;
    setb(ctx,A,result);
    setf(ctx,Zf,result == 0x00);
    setf(ctx,Nf,true);
    setf(ctx,Hf,((0x0F & av) - (0x0F & rv) - cv < 0));
    setf(ctx,Cf,av - rv - cv < 0);
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

pub fn cpl(ctx: &mut Context) -> usize {
    let val = getb(ctx,A);
    setb(ctx,A,!val);
    setf(ctx,Nf,true);
    setf(ctx,Hf,true);
    1
}

pub fn scf(ctx: &mut Context) -> usize {
    setf(ctx,Nf,false);
    setf(ctx,Hf,false);
    setf(ctx,Cf,true);
    1
}
pub fn ccf(ctx: &mut Context) -> usize {
    let carry = getf(ctx,Cf);
    setf(ctx,Cf,!carry);
    setf(ctx,Hf,false);
    setf(ctx,Nf,false);
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
    let res = a_val & r_val;
    setb(ctx,A,res);
    setb(ctx,F,0x00);
    setf(ctx,Zf,res == 0x00);
    setf(ctx,Hf,true);
    0
}

pub fn or(ctx: &mut Context, r: ByteRegister) -> usize {
    let a_val = getb(ctx,A);
    let r_val = getb(ctx,r);
    let res = a_val | r_val;
    setb(ctx,A,res);
    setb(ctx,F,0);
    setf(ctx,Zf,res == 0x00);
    1
}

pub fn swap(ctx:&mut Context, r:ByteRegister) -> usize {
    let value = getb(ctx,r);
    let result = ((value & 0xf) << 4) | ((value & 0xf0) >> 4);
    setb(ctx,r,result);
    setb(ctx,F,0);
    setf(ctx,Zf,result == 0x00);
    2
}

pub fn bit(ctx: &mut Context, idx:usize, r: ByteRegister) -> usize {
    let r_val = getb(ctx,r);
    setf(ctx,Zf,(r_val & (0x01 << idx)) == 0x00);
    setf(ctx,Nf,false);
    setf(ctx,Hf,true);
    2
}

pub fn res(ctx: &mut Context, idx:usize, r:ByteRegister) -> usize {
    let value = getb(ctx,r);
    let result = value & !(1 << idx);
    setb(ctx,r,result);
    2
}

pub fn set(ctx: &mut Context, idx:usize, r:ByteRegister) -> usize {
    let value = getb(ctx,r);
    let result = value | (1 << idx);
    setb(ctx,r,result);
    2
}

pub fn rla(ctx:&mut Context) -> usize {
    let low = if getf(ctx,Cf) {1} else {0};
    let value = getb(ctx,A);
    let result = (value << 1) | low;
    setb(ctx,A,result);
    setb(ctx,F,0);
    setf(ctx,Cf,(value & 0x80) > 0x00);
    1
}

pub fn rl(ctx: &mut Context, r:ByteRegister) -> usize {
    let val = getb(ctx,r);
    let low = if getf(ctx,Cf) {1} else {0};
    let res = (val << 1) + low;
    setb(ctx,r,res);
    setb(ctx,F,0);
    setf(ctx,Zf,res == 0x00);
    setf(ctx,Cf,0x80 & val > 0x00);
    2
}

pub fn rlc(ctx: &mut Context, r:ByteRegister) -> usize {
    let val = getb(ctx,r);
    let carry = (val & 0x80) >> 7;
    let result = (val << 1) + carry;
    setb(ctx,r,result);
    setb(ctx,F,0);
    setf(ctx,Zf,result == 0x00);
    setf(ctx,Cf,carry != 0x00);
    2
}

pub fn rr(ctx: &mut Context, r:ByteRegister) -> usize {
    let value = getb(ctx,r);
    let result = (value >> 1) |
        if getf(ctx,Cf) { 0x80 } else {0x00};
    setb(ctx,r,result);
    setb(ctx,F,0);
    setf(ctx,Zf,result == 0x00);
    setf(ctx,Cf,(value & 0x01) > 0x00);
    2
}

pub fn rra(ctx:&mut Context) -> usize {
    let carry = if getf(ctx,Cf) {0x80} else {0x00};
    let value = getb(ctx,A);
    let result = (value >> 1) + carry;
    setb(ctx,A,result);
    setb(ctx,F,0);
    setf(ctx,Cf,(value & 0x01) > 0x00);
    1
}

pub fn rrc(ctx: &mut Context, r:ByteRegister) -> usize {
    let mut value = getb(ctx,r);
    let carry = value & 0x01;
    value >>= 1;
    if carry != 0x00 {
        value |= 0x80;
    };
    setb(ctx,r,value);
    setb(ctx,F,0);
    setf(ctx,Zf,value == 0);
    setf(ctx,Cf,carry != 0x00);
    2
}

pub fn rlca(ctx:&mut Context) -> usize {
    let val = getb(ctx,A);
    let high = (val & 0x80) >> 7;
    let res = (val << 1) | high;
    setb(ctx,A,res);
    setb(ctx,F,0);
    setf(ctx,Cf,high > 0x00);
    1
}

pub fn rrca(ctx:&mut Context) -> usize {
    let val = getb(ctx,A);
    let carry = 0x01 & val;
    let mut result = val >> 1;
    if carry != 0x00 {
        result |= 0x80;
    }
    setb(ctx,A,result);
    setb(ctx,F,0);
    setf(ctx,Cf,carry != 0x00);
    1
}

pub fn sla(ctx:&mut Context, r:ByteRegister) -> usize {
    let value = getb(ctx,r);
    let result = value << 1;
    setb(ctx,r,result);
    setb(ctx,F,0);
    setf(ctx,Zf,result == 0x00);
    setf(ctx,Cf,value & 0x80 > 0x00);
    2
}

pub fn sra(ctx:&mut Context, r:ByteRegister) -> usize {
    let value = getb(ctx,r);
    let result = (value & 0x80) | (value >> 1);
    setb(ctx,r,result);
    setb(ctx,F,0);
    setf(ctx,Zf,result == 0x00);
    setf(ctx,Cf,value & 0x01 != 0x00);
    2
}

pub fn srl(ctx:&mut Context, r:ByteRegister) -> usize {
    let value = getb(ctx,r);
    let result = value >> 1;
    setb(ctx,r,result);
    setb(ctx,F,0);
    setf(ctx,Zf,result == 0x00);
    setf(ctx,Cf,value & 0x01 != 0x00);
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

pub fn jp(ctx: &mut Context) -> usize {
    let addr = getw(ctx,DIMM);
    setw(ctx,PC,addr);
    4
}

pub fn jphl(ctx: &mut Context) -> usize {
    let addr = getw(ctx,HL);
    setw(ctx,PC,addr);
    1
}

pub fn jpf(ctx: &mut Context, f:FlagType, cond:bool) -> usize {
    let addr = getw(ctx,DIMM);
    if getf(ctx,f) == cond {
        setw(ctx,PC,addr);
        4
    } else {
        3
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
    let imm = getw(ctx,DIMM);
    if getf(ctx,f) == cond {
        let pc = getw(ctx,PC);
        to_stack(ctx,pc);
        setw(ctx,PC,imm);
        6
    } else {
        3
    }
}

pub fn ret(ctx: &mut Context) -> usize {
    let addr = from_stack(ctx);
    setw(ctx,PC,addr);
    4
}

pub fn retf(ctx: &mut Context, f:FlagType, cond:bool) -> usize {
    if getf(ctx,f) == cond {
        let addr = from_stack(ctx);
        setw(ctx,PC,addr);
        5
    } else {
        2
    }
}

pub fn reti(ctx:&mut Context) -> usize {
    ctx.ints().enable();
    let addr = from_stack(ctx);
    setw(ctx,PC,addr);
    4
}

pub fn rst(ctx:&mut Context, addr:u16) -> usize {
    let pc = getw(ctx,PC);
    to_stack(ctx,pc);
    setw(ctx,PC,addr);
    4
}

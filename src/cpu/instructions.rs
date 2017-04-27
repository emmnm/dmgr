use cpu::registers::ByteRegister::*;
use cpu::registers::WordRegister::*;
use cpu::registers::FlagType::*;
use context::Context;
use cpu::z80::*;


pub const OPS:[(usize,&'static Fn(&mut Context) -> usize); 256] = [

    // 0X
    (1, &|ctx| {nop(ctx)}), // X0
    (3, &|ctx| {ldw(ctx,BC,DIMM)}), // X1
    (2, &|ctx| {ldb(ctx,MEM(BC),A)}), // X2
    (2, &|ctx| {incw(ctx,BC)}), // X3
    (1, &|ctx| {incb(ctx,B)}), // X4
    (1, &|ctx| {decb(ctx,B)}), // X5
    (1, &|ctx| {ldb(ctx,B,IMM)}), // X6
    (1, &|ctx| {rlca(ctx)}), // X7

    (1, &|ctx| {fail(ctx)}), // X8

    (2, &|ctx| {addw(ctx,HL,BC)}), // X9
    (2, &|ctx| {ldb(ctx,A,MEM(BC))}), // XA
    (2, &|ctx| {decw(ctx,BC)}), // XB
    (1, &|ctx| {incb(ctx,C)}), // XC
    (1, &|ctx| {decb(ctx,C)}), // XD
    (2, &|ctx| {ldb(ctx,C,IMM)}), // XE
    (1, &|ctx| {rrca(ctx)}), // XF

    // 1X
    (1, &|ctx| {stop(ctx)}), // X0
    (3, &|ctx| {ldw(ctx,DE,DIMM)}), // X1
    (2, &|ctx| {ldb(ctx,MEM(DE),A)}), // X2
    (2, &|ctx| {incw(ctx,DE)}), // X3
    (1, &|ctx| {incb(ctx,D)}), // X4
    (1, &|ctx| {decb(ctx,D)}), // X5
    (2, &|ctx| {ldb(ctx,D,IMM)}), // X6
    (1, &|ctx| {rla(ctx)}), // X7
    (3, &|ctx| {jr(ctx)}), // X8
    (2, &|ctx| {addw(ctx,HL,DE)}), // X9
    (2, &|ctx| {ldb(ctx,A,MEM(DE))}), // XA
    (2, &|ctx| {decw(ctx,DE)}), // XB
    (1, &|ctx| {incb(ctx,E)}), // XC
    (1, &|ctx| {decb(ctx,E)}), // XD
    (2, &|ctx| {ldb(ctx,E,IMM)}), // XE
    (1, &|ctx| {rra(ctx)}), // XF

    // 2X
    (2, &|ctx| {jrf(ctx,Zf,false)}), // X0
    (3, &|ctx| {ldw(ctx,HL,DIMM)}), // X1
    (2, &|ctx| {ldb(ctx,MEMINC(HL),A)}), // X2
    (2, &|ctx| {incw(ctx,HL)}), // X3
    (1, &|ctx| {incb(ctx,H)}), // X4
    (1, &|ctx| {decb(ctx,H)}), // X5
    (2, &|ctx| {ldb(ctx,H,IMM)}), // X6
    (1, &|ctx| {daa(ctx)}), // X7
    (2, &|ctx| {jrf(ctx,Zf,true)}), // X8
    (1, &|ctx| {addw(ctx,HL,HL)}), // X9
    (2, &|ctx| {ldb(ctx,A,MEMINC(HL))}), // XA
    (1, &|ctx| {decw(ctx,HL)}), // XB
    (1, &|ctx| {incb(ctx,L)}), // XC
    (1, &|ctx| {decb(ctx,L)}), // XD
    (2, &|ctx| {ldb(ctx,L,IMM)}), // XE
    (1, &|ctx| {cpl(ctx)}), // XF

    // 3X
    (2, &|ctx| {jrf(ctx,Cf,false)}), // X0
    (3, &|ctx| {ldw(ctx,SP,DIMM)}), // X1
    (2, &|ctx| {ldb(ctx,MEMDEC(HL),A)}), // X2
    (1, &|ctx| {incw(ctx,SP)}), // X3
    (3, &|ctx| {incb(ctx,MEM(HL))}), // X4
    (3, &|ctx| {decb(ctx,MEM(HL))}), // X5
    (3, &|ctx| {ldb(ctx,MEM(HL),IMM)}), // X6
    (1, &|ctx| {scf(ctx)}), // X7
    (2, &|ctx| {jrf(ctx,Cf,true)}), // X8
    (2, &|ctx| {addw(ctx,HL,SP)}), // X9
    (2, &|ctx| {ldb(ctx,A,MEMDEC(HL))}), // XA
    (2, &|ctx| {decw(ctx,SP)}), // XB
    (1, &|ctx| {incb(ctx,A)}), // XC
    (1, &|ctx| {decb(ctx,A)}), // XD
    (2, &|ctx| {ldb(ctx,A,IMM)}), // XE
    (1, &|ctx| {ccf(ctx)}), // XF

    // 4X
    (1, &|ctx| {ldb(ctx,B,B)}), // X0
    (1, &|ctx| {ldb(ctx,B,C)}), // X1
    (1, &|ctx| {ldb(ctx,B,D)}), // X2
    (1, &|ctx| {ldb(ctx,B,E)}), // X3
    (1, &|ctx| {ldb(ctx,B,H)}), // X4
    (1, &|ctx| {ldb(ctx,B,L)}), // X5
    (2, &|ctx| {ldb(ctx,B,MEM(HL))}), // X6
    (1, &|ctx| {ldb(ctx,B,A)}), // X7
    (1, &|ctx| {ldb(ctx,C,B)}), // X8
    (1, &|ctx| {ldb(ctx,C,C)}), // X9
    (1, &|ctx| {ldb(ctx,C,D)}), // XA
    (1, &|ctx| {ldb(ctx,C,E)}), // XB
    (1, &|ctx| {ldb(ctx,C,H)}), // XC
    (1, &|ctx| {ldb(ctx,C,L)}), // XD
    (2, &|ctx| {ldb(ctx,C,MEM(HL))}), // XE
    (1, &|ctx| {ldb(ctx,C,A)}), // XF

    // 5X
    (1, &|ctx| {ldb(ctx,D,B)}), // X0
    (1, &|ctx| {ldb(ctx,D,C)}), // X1
    (1, &|ctx| {ldb(ctx,D,D)}), // X2
    (1, &|ctx| {ldb(ctx,D,E)}), // X3
    (1, &|ctx| {ldb(ctx,D,H)}), // X4
    (1, &|ctx| {ldb(ctx,D,L)}), // X5
    (2, &|ctx| {ldb(ctx,D,MEM(HL))}), // X6
    (1, &|ctx| {ldb(ctx,D,A)}), // X7
    (1, &|ctx| {ldb(ctx,E,B)}), // X8
    (1, &|ctx| {ldb(ctx,E,C)}), // X9
    (1, &|ctx| {ldb(ctx,E,D)}), // XA
    (1, &|ctx| {ldb(ctx,E,E)}), // XB
    (1, &|ctx| {ldb(ctx,E,H)}), // XC
    (1, &|ctx| {ldb(ctx,E,L)}), // XD
    (2, &|ctx| {ldb(ctx,E,MEM(HL))}), // XE
    (1, &|ctx| {ldb(ctx,E,A)}), // XF

    // 6X
    (1, &|ctx| {ldb(ctx,H,B)}), // X0
    (1, &|ctx| {ldb(ctx,H,C)}), // X1
    (1, &|ctx| {ldb(ctx,H,D)}), // X2
    (1, &|ctx| {ldb(ctx,H,E)}), // X3
    (1, &|ctx| {ldb(ctx,H,H)}), // X4
    (1, &|ctx| {ldb(ctx,H,L)}), // X5
    (2, &|ctx| {ldb(ctx,H,MEM(HL))}), // X6
    (1, &|ctx| {ldb(ctx,H,A)}), // X7
    (1, &|ctx| {ldb(ctx,L,B)}), // X8
    (1, &|ctx| {ldb(ctx,L,C)}), // X9
    (1, &|ctx| {ldb(ctx,L,D)}), // XA
    (1, &|ctx| {ldb(ctx,L,E)}), // XB
    (1, &|ctx| {ldb(ctx,L,H)}), // XC
    (1, &|ctx| {ldb(ctx,L,L)}), // XD
    (2, &|ctx| {ldb(ctx,L,MEM(HL))}), // XE
    (1, &|ctx| {ldb(ctx,L,A)}), // XF

    // 7X
    (2, &|ctx| {ldb(ctx,MEM(HL),B)}), // X0
    (2, &|ctx| {ldb(ctx,MEM(HL),C)}), // X1
    (2, &|ctx| {ldb(ctx,MEM(HL),D)}), // X2
    (2, &|ctx| {ldb(ctx,MEM(HL),E)}), // X3
    (2, &|ctx| {ldb(ctx,MEM(HL),H)}), // X4
    (2, &|ctx| {ldb(ctx,MEM(HL),L)}), // X5
    (1, &|ctx| {halt(ctx)}), // X6
    (2, &|ctx| {ldb(ctx,MEM(HL),A)}), // X7
    (1, &|ctx| {ldb(ctx,A,B)}), // X8
    (1, &|ctx| {ldb(ctx,A,C)}), // X9
    (1, &|ctx| {ldb(ctx,A,D)}), // XA
    (1, &|ctx| {ldb(ctx,A,E)}), // XB
    (1, &|ctx| {ldb(ctx,A,H)}), // XC
    (1, &|ctx| {ldb(ctx,A,L)}), // XD
    (2, &|ctx| {ldb(ctx,A,MEM(HL))}), // XE
    (1, &|ctx| {ldb(ctx,A,A)}), // XF

    // 8X
    (1, &|ctx| {add(ctx,B)}), // X0
    (1, &|ctx| {add(ctx,C)}), // X1
    (1, &|ctx| {add(ctx,D)}), // X2
    (1, &|ctx| {add(ctx,E)}), // X3
    (1, &|ctx| {add(ctx,H)}), // X4
    (1, &|ctx| {add(ctx,L)}), // X5
    (2, &|ctx| {add(ctx,MEM(HL))}), // X6
    (1, &|ctx| {add(ctx,A)}), // X7
    (1, &|ctx| {adc(ctx,B)}), // X8
    (1, &|ctx| {adc(ctx,C)}), // X9
    (1, &|ctx| {adc(ctx,D)}), // XA
    (1, &|ctx| {adc(ctx,E)}), // XB
    (1, &|ctx| {adc(ctx,H)}), // XC
    (1, &|ctx| {adc(ctx,L)}), // XD
    (2, &|ctx| {adc(ctx,MEM(HL))}), // XE
    (1, &|ctx| {adc(ctx,A)}), // XF

    // 9X
    (1, &|ctx| {sub(ctx,B)}), // X0
    (1, &|ctx| {sub(ctx,C)}), // X1
    (1, &|ctx| {sub(ctx,D)}), // X2
    (1, &|ctx| {sub(ctx,E)}), // X3
    (1, &|ctx| {sub(ctx,H)}), // X4
    (1, &|ctx| {sub(ctx,L)}), // X5
    (2, &|ctx| {sub(ctx,MEM(HL))}), // X6
    (1, &|ctx| {sub(ctx,A)}), // X7
    (1, &|ctx| {sbc(ctx,B)}), // X8
    (1, &|ctx| {sbc(ctx,C)}), // X9
    (1, &|ctx| {sbc(ctx,D)}), // XA
    (1, &|ctx| {sbc(ctx,E)}), // XB
    (1, &|ctx| {sbc(ctx,H)}), // XC
    (1, &|ctx| {sbc(ctx,L)}), // XD
    (2, &|ctx| {sbc(ctx,MEM(HL))}), // XE
    (1, &|ctx| {sbc(ctx,A)}), // XF

    // AX
    (1, &|ctx| {and(ctx,B)}), // X0
    (1, &|ctx| {and(ctx,C)}), // X1
    (1, &|ctx| {and(ctx,D)}), // X2
    (1, &|ctx| {and(ctx,E)}), // X3
    (1, &|ctx| {and(ctx,H)}), // X4
    (1, &|ctx| {and(ctx,L)}), // X5
    (2, &|ctx| {and(ctx,MEM(HL))}), // X6
    (1, &|ctx| {and(ctx,A)}), // X74

    (1, &|ctx| {xor(ctx,B)}), // X8
    (1, &|ctx| {xor(ctx,C)}), // X9
    (1, &|ctx| {xor(ctx,D)}), // XA
    (1, &|ctx| {xor(ctx,E)}), // XB
    (1, &|ctx| {xor(ctx,H)}), // XC
    (1, &|ctx| {xor(ctx,L)}), // XD
    (2, &|ctx| {xor(ctx,MEM(HL))}), // XE
    (1, &|ctx| {xor(ctx,A)}), // XF

    // BX
    (1, &|ctx| {or(ctx,B)}), // X0
    (1, &|ctx| {or(ctx,C)}), // X1
    (1, &|ctx| {or(ctx,D)}), // X2
    (1, &|ctx| {or(ctx,E)}), // X3
    (1, &|ctx| {or(ctx,H)}), // X4
    (1, &|ctx| {or(ctx,L)}), // X5
    (2, &|ctx| {or(ctx,MEM(HL))}), // X6
    (1, &|ctx| {or(ctx,A)}), // X7
    (1, &|ctx| {cp(ctx,B)}), // X8
    (1, &|ctx| {cp(ctx,C)}), // X9
    (1, &|ctx| {cp(ctx,D)}), // XA
    (1, &|ctx| {cp(ctx,E)}), // XB
    (1, &|ctx| {cp(ctx,H)}), // XC
    (1, &|ctx| {cp(ctx,L)}), // XD
    (2, &|ctx| {cp(ctx,MEM(HL))}), // XE
    (1, &|ctx| {cp(ctx,A)}), // XF

    // CX
    (2, &|ctx| {retf(ctx,Zf,false)}), // X0
    (3, &|ctx| {pop(ctx,BC)}), // X1
    (3, &|ctx| {jpf(ctx,Zf,false)}), // X2
    (4, &|ctx| {jp(ctx)}), // X3
    (3, &|ctx| {callf(ctx,Zf,false)}), // X4
    (4, &|ctx| {push(ctx,BC)}), // X5
    (2, &|ctx| {add(ctx,IMM)}), // X6
    (4, &|ctx| {rst(ctx,0x00)}), // X7
    (2, &|ctx| {retf(ctx,Zf,true)}), // X8
    (4, &|ctx| {ret(ctx)}), // X9
    (3, &|ctx| {jpf(ctx,Zf,true)}), // XA
    (1, &|ctx| {panic!("CB Prefix!")}), // XB
    (3, &|ctx| {callf(ctx,Zf,true)}), // XC
    (6, &|ctx| {call(ctx)}), // XD
    (1, &|ctx| {adc(ctx,IMM)}), // XE
    (4, &|ctx| {rst(ctx,0x08)}), // XF

    // DX
    (2, &|ctx| {retf(ctx,Cf,false)}), // X0
    (3, &|ctx| {pop(ctx,DE)}), // X1
    (3, &|ctx| {jpf(ctx,Cf,false)}), // X2
    (1, &|ctx| {invalid(ctx)}), // X3
    (3, &|ctx| {callf(ctx,Cf,false)}), // X4
    (4, &|ctx| {push(ctx,DE)}), // X5
    (2, &|ctx| {sub(ctx,IMM)}), // X6
    (4, &|ctx| {rst(ctx,0x10)}), // X7
    (2, &|ctx| {retf(ctx,Cf,true)}), // X8
    (4, &|ctx| {reti(ctx)}), // X9
    (3, &|ctx| {jpf(ctx,Cf,true)}), // XA
    (1, &|ctx| {invalid(ctx)}), // XB
    (3, &|ctx| {callf(ctx,Cf,true)}), // XC
    (1, &|ctx| {invalid(ctx)}), // XD
    (2, &|ctx| {sbc(ctx,IMM)}), // XE
    (4, &|ctx| {rst(ctx,0x18)}), // XF

    // EX
    (3, &|ctx| {ldb(ctx,MEM_FF00_IMM,A)}), // X0
    (3, &|ctx| {pop(ctx,HL)}), // X1
    (2, &|ctx| {ldb(ctx,MEM_FF00_C,A)}), // X2
    (1, &|ctx| {invalid(ctx)}), // X3
    (1, &|ctx| {invalid(ctx)}), // X4
    (4, &|ctx| {push(ctx,HL)}), // X5
    (2, &|ctx| {and(ctx,IMM)}), // X6
    (4, &|ctx| {rst(ctx,0x20)}), // X7

    (1, &|ctx| {fail(ctx)}), // X8

    (1, &|ctx| {jphl(ctx)}), // X9
    (4, &|ctx| {ldb(ctx,MEM(DIMM),A)}), // XA
    (1, &|ctx| {invalid(ctx)}), // XB
    (1, &|ctx| {invalid(ctx)}), // XC
    (1, &|ctx| {invalid(ctx)}), // XD
    (2, &|ctx| {xor(ctx,IMM)}), // XE
    (4, &|ctx| {rst(ctx,0x28)}), // XF

    // FX
    (3, &|ctx| {ldb(ctx,A,MEM_FF00_IMM)}), // X0
    (3, &|ctx| {pop(ctx,AF)}), // X1
    (2, &|ctx| {ldb(ctx,A,MEM_FF00_C)}), // X2
    (1, &|ctx| {di(ctx)}), // X3
    (1, &|ctx| {invalid(ctx)}), // X4
    (4, &|ctx| {push(ctx,AF)}), // X5
    (2, &|ctx| {or(ctx,IMM)}), // X6
    (4, &|ctx| {rst(ctx,0x30)}), // X7

    (1, &|ctx| {fail(ctx)}), // X8

    (2, &|ctx| {ldw(ctx,SP,HL)}), // X9
    (4, &|ctx| {ldb(ctx,A,MEM(DIMM))}), // XA
    (1, &|ctx| {ei(ctx)}), // XB
    (1, &|ctx| {invalid(ctx)}), // XC
    (1, &|ctx| {invalid(ctx)}), // XD
    (2, &|ctx| {cp(ctx,IMM)}), // XE
    (4, &|ctx| {rst(ctx,0x38)}), // XF

];

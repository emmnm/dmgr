use cpu::registers::ByteRegister::*;
use cpu::registers::WordRegister::*;
use cpu::registers::FlagType::*;
use context::Context;
use cpu::z80::*;


pub const CBOPS:[(usize,&'static Fn(&mut Context) -> usize); 256] = [

    // 0X
    (2, &|ctx| {rlc(ctx,B)}), // X0
    (2, &|ctx| {rlc(ctx,C)}), // X1
    (2, &|ctx| {rlc(ctx,D)}), // X2
    (2, &|ctx| {rlc(ctx,E)}), // X3
    (2, &|ctx| {rlc(ctx,H)}), // X4
    (2, &|ctx| {rlc(ctx,L)}), // X5
    (4, &|ctx| {rlc(ctx,MEM(HL))}), // X6
    (2, &|ctx| {rlc(ctx,A)}), // X7
    (2, &|ctx| {rrc(ctx,B)}), // X8
    (2, &|ctx| {rrc(ctx,C)}), // X9
    (2, &|ctx| {rrc(ctx,D)}), // XA
    (2, &|ctx| {rrc(ctx,E)}), // XB
    (2, &|ctx| {rrc(ctx,H)}), // XC
    (2, &|ctx| {rrc(ctx,L)}), // XD
    (4, &|ctx| {rrc(ctx,MEM(HL))}), // XE
    (2, &|ctx| {rrc(ctx,A)}), // XF

    // 1X
    (2, &|ctx| {rl(ctx,B)}), // X0
    (2, &|ctx| {rl(ctx,C)}), // X1
    (2, &|ctx| {rl(ctx,D)}), // X2
    (2, &|ctx| {rl(ctx,E)}), // X3
    (2, &|ctx| {rl(ctx,H)}), // X4
    (2, &|ctx| {rl(ctx,L)}), // X5
    (4, &|ctx| {rl(ctx,MEM(HL))}), // X6
    (2, &|ctx| {rl(ctx,A)}), // X7
    (2, &|ctx| {rr(ctx,B)}), // X8
    (2, &|ctx| {rr(ctx,C)}), // X9
    (2, &|ctx| {rr(ctx,D)}), // XA
    (2, &|ctx| {rr(ctx,E)}), // XB
    (2, &|ctx| {rr(ctx,H)}), // XC
    (2, &|ctx| {rr(ctx,L)}), // XD
    (4, &|ctx| {rr(ctx,MEM(HL))}), // XE
    (2, &|ctx| {rr(ctx,A)}), // XF

    // 2X
    (2, &|ctx| {sla(ctx,B)}), // X0
    (2, &|ctx| {sla(ctx,C)}), // X1
    (2, &|ctx| {sla(ctx,D)}), // X2
    (2, &|ctx| {sla(ctx,E)}), // X3
    (2, &|ctx| {sla(ctx,H)}), // X4
    (2, &|ctx| {sla(ctx,L)}), // X5
    (4, &|ctx| {sla(ctx,MEM(HL))}), // X6
    (2, &|ctx| {sla(ctx,A)}), // X7
    (2, &|ctx| {sra(ctx,B)}), // X8
    (2, &|ctx| {sra(ctx,C)}), // X9
    (2, &|ctx| {sra(ctx,D)}), // XA
    (2, &|ctx| {sra(ctx,E)}), // XB
    (2, &|ctx| {sra(ctx,H)}), // XC
    (2, &|ctx| {sra(ctx,L)}), // XD
    (4, &|ctx| {sra(ctx,MEM(HL))}), // XE
    (2, &|ctx| {sra(ctx,A)}), // XF

    // 3X
    (2, &|ctx| {swap(ctx,B)}), // X0
    (2, &|ctx| {swap(ctx,C)}), // X1
    (2, &|ctx| {swap(ctx,D)}), // X2
    (2, &|ctx| {swap(ctx,E)}), // X3
    (2, &|ctx| {swap(ctx,H)}), // X4
    (2, &|ctx| {swap(ctx,L)}), // X5
    (4, &|ctx| {swap(ctx,MEM(HL))}), // X6
    (2, &|ctx| {swap(ctx,A)}), // X7
    (2, &|ctx| {srl(ctx,B)}), // X8
    (2, &|ctx| {srl(ctx,C)}), // X9
    (2, &|ctx| {srl(ctx,D)}), // XA
    (2, &|ctx| {srl(ctx,E)}), // XB
    (2, &|ctx| {srl(ctx,H)}), // XC
    (2, &|ctx| {srl(ctx,L)}), // XD
    (4, &|ctx| {srl(ctx,MEM(HL))}), // XE
    (2, &|ctx| {srl(ctx,A)}), // XF

    // 4X
    (2, &|ctx| {bit(ctx,0,B)}), // X0
    (2, &|ctx| {bit(ctx,0,C)}), // X1
    (2, &|ctx| {bit(ctx,0,D)}), // X2
    (2, &|ctx| {bit(ctx,0,E)}), // X3
    (2, &|ctx| {bit(ctx,0,H)}), // X4
    (2, &|ctx| {bit(ctx,0,L)}), // X5
    (4, &|ctx| {bit(ctx,0,MEM(HL))}), // X6
    (2, &|ctx| {bit(ctx,0,A)}), // X7
    (2, &|ctx| {bit(ctx,1,B)}), // X8
    (2, &|ctx| {bit(ctx,1,C)}), // X9
    (2, &|ctx| {bit(ctx,1,D)}), // XA
    (2, &|ctx| {bit(ctx,1,E)}), // XB
    (2, &|ctx| {bit(ctx,1,H)}), // XC
    (2, &|ctx| {bit(ctx,1,L)}), // XD
    (4, &|ctx| {bit(ctx,1,MEM(HL))}), // XE
    (2, &|ctx| {bit(ctx,1,A)}), // XF

    // 5X
    (2, &|ctx| {bit(ctx,2,B)}), // X0
    (2, &|ctx| {bit(ctx,2,C)}), // X1
    (2, &|ctx| {bit(ctx,2,D)}), // X2
    (2, &|ctx| {bit(ctx,2,E)}), // X3
    (2, &|ctx| {bit(ctx,2,H)}), // X4
    (2, &|ctx| {bit(ctx,2,L)}), // X5
    (4, &|ctx| {bit(ctx,2,MEM(HL))}), // X6
    (2, &|ctx| {bit(ctx,2,A)}), // X7
    (2, &|ctx| {bit(ctx,3,B)}), // X8
    (2, &|ctx| {bit(ctx,3,C)}), // X9
    (2, &|ctx| {bit(ctx,3,D)}), // XA
    (2, &|ctx| {bit(ctx,3,E)}), // XB
    (2, &|ctx| {bit(ctx,3,H)}), // XC
    (2, &|ctx| {bit(ctx,3,L)}), // XD
    (4, &|ctx| {bit(ctx,3,MEM(HL))}), // XE
    (2, &|ctx| {bit(ctx,3,A)}), // XF

    // 6X
    (2, &|ctx| {bit(ctx,4,B)}), // X0
    (2, &|ctx| {bit(ctx,4,C)}), // X1
    (2, &|ctx| {bit(ctx,4,D)}), // X2
    (2, &|ctx| {bit(ctx,4,E)}), // X3
    (2, &|ctx| {bit(ctx,4,H)}), // X4
    (2, &|ctx| {bit(ctx,4,L)}), // X5
    (4, &|ctx| {bit(ctx,4,MEM(HL))}), // X6
    (2, &|ctx| {bit(ctx,4,A)}), // X7
    (2, &|ctx| {bit(ctx,5,B)}), // X8
    (2, &|ctx| {bit(ctx,5,C)}), // X9
    (2, &|ctx| {bit(ctx,5,D)}), // XA
    (2, &|ctx| {bit(ctx,5,E)}), // XB
    (2, &|ctx| {bit(ctx,5,H)}), // XC
    (2, &|ctx| {bit(ctx,5,L)}), // XD
    (4, &|ctx| {bit(ctx,5,MEM(HL))}), // XE
    (2, &|ctx| {bit(ctx,5,A)}), // XF

    // 7X
    (2, &|ctx| {bit(ctx,6,B)}), // X0
    (2, &|ctx| {bit(ctx,6,C)}), // X1
    (2, &|ctx| {bit(ctx,6,D)}), // X2
    (2, &|ctx| {bit(ctx,6,E)}), // X3
    (2, &|ctx| {bit(ctx,6,H)}), // X4
    (2, &|ctx| {bit(ctx,6,L)}), // X5
    (4, &|ctx| {bit(ctx,6,MEM(HL))}), // X6
    (2, &|ctx| {bit(ctx,6,A)}), // X7
    (2, &|ctx| {bit(ctx,7,B)}), // X8
    (2, &|ctx| {bit(ctx,7,C)}), // X9
    (2, &|ctx| {bit(ctx,7,D)}), // XA
    (2, &|ctx| {bit(ctx,7,E)}), // XB
    (2, &|ctx| {bit(ctx,7,H)}), // XC
    (2, &|ctx| {bit(ctx,7,L)}), // XD
    (4, &|ctx| {bit(ctx,7,MEM(HL))}), // XE
    (2, &|ctx| {bit(ctx,7,A)}), // XF

    // 8X
    (2, &|ctx| {res(ctx,0,B)}), // X0
    (2, &|ctx| {res(ctx,0,C)}), // X1
    (2, &|ctx| {res(ctx,0,D)}), // X2
    (2, &|ctx| {res(ctx,0,E)}), // X3
    (2, &|ctx| {res(ctx,0,H)}), // X4
    (2, &|ctx| {res(ctx,0,L)}), // X5
    (4, &|ctx| {res(ctx,0,MEM(HL))}), // X6
    (2, &|ctx| {res(ctx,0,A)}), // X7
    (2, &|ctx| {res(ctx,1,B)}), // X8
    (2, &|ctx| {res(ctx,1,C)}), // X9
    (2, &|ctx| {res(ctx,1,D)}), // XA
    (2, &|ctx| {res(ctx,1,E)}), // XB
    (2, &|ctx| {res(ctx,1,H)}), // XC
    (2, &|ctx| {res(ctx,1,L)}), // XD
    (4, &|ctx| {res(ctx,1,MEM(HL))}), // XE
    (2, &|ctx| {res(ctx,1,A)}), // XF

    // 9X
    (2, &|ctx| {res(ctx,2,B)}), // X0
    (2, &|ctx| {res(ctx,2,C)}), // X1
    (2, &|ctx| {res(ctx,2,D)}), // X2
    (2, &|ctx| {res(ctx,2,E)}), // X3
    (2, &|ctx| {res(ctx,2,H)}), // X4
    (2, &|ctx| {res(ctx,2,L)}), // X5
    (4, &|ctx| {res(ctx,2,MEM(HL))}), // X6
    (2, &|ctx| {res(ctx,2,A)}), // X7
    (2, &|ctx| {res(ctx,3,B)}), // X8
    (2, &|ctx| {res(ctx,3,C)}), // X9
    (2, &|ctx| {res(ctx,3,D)}), // XA
    (2, &|ctx| {res(ctx,3,E)}), // XB
    (2, &|ctx| {res(ctx,3,H)}), // XC
    (2, &|ctx| {res(ctx,3,L)}), // XD
    (4, &|ctx| {res(ctx,3,MEM(HL))}), // XE
    (2, &|ctx| {res(ctx,3,A)}), // XF

    // AX
    (2, &|ctx| {res(ctx,4,B)}), // X0
    (2, &|ctx| {res(ctx,4,C)}), // X1
    (2, &|ctx| {res(ctx,4,D)}), // X2
    (2, &|ctx| {res(ctx,4,E)}), // X3
    (2, &|ctx| {res(ctx,4,H)}), // X4
    (2, &|ctx| {res(ctx,4,L)}), // X5
    (4, &|ctx| {res(ctx,4,MEM(HL))}), // X6
    (2, &|ctx| {res(ctx,4,A)}), // X7
    (2, &|ctx| {res(ctx,5,B)}), // X8
    (2, &|ctx| {res(ctx,5,C)}), // X9
    (2, &|ctx| {res(ctx,5,D)}), // XA
    (2, &|ctx| {res(ctx,5,E)}), // XB
    (2, &|ctx| {res(ctx,5,H)}), // XC
    (2, &|ctx| {res(ctx,5,L)}), // XD
    (4, &|ctx| {res(ctx,5,MEM(HL))}), // XE
    (2, &|ctx| {res(ctx,5,A)}), // XF

    // BX
    (2, &|ctx| {res(ctx,6,B)}), // X0
    (2, &|ctx| {res(ctx,6,C)}), // X1
    (2, &|ctx| {res(ctx,6,D)}), // X2
    (2, &|ctx| {res(ctx,6,E)}), // X3
    (2, &|ctx| {res(ctx,6,H)}), // X4
    (2, &|ctx| {res(ctx,6,L)}), // X5
    (4, &|ctx| {res(ctx,6,MEM(HL))}), // X6
    (2, &|ctx| {res(ctx,6,A)}), // X7
    (2, &|ctx| {res(ctx,7,B)}), // X8
    (2, &|ctx| {res(ctx,7,C)}), // X9
    (2, &|ctx| {res(ctx,7,D)}), // XA
    (2, &|ctx| {res(ctx,7,E)}), // XB
    (2, &|ctx| {res(ctx,7,H)}), // XC
    (2, &|ctx| {res(ctx,7,L)}), // XD
    (4, &|ctx| {res(ctx,7,MEM(HL))}), // XE
    (2, &|ctx| {res(ctx,7,A)}), // XF

    // CX
    (2, &|ctx| {set(ctx,0,B)}), // X0
    (2, &|ctx| {set(ctx,0,C)}), // X1
    (2, &|ctx| {set(ctx,0,D)}), // X2
    (2, &|ctx| {set(ctx,0,E)}), // X3
    (2, &|ctx| {set(ctx,0,H)}), // X4
    (2, &|ctx| {set(ctx,0,L)}), // X5
    (4, &|ctx| {set(ctx,0,MEM(HL))}), // X6
    (2, &|ctx| {set(ctx,0,A)}), // X7
    (2, &|ctx| {set(ctx,1,B)}), // X8
    (2, &|ctx| {set(ctx,1,C)}), // X9
    (2, &|ctx| {set(ctx,1,D)}), // XA
    (2, &|ctx| {set(ctx,1,E)}), // XB
    (2, &|ctx| {set(ctx,1,H)}), // XC
    (2, &|ctx| {set(ctx,1,L)}), // XD
    (4, &|ctx| {set(ctx,1,MEM(HL))}), // XE
    (2, &|ctx| {set(ctx,1,A)}), // XF

    // DX
    (2, &|ctx| {set(ctx,2,B)}), // X0
    (2, &|ctx| {set(ctx,2,C)}), // X1
    (2, &|ctx| {set(ctx,2,D)}), // X2
    (2, &|ctx| {set(ctx,2,E)}), // X3
    (2, &|ctx| {set(ctx,2,H)}), // X4
    (2, &|ctx| {set(ctx,2,L)}), // X5
    (4, &|ctx| {set(ctx,2,MEM(HL))}), // X6
    (2, &|ctx| {set(ctx,2,A)}), // X7
    (2, &|ctx| {set(ctx,3,B)}), // X8
    (2, &|ctx| {set(ctx,3,C)}), // X9
    (2, &|ctx| {set(ctx,3,D)}), // XA
    (2, &|ctx| {set(ctx,3,E)}), // XB
    (2, &|ctx| {set(ctx,3,H)}), // XC
    (2, &|ctx| {set(ctx,3,L)}), // XD
    (4, &|ctx| {set(ctx,3,MEM(HL))}), // XE
    (2, &|ctx| {set(ctx,3,A)}), // XF

    // EX
    (2, &|ctx| {set(ctx,4,B)}), // X0
    (2, &|ctx| {set(ctx,4,C)}), // X1
    (2, &|ctx| {set(ctx,4,D)}), // X2
    (2, &|ctx| {set(ctx,4,E)}), // X3
    (2, &|ctx| {set(ctx,4,H)}), // X4
    (2, &|ctx| {set(ctx,4,L)}), // X5
    (4, &|ctx| {set(ctx,4,MEM(HL))}), // X6
    (2, &|ctx| {set(ctx,4,A)}), // X7
    (2, &|ctx| {set(ctx,5,B)}), // X8
    (2, &|ctx| {set(ctx,5,C)}), // X9
    (2, &|ctx| {set(ctx,5,D)}), // XA
    (2, &|ctx| {set(ctx,5,E)}), // XB
    (2, &|ctx| {set(ctx,5,H)}), // XC
    (2, &|ctx| {set(ctx,5,L)}), // XD
    (4, &|ctx| {set(ctx,5,MEM(HL))}), // XE
    (2, &|ctx| {set(ctx,5,A)}), // XF

    // FX
    (2, &|ctx| {set(ctx,6,B)}), // X0
    (2, &|ctx| {set(ctx,6,C)}), // X1
    (2, &|ctx| {set(ctx,6,D)}), // X2
    (2, &|ctx| {set(ctx,6,E)}), // X3
    (2, &|ctx| {set(ctx,6,H)}), // X4
    (2, &|ctx| {set(ctx,6,L)}), // X5
    (4, &|ctx| {set(ctx,6,MEM(HL))}), // X6
    (2, &|ctx| {set(ctx,6,A)}), // X7
    (2, &|ctx| {set(ctx,7,B)}), // X8
    (2, &|ctx| {set(ctx,7,C)}), // X9
    (2, &|ctx| {set(ctx,7,D)}), // XA
    (2, &|ctx| {set(ctx,7,E)}), // XB
    (2, &|ctx| {set(ctx,7,H)}), // XC
    (2, &|ctx| {set(ctx,7,L)}), // XD
    (4, &|ctx| {set(ctx,7,MEM(HL))}), // XE
    (2, &|ctx| {set(ctx,7,A)}), // XF

];

use cpu::registers::ByteRegister::*;
use cpu::registers::WordRegister::*;
use cpu::registers::FlagType::*;
use context::Context;
use cpu::z80::*;


pub const CBOPS:[(usize,&'static Fn(&mut Context) -> usize); 256] = [

    // 0X
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // 1X
    (2, &|ctx| {rl(ctx,B)}), // X0
    (2, &|ctx| {rl(ctx,C)}), // X1
    (2, &|ctx| {rl(ctx,D)}), // X2
    (2, &|ctx| {rl(ctx,E)}), // X3
    (2, &|ctx| {rl(ctx,H)}), // X4
    (2, &|ctx| {rl(ctx,L)}), // X5
    (4, &|ctx| {rl(ctx,MEM(HL))}), // X6
    (2, &|ctx| {rl(ctx,A)}), // X7

    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // 2X
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // 3X
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // 4X
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // 5X
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // 6X
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // 7X
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7

    (2, &|ctx| {bit(ctx,7,B)}), // X8
    (2, &|ctx| {bit(ctx,7,C)}), // X9
    (2, &|ctx| {bit(ctx,7,D)}), // XA
    (2, &|ctx| {bit(ctx,7,E)}), // XB
    (2, &|ctx| {bit(ctx,7,H)}), // XC
    (2, &|ctx| {bit(ctx,7,L)}), // XD
    (4, &|ctx| {bit(ctx,7,MEM(HL))}), // XE
    (2, &|ctx| {bit(ctx,7,A)}), // XF

    // 8X
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // 9X
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // AX
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // BX
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // CX
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // DX
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // EX
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

    // FX
    (1, &|ctx| {fail(ctx)}), // X0
    (1, &|ctx| {fail(ctx)}), // X1
    (1, &|ctx| {fail(ctx)}), // X2
    (1, &|ctx| {fail(ctx)}), // X3
    (1, &|ctx| {fail(ctx)}), // X4
    (1, &|ctx| {fail(ctx)}), // X5
    (1, &|ctx| {fail(ctx)}), // X6
    (1, &|ctx| {fail(ctx)}), // X7
    (1, &|ctx| {fail(ctx)}), // X8
    (1, &|ctx| {fail(ctx)}), // X9
    (1, &|ctx| {fail(ctx)}), // XA
    (1, &|ctx| {fail(ctx)}), // XB
    (1, &|ctx| {fail(ctx)}), // XC
    (1, &|ctx| {fail(ctx)}), // XD
    (1, &|ctx| {fail(ctx)}), // XE
    (1, &|ctx| {fail(ctx)}), // XF

];

/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive.
 *
 * Copyright (C) 1985 MIPS Computer Systems, Inc.
 * Copyright (C) 1994, 95, 99, 2003 by Ralf Baechle
 * Copyright (C) 1990 - 1992, 1999 Silicon Graphics, Inc.
 * Copyright (C) 2011 Wind River Systems, written by Ralf Baechle
 */

// The original header includes <asm/sgidefs.h>; ABI selection is represented
// here by the `mips_abi32`, `mips_abi64`, and `mips_nabi32` configuration flags.

#[cfg(feature = "mips_abi32")]
pub mod abi32 {
    pub const GPR_ZERO: u8 = 0;  // wired zero
    pub const GPR_AT: u8 = 1;    // assembler temp
    pub const GPR_V0: u8 = 2;    // return value
    pub const GPR_V1: u8 = 3;
    pub const GPR_A0: u8 = 4;    // argument registers
    pub const GPR_A1: u8 = 5;
    pub const GPR_A2: u8 = 6;
    pub const GPR_A3: u8 = 7;
    pub const GPR_T0: u8 = 8;    // caller saved
    pub const GPR_T1: u8 = 9;
    pub const GPR_T2: u8 = 10;
    pub const GPR_T3: u8 = 11;
    pub const GPR_T4: u8 = 12;
    pub const GPR_TA0: u8 = 12;
    pub const GPR_T5: u8 = 13;
    pub const GPR_TA1: u8 = 13;
    pub const GPR_T6: u8 = 14;
    pub const GPR_TA2: u8 = 14;
    pub const GPR_T7: u8 = 15;
    pub const GPR_TA3: u8 = 15;
    pub const GPR_S0: u8 = 16;   // callee saved
    pub const GPR_S1: u8 = 17;
    pub const GPR_S2: u8 = 18;
    pub const GPR_S3: u8 = 19;
    pub const GPR_S4: u8 = 20;
    pub const GPR_S5: u8 = 21;
    pub const GPR_S6: u8 = 22;
    pub const GPR_S7: u8 = 23;
    pub const GPR_T8: u8 = 24;   // caller saved
    pub const GPR_T9: u8 = 25;
    pub const GPR_JP: u8 = 25;   // PIC jump register
    pub const GPR_K0: u8 = 26;   // kernel scratch
    pub const GPR_K1: u8 = 27;
    pub const GPR_GP: u8 = 28;   // global pointer
    pub const GPR_SP: u8 = 29;   // stack pointer
    pub const GPR_FP: u8 = 30;   // frame pointer
    pub const GPR_S8: u8 = 30;   // same like fp!
    pub const GPR_RA: u8 = 31;   // return address

    // Symbolic assembler register names (the `$` syntax is assembler-only).
    pub const zero: u8 = 0; pub const AT: u8 = 1; pub const v0: u8 = 2;
    pub const v1: u8 = 3; pub const a0: u8 = 4; pub const a1: u8 = 5;
    pub const a2: u8 = 6; pub const a3: u8 = 7; pub const t0: u8 = 8;
    pub const t1: u8 = 9; pub const t2: u8 = 10; pub const t3: u8 = 11;
    pub const t4: u8 = 12; pub const ta0: u8 = 12; pub const t5: u8 = 13;
    pub const ta1: u8 = 13; pub const t6: u8 = 14; pub const ta2: u8 = 14;
    pub const t7: u8 = 15; pub const ta3: u8 = 15; pub const s0: u8 = 16;
    pub const s1: u8 = 17; pub const s2: u8 = 18; pub const s3: u8 = 19;
    pub const s4: u8 = 20; pub const s5: u8 = 21; pub const s6: u8 = 22;
    pub const s7: u8 = 23; pub const t8: u8 = 24; pub const t9: u8 = 25;
    pub const jp: u8 = 25; pub const k0: u8 = 26; pub const k1: u8 = 27;
    pub const gp: u8 = 28; pub const sp: u8 = 29; pub const fp: u8 = 30;
    pub const s8: u8 = 30; pub const ra: u8 = 31;
}

#[cfg(any(feature = "mips_abi64", feature = "mips_nabi32"))]
pub mod abi64 {
    pub const GPR_ZERO: u8=0; pub const GPR_AT: u8=1; pub const GPR_V0: u8=2; pub const GPR_V1: u8=3;
    pub const GPR_A0: u8=4; pub const GPR_A1: u8=5; pub const GPR_A2: u8=6; pub const GPR_A3: u8=7;
    pub const GPR_A4: u8=8; pub const GPR_TA0: u8=8; pub const GPR_A5: u8=9; pub const GPR_TA1: u8=9;
    pub const GPR_A6: u8=10; pub const GPR_TA2: u8=10; pub const GPR_A7: u8=11; pub const GPR_TA3: u8=11;
    pub const GPR_T0: u8=12; pub const GPR_T1: u8=13; pub const GPR_T2: u8=14; pub const GPR_T3: u8=15;
    pub const GPR_S0: u8=16; pub const GPR_S1: u8=17; pub const GPR_S2: u8=18; pub const GPR_S3: u8=19;
    pub const GPR_S4: u8=20; pub const GPR_S5: u8=21; pub const GPR_S6: u8=22; pub const GPR_S7: u8=23;
    pub const GPR_T8: u8=24; pub const GPR_T9: u8=25; pub const GPR_JP: u8=25; pub const GPR_K0: u8=26;
    pub const GPR_K1: u8=27; pub const GPR_GP: u8=28; pub const GPR_SP: u8=29; pub const GPR_FP: u8=30;
    pub const GPR_S8: u8=30; pub const GPR_RA: u8=31;

    // Symbolic assembler register names (the `$` syntax is assembler-only).
    pub const zero: u8=0; pub const AT: u8=1; pub const v0: u8=2; pub const v1: u8=3;
    pub const a0: u8=4; pub const a1: u8=5; pub const a2: u8=6; pub const a3: u8=7;
    pub const a4: u8=8; pub const ta0: u8=8; pub const a5: u8=9; pub const ta1: u8=9;
    pub const a6: u8=10; pub const ta2: u8=10; pub const a7: u8=11; pub const ta3: u8=11;
    pub const t0: u8=12; pub const t1: u8=13; pub const t2: u8=14; pub const t3: u8=15;
    pub const s0: u8=16; pub const s1: u8=17; pub const s2: u8=18; pub const s3: u8=19;
    pub const s4: u8=20; pub const s5: u8=21; pub const s6: u8=22; pub const s7: u8=23;
    pub const t8: u8=24; pub const t9: u8=25; pub const jp: u8=25; pub const k0: u8=26;
    pub const k1: u8=27; pub const gp: u8=28; pub const sp: u8=29; pub const fp: u8=30;
    pub const s8: u8=30; pub const ra: u8=31;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

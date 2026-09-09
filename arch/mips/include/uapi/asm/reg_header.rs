/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Various register offset definitions for debuggers, core file
 * examiners and whatnot.
 *
 * Copyright (C) 1995, 1999 Ralf Baechle
 * Copyright (C) 1995, 1999 Silicon Graphics
 */

pub const MIPS32_EF_R0: i32 = 6;
pub const MIPS32_EF_R1: i32 = 7;
pub const MIPS32_EF_R2: i32 = 8;
pub const MIPS32_EF_R3: i32 = 9;
pub const MIPS32_EF_R4: i32 = 10;
pub const MIPS32_EF_R5: i32 = 11;
pub const MIPS32_EF_R6: i32 = 12;
pub const MIPS32_EF_R7: i32 = 13;
pub const MIPS32_EF_R8: i32 = 14;
pub const MIPS32_EF_R9: i32 = 15;
pub const MIPS32_EF_R10: i32 = 16;
pub const MIPS32_EF_R11: i32 = 17;
pub const MIPS32_EF_R12: i32 = 18;
pub const MIPS32_EF_R13: i32 = 19;
pub const MIPS32_EF_R14: i32 = 20;
pub const MIPS32_EF_R15: i32 = 21;
pub const MIPS32_EF_R16: i32 = 22;
pub const MIPS32_EF_R17: i32 = 23;
pub const MIPS32_EF_R18: i32 = 24;
pub const MIPS32_EF_R19: i32 = 25;
pub const MIPS32_EF_R20: i32 = 26;
pub const MIPS32_EF_R21: i32 = 27;
pub const MIPS32_EF_R22: i32 = 28;
pub const MIPS32_EF_R23: i32 = 29;
pub const MIPS32_EF_R24: i32 = 30;
pub const MIPS32_EF_R25: i32 = 31;
/* k0/k1 unsaved */
pub const MIPS32_EF_R26: i32 = 32;
pub const MIPS32_EF_R27: i32 = 33;
pub const MIPS32_EF_R28: i32 = 34;
pub const MIPS32_EF_R29: i32 = 35;
pub const MIPS32_EF_R30: i32 = 36;
pub const MIPS32_EF_R31: i32 = 37;
/* Saved special registers */
pub const MIPS32_EF_LO: i32 = 38;
pub const MIPS32_EF_HI: i32 = 39;
pub const MIPS32_EF_CP0_EPC: i32 = 40;
pub const MIPS32_EF_CP0_BADVADDR: i32 = 41;
pub const MIPS32_EF_CP0_STATUS: i32 = 42;
pub const MIPS32_EF_CP0_CAUSE: i32 = 43;
pub const MIPS32_EF_UNUSED0: i32 = 44;
pub const MIPS32_EF_SIZE: i32 = 180;

pub const MIPS64_EF_R0: i32 = 0;
pub const MIPS64_EF_R1: i32 = 1;
pub const MIPS64_EF_R2: i32 = 2;
pub const MIPS64_EF_R3: i32 = 3;
pub const MIPS64_EF_R4: i32 = 4;
pub const MIPS64_EF_R5: i32 = 5;
pub const MIPS64_EF_R6: i32 = 6;
pub const MIPS64_EF_R7: i32 = 7;
pub const MIPS64_EF_R8: i32 = 8;
pub const MIPS64_EF_R9: i32 = 9;
pub const MIPS64_EF_R10: i32 = 10;
pub const MIPS64_EF_R11: i32 = 11;
pub const MIPS64_EF_R12: i32 = 12;
pub const MIPS64_EF_R13: i32 = 13;
pub const MIPS64_EF_R14: i32 = 14;
pub const MIPS64_EF_R15: i32 = 15;
pub const MIPS64_EF_R16: i32 = 16;
pub const MIPS64_EF_R17: i32 = 17;
pub const MIPS64_EF_R18: i32 = 18;
pub const MIPS64_EF_R19: i32 = 19;
pub const MIPS64_EF_R20: i32 = 20;
pub const MIPS64_EF_R21: i32 = 21;
pub const MIPS64_EF_R22: i32 = 22;
pub const MIPS64_EF_R23: i32 = 23;
pub const MIPS64_EF_R24: i32 = 24;
pub const MIPS64_EF_R25: i32 = 25;
/* k0/k1 unsaved */
pub const MIPS64_EF_R26: i32 = 26;
pub const MIPS64_EF_R27: i32 = 27;
pub const MIPS64_EF_R28: i32 = 28;
pub const MIPS64_EF_R29: i32 = 29;
pub const MIPS64_EF_R30: i32 = 30;
pub const MIPS64_EF_R31: i32 = 31;
/* Saved special registers */
pub const MIPS64_EF_LO: i32 = 32;
pub const MIPS64_EF_HI: i32 = 33;
pub const MIPS64_EF_CP0_EPC: i32 = 34;
pub const MIPS64_EF_CP0_BADVADDR: i32 = 35;
pub const MIPS64_EF_CP0_STATUS: i32 = 36;
pub const MIPS64_EF_CP0_CAUSE: i32 = 37;
pub const MIPS64_EF_SIZE: i32 = 304; /* size in bytes */

/* The following aliases are selected by the C preprocessor's MIPS ABI test. */
#[cfg(mips_abi32)]
pub use self::mips32_aliases::*;
#[cfg(mips_abi32)]
mod mips32_aliases {
    pub const EF_R0: i32 = super::MIPS32_EF_R0; pub const EF_R1: i32 = super::MIPS32_EF_R1;
    pub const EF_R2: i32 = super::MIPS32_EF_R2; pub const EF_R3: i32 = super::MIPS32_EF_R3;
    pub const EF_R4: i32 = super::MIPS32_EF_R4; pub const EF_R5: i32 = super::MIPS32_EF_R5;
    pub const EF_R6: i32 = super::MIPS32_EF_R6; pub const EF_R7: i32 = super::MIPS32_EF_R7;
    pub const EF_R8: i32 = super::MIPS32_EF_R8; pub const EF_R9: i32 = super::MIPS32_EF_R9;
    pub const EF_R10: i32 = super::MIPS32_EF_R10; pub const EF_R11: i32 = super::MIPS32_EF_R11;
    pub const EF_R12: i32 = super::MIPS32_EF_R12; pub const EF_R13: i32 = super::MIPS32_EF_R13;
    pub const EF_R14: i32 = super::MIPS32_EF_R14; pub const EF_R15: i32 = super::MIPS32_EF_R15;
    pub const EF_R16: i32 = super::MIPS32_EF_R16; pub const EF_R17: i32 = super::MIPS32_EF_R17;
    pub const EF_R18: i32 = super::MIPS32_EF_R18; pub const EF_R19: i32 = super::MIPS32_EF_R19;
    pub const EF_R20: i32 = super::MIPS32_EF_R20; pub const EF_R21: i32 = super::MIPS32_EF_R21;
    pub const EF_R22: i32 = super::MIPS32_EF_R22; pub const EF_R23: i32 = super::MIPS32_EF_R23;
    pub const EF_R24: i32 = super::MIPS32_EF_R24; pub const EF_R25: i32 = super::MIPS32_EF_R25;
    pub const EF_R26: i32 = super::MIPS32_EF_R26; pub const EF_R27: i32 = super::MIPS32_EF_R27;
    pub const EF_R28: i32 = super::MIPS32_EF_R28; pub const EF_R29: i32 = super::MIPS32_EF_R29;
    pub const EF_R30: i32 = super::MIPS32_EF_R30; pub const EF_R31: i32 = super::MIPS32_EF_R31;
    pub const EF_LO: i32 = super::MIPS32_EF_LO; pub const EF_HI: i32 = super::MIPS32_EF_HI;
    pub const EF_CP0_EPC: i32 = super::MIPS32_EF_CP0_EPC; pub const EF_CP0_BADVADDR: i32 = super::MIPS32_EF_CP0_BADVADDR;
    pub const EF_CP0_STATUS: i32 = super::MIPS32_EF_CP0_STATUS; pub const EF_CP0_CAUSE: i32 = super::MIPS32_EF_CP0_CAUSE;
    pub const EF_UNUSED0: i32 = super::MIPS32_EF_UNUSED0; pub const EF_SIZE: i32 = super::MIPS32_EF_SIZE;
}

/* _MIPS_SIM == _MIPS_SIM_ABI64 || _MIPS_SIM == _MIPS_SIM_NABI32 */
#[cfg(any(mips_abi64, mips_nabi32))]
pub use self::mips64_aliases::*;
#[cfg(any(mips_abi64, mips_nabi32))]
mod mips64_aliases {
    pub const EF_R0: i32 = super::MIPS64_EF_R0; pub const EF_R1: i32 = super::MIPS64_EF_R1;
    pub const EF_R2: i32 = super::MIPS64_EF_R2; pub const EF_R3: i32 = super::MIPS64_EF_R3;
    pub const EF_R4: i32 = super::MIPS64_EF_R4; pub const EF_R5: i32 = super::MIPS64_EF_R5;
    pub const EF_R6: i32 = super::MIPS64_EF_R6; pub const EF_R7: i32 = super::MIPS64_EF_R7;
    pub const EF_R8: i32 = super::MIPS64_EF_R8; pub const EF_R9: i32 = super::MIPS64_EF_R9;
    pub const EF_R10: i32 = super::MIPS64_EF_R10; pub const EF_R11: i32 = super::MIPS64_EF_R11;
    pub const EF_R12: i32 = super::MIPS64_EF_R12; pub const EF_R13: i32 = super::MIPS64_EF_R13;
    pub const EF_R14: i32 = super::MIPS64_EF_R14; pub const EF_R15: i32 = super::MIPS64_EF_R15;
    pub const EF_R16: i32 = super::MIPS64_EF_R16; pub const EF_R17: i32 = super::MIPS64_EF_R17;
    pub const EF_R18: i32 = super::MIPS64_EF_R18; pub const EF_R19: i32 = super::MIPS64_EF_R19;
    pub const EF_R20: i32 = super::MIPS64_EF_R20; pub const EF_R21: i32 = super::MIPS64_EF_R21;
    pub const EF_R22: i32 = super::MIPS64_EF_R22; pub const EF_R23: i32 = super::MIPS64_EF_R23;
    pub const EF_R24: i32 = super::MIPS64_EF_R24; pub const EF_R25: i32 = super::MIPS64_EF_R25;
    pub const EF_R26: i32 = super::MIPS64_EF_R26; pub const EF_R27: i32 = super::MIPS64_EF_R27;
    pub const EF_R28: i32 = super::MIPS64_EF_R28; pub const EF_R29: i32 = super::MIPS64_EF_R29;
    pub const EF_R30: i32 = super::MIPS64_EF_R30; pub const EF_R31: i32 = super::MIPS64_EF_R31;
    pub const EF_LO: i32 = super::MIPS64_EF_LO; pub const EF_HI: i32 = super::MIPS64_EF_HI;
    pub const EF_CP0_EPC: i32 = super::MIPS64_EF_CP0_EPC; pub const EF_CP0_BADVADDR: i32 = super::MIPS64_EF_CP0_BADVADDR;
    pub const EF_CP0_STATUS: i32 = super::MIPS64_EF_CP0_STATUS; pub const EF_CP0_CAUSE: i32 = super::MIPS64_EF_CP0_CAUSE;
    pub const EF_SIZE: i32 = super::MIPS64_EF_SIZE;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

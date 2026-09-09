/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Various register offset definitions for debuggers, core file
 * examiners and whatnot.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

pub const LOONGARCH_EF_R0: i32 = 0;
pub const LOONGARCH_EF_R1: i32 = 1;
pub const LOONGARCH_EF_R2: i32 = 2;
pub const LOONGARCH_EF_R3: i32 = 3;
pub const LOONGARCH_EF_R4: i32 = 4;
pub const LOONGARCH_EF_R5: i32 = 5;
pub const LOONGARCH_EF_R6: i32 = 6;
pub const LOONGARCH_EF_R7: i32 = 7;
pub const LOONGARCH_EF_R8: i32 = 8;
pub const LOONGARCH_EF_R9: i32 = 9;
pub const LOONGARCH_EF_R10: i32 = 10;
pub const LOONGARCH_EF_R11: i32 = 11;
pub const LOONGARCH_EF_R12: i32 = 12;
pub const LOONGARCH_EF_R13: i32 = 13;
pub const LOONGARCH_EF_R14: i32 = 14;
pub const LOONGARCH_EF_R15: i32 = 15;
pub const LOONGARCH_EF_R16: i32 = 16;
pub const LOONGARCH_EF_R17: i32 = 17;
pub const LOONGARCH_EF_R18: i32 = 18;
pub const LOONGARCH_EF_R19: i32 = 19;
pub const LOONGARCH_EF_R20: i32 = 20;
pub const LOONGARCH_EF_R21: i32 = 21;
pub const LOONGARCH_EF_R22: i32 = 22;
pub const LOONGARCH_EF_R23: i32 = 23;
pub const LOONGARCH_EF_R24: i32 = 24;
pub const LOONGARCH_EF_R25: i32 = 25;
pub const LOONGARCH_EF_R26: i32 = 26;
pub const LOONGARCH_EF_R27: i32 = 27;
pub const LOONGARCH_EF_R28: i32 = 28;
pub const LOONGARCH_EF_R29: i32 = 29;
pub const LOONGARCH_EF_R30: i32 = 30;
pub const LOONGARCH_EF_R31: i32 = 31;

/*
 * Saved special registers
 */
pub const LOONGARCH_EF_ORIG_A0: i32 = 32;
pub const LOONGARCH_EF_CSR_ERA: i32 = 33;
pub const LOONGARCH_EF_CSR_BADV: i32 = 34;
pub const LOONGARCH_EF_CSR_CRMD: i32 = 35;
pub const LOONGARCH_EF_CSR_PRMD: i32 = 36;
pub const LOONGARCH_EF_CSR_EUEN: i32 = 37;
pub const LOONGARCH_EF_CSR_ECFG: i32 = 38;
pub const LOONGARCH_EF_CSR_ESTAT: i32 = 39;

pub const LOONGARCH_EF_SIZE: i32 = 320; /* size in bytes */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

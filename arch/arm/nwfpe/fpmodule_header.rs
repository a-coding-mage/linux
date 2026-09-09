/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    NetWinder Floating Point Emulator
    (c) Rebel.com, 1998-1999

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>


    27/03/03 Ian Molton Clean up CONFIG_CPU
*/

// Translated from the C header guard __FPMODULE_H__.

pub const REG_ORIG_R0: i32 = 17;
pub const REG_CPSR: i32 = 16;
pub const REG_PC: i32 = 15;
pub const REG_LR: i32 = 14;
pub const REG_SP: i32 = 13;
pub const REG_IP: i32 = 12;
pub const REG_FP: i32 = 11;
pub const REG_R10: i32 = 10;
pub const REG_R9: i32 = 9;
// The source redundantly defines REG_R9 with the same value a second time.
pub const REG_R8: i32 = 8;
pub const REG_R7: i32 = 7;
pub const REG_R6: i32 = 6;
pub const REG_R5: i32 = 5;
pub const REG_R4: i32 = 4;
pub const REG_R3: i32 = 3;
pub const REG_R2: i32 = 2;
pub const REG_R1: i32 = 1;
pub const REG_R0: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

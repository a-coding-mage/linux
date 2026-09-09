/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Alpha register definitions.  The C preprocessor aliases are represented as
// register-number constants for use by low-level Rust code.

pub const V0: u32 = 0; // function return value

pub const T0: u32 = 1; // temporary registers (caller-saved)
pub const T1: u32 = 2;
pub const T2: u32 = 3;
pub const T3: u32 = 4;
pub const T4: u32 = 5;
pub const T5: u32 = 6;
pub const T6: u32 = 7;
pub const T7: u32 = 8;

pub const S0: u32 = 9; // saved-registers (callee-saved registers)
pub const S1: u32 = 10;
pub const S2: u32 = 11;
pub const S3: u32 = 12;
pub const S4: u32 = 13;
pub const S5: u32 = 14;
pub const S6: u32 = 15;
pub const FP: u32 = S6; // frame-pointer (s6 in frame-less procedures)

pub const A0: u32 = 16; // argument registers (caller-saved)
pub const A1: u32 = 17;
pub const A2: u32 = 18;
pub const A3: u32 = 19;
pub const A4: u32 = 20;
pub const A5: u32 = 21;

pub const T8: u32 = 22; // more temps (caller-saved)
pub const T9: u32 = 23;
pub const T10: u32 = 24;
pub const T11: u32 = 25;
pub const RA: u32 = 26; // return address register
pub const T12: u32 = 27;

pub const PV: u32 = T12; // procedure-variable register
// $at is the Alpha assembler temporary register (register 28).
pub const AT: u32 = 28;
pub const GP: u32 = 29; // global pointer
pub const SP: u32 = 30; // stack pointer
pub const ZERO: u32 = 31; // reads as zero, writes are noops

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

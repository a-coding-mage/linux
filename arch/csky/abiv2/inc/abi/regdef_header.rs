/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard is omitted in Rust; items are naturally defined once per module.

// In assembler builds, `syscallid` expands to the register name `r7`.
// In C builds, it expands to the string literal "r7".
pub const syscallid: &str = "r7";

macro_rules! regs_syscallid {
    ($regs:expr) => {
        $regs.regs[3]
    };
}

macro_rules! regs_fp {
    ($regs:expr) => {
        $regs.regs[4]
    };
}

/*
 * PSR format:
 * | 31 | 30-24 | 23-16 | 15 14 | 13-10 | 9 | 8-0 |
 *   S              VEC     TM            MM
 *
 *   S: Super Mode
 * VEC: Exception Number
 *  TM: Trace Mode
 *  MM: Memory unaligned addr access
 */
pub const DEFAULT_PSR_VALUE: u32 = 0x80000200;

pub const SYSTRACE_SAVENUM: u32 = 5;

pub const TRAP0_SIZE: u32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

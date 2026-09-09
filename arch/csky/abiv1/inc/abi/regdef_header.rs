/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_CSKY_REGDEF_H

// In assembler builds, syscallid expands to the register name r1.
// In C builds, syscallid expands to the string "r1".
#[macro_export]
macro_rules! syscallid {
    () => { "r1" };
}

#[macro_export]
macro_rules! regs_syscallid {
    ($regs:expr) => { $regs.regs[9] };
}

#[macro_export]
macro_rules! regs_fp {
    ($regs:expr) => { $regs.regs[2] };
}

/*
 * PSR format:
 * | 31 | 30-24 | 23-16 | 15 14 | 13-0 |
 *   S     CPID     VEC     TM
 *
 *    S: Super Mode
 * CPID: Coprocessor id, only 15 for MMU
 *  VEC: Exception Number
 *   TM: Trace Mode
 */
pub const DEFAULT_PSR_VALUE: u32 = 0x8f000000;

pub const SYSTRACE_SAVENUM: usize = 2;

pub const TRAP0_SIZE: usize = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const PT_D1: i32 = 0;
pub const PT_D2: i32 = 1;
pub const PT_D3: i32 = 2;
pub const PT_D4: i32 = 3;
pub const PT_D5: i32 = 4;
pub const PT_D6: i32 = 5;
pub const PT_D7: i32 = 6;
pub const PT_A0: i32 = 7;
pub const PT_A1: i32 = 8;
pub const PT_A2: i32 = 9;
pub const PT_A3: i32 = 10;
pub const PT_A4: i32 = 11;
pub const PT_A5: i32 = 12;
pub const PT_A6: i32 = 13;
pub const PT_D0: i32 = 14;
pub const PT_USP: i32 = 15;
pub const PT_ORIG_D0: i32 = 16;
pub const PT_SR: i32 = 17;
pub const PT_PC: i32 = 18;

/* this struct defines the way the registers are stored on the
   stack during a system call. */
#[repr(C)]
pub struct pt_regs {
    pub d1: isize,
    pub d2: isize,
    pub d3: isize,
    pub d4: isize,
    pub d5: isize,
    pub a0: isize,
    pub a1: isize,
    pub a2: isize,
    pub d0: isize,
    pub orig_d0: isize,
    pub stkadj: isize,
    /* C bit-fields are represented by their 16-bit allocation unit. */
    #[cfg(feature = "mcoldfire")]
    pub format_vector: u16,
    #[cfg(feature = "mcoldfire")]
    pub sr: u16,
    #[cfg(feature = "mcoldfire")]
    pub pc: u32,
    #[cfg(not(feature = "mcoldfire"))]
    pub sr: u16,
    #[cfg(not(feature = "mcoldfire"))]
    pub pc: u32,
    #[cfg(not(feature = "mcoldfire"))]
    pub format_vector: u16,
}

/*
 * This is the extended stack used by signal handlers and the context
 * switcher: it's pushed after the normal "struct pt_regs".
 */
#[repr(C)]
pub struct switch_stack {
    pub d6: u32,
    pub d7: u32,
    pub a3: u32,
    pub a4: u32,
    pub a5: u32,
    pub a6: u32,
    pub retpc: u32,
}

/* Arbitrarily choose the same ptrace numbers as used by the Sparc code. */
pub const PTRACE_GETREGS: i32 = 12;
pub const PTRACE_SETREGS: i32 = 13;
pub const PTRACE_GETFPREGS: i32 = 14;
pub const PTRACE_SETFPREGS: i32 = 15;

pub const PTRACE_GET_THREAD_AREA: i32 = 25;

pub const PTRACE_GETFDPIC: i32 = 31;

pub const PTRACE_SINGLEBLOCK: i32 = 33; /* resume execution until next branch */

pub const PTRACE_GETFDPIC_EXEC: i32 = 0;
pub const PTRACE_GETFDPIC_INTERP: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

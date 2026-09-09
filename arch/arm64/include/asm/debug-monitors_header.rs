/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// C header dependencies are supplied by other translated files.

/* Low-level stepping controls. */
pub const DBG_SPSR_SS: u32 = 1u32 << 21;

#[inline]
pub const fn DBG_ESR_EVT(x: u64) -> u64 {
    (x >> 27) & 0x7
}

/* AArch64 */
pub const DBG_ESR_EVT_HWBP: u32 = 0x0;
pub const DBG_ESR_EVT_HWSS: u32 = 0x1;
pub const DBG_ESR_EVT_HWWP: u32 = 0x2;
pub const DBG_ESR_EVT_BRK: u32 = 0x6;

/*
 * Break point instruction encoding
 */
pub const BREAK_INSTR_SIZE: usize = AARCH64_INSN_SIZE;

pub const AARCH64_BREAK_KGDB_DYN_DBG: u32 =
    AARCH64_BREAK_MON | (KGDB_DYN_DBG_BRK_IMM << 5);

pub const CACHE_FLUSH_IS_SAFE: i32 = 1;

/* kprobes BRK opcodes with ESR encoding  */
pub const BRK64_OPCODE_KPROBES: u32 = AARCH64_BREAK_MON | (KPROBES_BRK_IMM << 5);
pub const BRK64_OPCODE_KPROBES_SS: u32 = AARCH64_BREAK_MON | (KPROBES_BRK_SS_IMM << 5);
/* uprobes BRK opcodes with ESR encoding  */
pub const BRK64_OPCODE_UPROBES: u32 = AARCH64_BREAK_MON | (UPROBES_BRK_IMM << 5);

/* AArch32 */
pub const DBG_ESR_EVT_BKPT: u32 = 0x4;
pub const DBG_ESR_EVT_VECC: u32 = 0x5;

pub const AARCH32_BREAK_ARM: u32 = 0x07f001f0;
pub const AARCH32_BREAK_THUMB: u16 = 0xde01;
pub const AARCH32_BREAK_THUMB2_LO: u16 = 0xf7f0;
pub const AARCH32_BREAK_THUMB2_HI: u16 = 0xa000;

// The following declarations are excluded when compiling as an assembler in C.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

pub const DBG_ARCH_ID_RESERVED: i32 = 0; /* In case of ptrace ABI updates. */

pub const DBG_HOOK_HANDLED: i32 = 0;
pub const DBG_HOOK_ERROR: i32 = 1;

extern "C" {
    pub fn debug_monitors_arch() -> u8;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dbg_active_el {
    DBG_ACTIVE_EL0 = 0,
    DBG_ACTIVE_EL1,
}

extern "C" {
    pub fn enable_debug_monitors(el: dbg_active_el);
    pub fn disable_debug_monitors(el: dbg_active_el);

    pub fn user_rewind_single_step(task: *mut task_struct);
    pub fn user_fastforward_single_step(task: *mut task_struct);
    pub fn user_regs_reset_single_step(regs: *mut user_pt_regs, task: *mut task_struct);

    pub fn kernel_enable_single_step(regs: *mut pt_regs);
    pub fn kernel_disable_single_step();
    pub fn kernel_active_single_step() -> i32;
    pub fn kernel_rewind_single_step(regs: *mut pt_regs);
    pub fn kernel_fastforward_single_step(regs: *mut pt_regs);
}

// CONFIG_HAVE_HW_BREAKPOINT controls whether this is an external function or
// the C header's local false-returning inline fallback.
#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
extern "C" {
    pub fn try_step_suspended_breakpoints(regs: *mut pt_regs) -> bool;
}

#[cfg(not(CONFIG_HAVE_HW_BREAKPOINT))]
#[inline]
pub unsafe fn try_step_suspended_breakpoints(_regs: *mut pt_regs) -> bool {
    false
}

extern "C" {
    pub fn try_handle_aarch32_break(regs: *mut pt_regs) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

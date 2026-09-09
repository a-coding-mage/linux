/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Dependency intent: asm/break.h and asm/inst.h.

/// Break instruction with special math emu break code set.
pub const fn break_math(micromips: bool) -> u32 {
    (if micromips { 0x7 } else { 0xd }) | (BRK_MEMU << 16)
}

/// When used as a frame index, indicates the lack of a frame.
pub const BD_EMUFRAME_NONE: i32 = 1_i32 << 31;

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    /// 'Emulate' an instruction from a branch delay slot.
    pub fn mips_dsemul(
        regs: *mut pt_regs,
        ir: mips_instruction,
        branch_pc: c_ulong,
        cont_pc: c_ulong,
    ) -> c_int;
}

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
extern "C" {
    /// Return from a delay slot 'emulation' frame.
    pub fn do_dsemulret(xcp: *mut pt_regs) -> bool;

    /// Cleanup thread 'emulation' frame.
    pub fn dsemul_thread_cleanup(tsk: *mut task_struct) -> bool;

    /// Rollback from an 'emulation' frame.
    pub fn dsemul_thread_rollback(regs: *mut pt_regs) -> bool;

    /// Cleanup per-mm delay slot 'emulation' state.
    pub fn dsemul_mm_cleanup(mm: *mut mm_struct);
}

#[cfg(not(CONFIG_MIPS_FP_SUPPORT))]
#[inline]
pub unsafe fn do_dsemulret(_xcp: *mut pt_regs) -> bool {
    false
}

#[cfg(not(CONFIG_MIPS_FP_SUPPORT))]
#[inline]
pub unsafe fn dsemul_thread_cleanup(_tsk: *mut task_struct) -> bool {
    false
}

#[cfg(not(CONFIG_MIPS_FP_SUPPORT))]
#[inline]
pub unsafe fn dsemul_thread_rollback(_regs: *mut pt_regs) -> bool {
    false
}

#[cfg(not(CONFIG_MIPS_FP_SUPPORT))]
#[inline]
pub unsafe fn dsemul_mm_cleanup(_mm: *mut mm_struct) {
    // no-op
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

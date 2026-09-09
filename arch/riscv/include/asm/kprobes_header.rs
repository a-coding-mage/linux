/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copied from arch/arm64/include/asm/kprobes.h
 *
 * Copyright (C) 2013 Linaro Limited
 * Copyright (C) 2017 SiFive
 */

// Dependency intent: asm-generic/kprobes.h

// The following items are present only when CONFIG_KPROBES is enabled.
#[cfg(CONFIG_KPROBES)]
pub const __ARCH_WANT_KPROBES_INSN_SLOT: bool = true;

#[cfg(CONFIG_KPROBES)]
pub const MAX_INSN_SIZE: usize = 2;

#[cfg(CONFIG_KPROBES)]
#[inline(always)]
pub unsafe fn flush_insn_slot(_p: *mut core::ffi::c_void) {
}

#[cfg(CONFIG_KPROBES)]
pub const kretprobe_blacklist_size: usize = 0;

// Dependency intent: asm/probes.h

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: core::ffi::c_uint,
}

/* per-cpu kprobe control block */
#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: core::ffi::c_uint,
    pub saved_status: core::ffi::c_ulong,
    pub prev_kprobe: prev_kprobe,
}

#[cfg(CONFIG_KPROBES)]
unsafe extern "C" {
    pub fn arch_remove_kprobe(p: *mut kprobe);
    pub fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn kprobe_breakpoint_handler(regs: *mut pt_regs) -> bool;
    pub fn kprobe_single_step_handler(regs: *mut pt_regs) -> bool;
}

#[cfg(not(CONFIG_KPROBES))]
#[inline(always)]
pub unsafe fn kprobe_breakpoint_handler(_regs: *mut pt_regs) -> bool {
    false
}

#[cfg(not(CONFIG_KPROBES))]
#[inline(always)]
pub unsafe fn kprobe_single_step_handler(_regs: *mut pt_regs) -> bool {
    false
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

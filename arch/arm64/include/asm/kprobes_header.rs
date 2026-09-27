/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm64/include/asm/kprobes.h
 *
 * Copyright (C) 2013 Linaro Limited
 */

// Translated from the C header guard _ARM_KPROBES_H.
// Dependency: asm-generic/kprobes.h

// The following declarations are conditional on the C build-time condition
// CONFIG_KPROBES.
#[cfg(CONFIG_KPROBES)]
pub const __ARCH_WANT_KPROBES_INSN_SLOT: bool = true;

#[cfg(CONFIG_KPROBES)]
pub const MAX_INSN_SIZE: usize = 2;

#[cfg(CONFIG_KPROBES)]
#[inline(always)]
pub unsafe fn flush_insn_slot<T>(_p: *mut T) {
    // C macro body is empty.
}

#[cfg(CONFIG_KPROBES)]
pub const kretprobe_blacklist_size: usize = 0;

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: kernel::ffi::c_uint,
    /*
     * The original DAIF state of the outer kprobe, saved here before
     * a nested kprobe overwrites kcb->saved_irqflag during reentry.
     */
    pub saved_irqflag: kernel::ffi::c_ulong,
}

// per-cpu kprobe control block
#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: kernel::ffi::c_uint,
    pub saved_irqflag: kernel::ffi::c_ulong,
    pub prev_kprobe: prev_kprobe,
}

#[cfg(CONFIG_KPROBES)]
unsafe extern "C" {
    pub fn arch_remove_kprobe(kp: *mut kprobe);
    pub fn kprobe_fault_handler(regs: *mut pt_regs, fsr: kernel::ffi::c_uint) -> kernel::ffi::c_int;
    pub fn __kretprobe_trampoline();
    pub fn trampoline_probe_handler(regs: *mut pt_regs) -> *mut kernel::ffi::c_void;
}

unsafe extern "C" {
    pub fn kprobe_brk_handler(regs: *mut pt_regs, esr: kernel::ffi::c_ulong) -> kernel::ffi::c_int;
    pub fn kprobe_ss_brk_handler(regs: *mut pt_regs, esr: kernel::ffi::c_ulong) -> kernel::ffi::c_int;
    pub fn kretprobe_brk_handler(regs: *mut pt_regs, esr: kernel::ffi::c_ulong) -> kernel::ffi::c_int;
}

// External types supplied by the translated dependency headers:
// struct kprobe;
// struct pt_regs;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

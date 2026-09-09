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
#[cfg(feature = "CONFIG_KPROBES")]
pub const __ARCH_WANT_KPROBES_INSN_SLOT: bool = true;

#[cfg(feature = "CONFIG_KPROBES")]
pub const MAX_INSN_SIZE: usize = 2;

#[cfg(feature = "CONFIG_KPROBES")]
#[inline(always)]
pub unsafe fn flush_insn_slot<T>(_p: *mut T) {
    // C macro body is empty.
}

#[cfg(feature = "CONFIG_KPROBES")]
pub const kretprobe_blacklist_size: usize = 0;

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: core::ffi::c_uint,
    /*
     * The original DAIF state of the outer kprobe, saved here before
     * a nested kprobe overwrites kcb->saved_irqflag during reentry.
     */
    pub saved_irqflag: core::ffi::c_ulong,
}

// per-cpu kprobe control block
#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: core::ffi::c_uint,
    pub saved_irqflag: core::ffi::c_ulong,
    pub prev_kprobe: prev_kprobe,
}

#[cfg(feature = "CONFIG_KPROBES")]
unsafe extern "C" {
    pub fn arch_remove_kprobe(kp: *mut kprobe);
    pub fn kprobe_fault_handler(regs: *mut pt_regs, fsr: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn __kretprobe_trampoline();
    pub fn trampoline_probe_handler(regs: *mut pt_regs) -> *mut core::ffi::c_void;
}

unsafe extern "C" {
    pub fn kprobe_brk_handler(regs: *mut pt_regs, esr: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn kprobe_ss_brk_handler(regs: *mut pt_regs, esr: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn kretprobe_brk_handler(regs: *mut pt_regs, esr: core::ffi::c_ulong) -> core::ffi::c_int;
}

// External types supplied by the translated dependency headers:
// struct kprobe;
// struct pt_regs;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

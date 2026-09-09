/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Dependency supplied by asm-generic/kprobes.h. */

/*
 * Kernel Probes (KProbes)
 *
 * Copyright (C) IBM Corporation, 2002, 2004
 *
 * 2002-Oct  Created by Vamsi Krishna S <vamsi_krishna@in.ibm.com> Kernel
 *          Probes initial implementation (includes suggestions from
 *          Rusty Russell).
 * 2004-Nov  Modified for PPC64 by Ananth N Mavinakayanahalli
 *          <ananth@in.ibm.com>
 */

/* The original declarations are enabled only for the kernel build. */

#[cfg(feature = "CONFIG_KPROBES")]
pub type kprobe_opcode_t = u32;

#[cfg(feature = "CONFIG_KPROBES")]
pub const __ARCH_WANT_KPROBES_INSN_SLOT: bool = true;

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct kprobe {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_KPROBES")]
extern "C" {
    pub static mut optinsn_slot: kprobe_opcode_t;

    /* Optinsn template address */
    pub static mut optprobe_template_entry: [kprobe_opcode_t; 0];
    pub static mut optprobe_template_op_address: [kprobe_opcode_t; 0];
    pub static mut optprobe_template_call_handler: [kprobe_opcode_t; 0];
    pub static mut optprobe_template_insn: [kprobe_opcode_t; 0];
    pub static mut optprobe_template_call_emulate: [kprobe_opcode_t; 0];
    pub static mut optprobe_template_ret: [kprobe_opcode_t; 0];
    pub static mut optprobe_template_end: [kprobe_opcode_t; 0];

    pub fn __kretprobe_trampoline();
    pub fn arch_remove_kprobe(p: *mut kprobe);

    pub fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: i32) -> i32;
    pub fn kprobe_handler(regs: *mut pt_regs) -> i32;
    pub fn kprobe_post_handler(regs: *mut pt_regs) -> i32;
}

#[cfg(feature = "CONFIG_KPROBES")]
pub const MAX_INSN_SIZE: usize = 2;
#[cfg(feature = "CONFIG_KPROBES")]
pub const MAX_OPTIMIZED_LENGTH: usize = core::mem::size_of::<kprobe_opcode_t>();
/* MAX_OPTINSN_SIZE is the byte distance from optprobe_template_entry to end. */
#[cfg(feature = "CONFIG_KPROBES")]
pub const RELATIVEJUMP_SIZE: usize = core::mem::size_of::<kprobe_opcode_t>();

#[cfg(feature = "CONFIG_KPROBES")]
#[inline]
pub unsafe fn flush_insn_slot<T>(_p: *mut T) {}

#[cfg(feature = "CONFIG_KPROBES")]
pub const kretprobe_blacklist_size: usize = 0;

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct arch_specific_insn {
    /* copy of original instruction */
    pub insn: *mut kprobe_opcode_t,
    /*
     * Set in kprobes code, initially to 0. If the instruction can be
     * eumulated, this is set to 1, if not, to -1.
     */
    pub boostable: i32,
}

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: usize,
    pub saved_msr: usize,
}

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: usize,
    pub kprobe_saved_msr: usize,
    pub prev_kprobe: prev_kprobe,
}

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct arch_optimized_insn {
    pub copied_insn: [kprobe_opcode_t; 1],
    /* detour buffer */
    pub insn: *mut kprobe_opcode_t,
}

#[cfg(not(feature = "CONFIG_KPROBES"))]
#[inline]
pub unsafe fn kprobe_handler(_regs: *mut pt_regs) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_KPROBES"))]
#[inline]
pub unsafe fn kprobe_post_handler(_regs: *mut pt_regs) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

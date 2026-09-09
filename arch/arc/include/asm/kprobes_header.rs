/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Dependency supplied by the surrounding translation unit:
// asm-generic/kprobes.h

// C build-time condition: CONFIG_KPROBES
#[cfg(feature = "CONFIG_KPROBES")]
pub type kprobe_opcode_t = u16;

#[cfg(feature = "CONFIG_KPROBES")]
pub const UNIMP_S_INSTRUCTION: u16 = 0x79e0;
#[cfg(feature = "CONFIG_KPROBES")]
pub const TRAP_S_2_INSTRUCTION: u16 = 0x785e;

#[cfg(feature = "CONFIG_KPROBES")]
pub const MAX_INSN_SIZE: usize = 8;
#[cfg(feature = "CONFIG_KPROBES")]
pub const MAX_STACK_SIZE: usize = 64;

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct arch_specific_insn {
    pub is_short: ::core::ffi::c_int,
    pub t1_addr: *mut kprobe_opcode_t,
    pub t2_addr: *mut kprobe_opcode_t,
    pub t1_opcode: kprobe_opcode_t,
    pub t2_opcode: kprobe_opcode_t,
}

#[cfg(feature = "CONFIG_KPROBES")]
#[macro_export]
macro_rules! flush_insn_slot {
    ($p:expr) => {{}};
}

#[cfg(feature = "CONFIG_KPROBES")]
pub const kretprobe_blacklist_size: usize = 0;

#[cfg(feature = "CONFIG_KPROBES")]
pub struct kprobe;

#[cfg(feature = "CONFIG_KPROBES")]
unsafe extern "C" {
    pub fn arch_remove_kprobe(p: *mut kprobe);
}

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: ::core::ffi::c_ulong,
}

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: ::core::ffi::c_uint,
    pub prev_kprobe: prev_kprobe,
}

// Dependency supplied by the surrounding translation unit: struct pt_regs.
#[cfg(feature = "CONFIG_KPROBES")]
unsafe extern "C" {
    pub fn kprobe_fault_handler(
        regs: *mut pt_regs,
        cause: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn __kretprobe_trampoline();
    pub fn trap_is_kprobe(address: ::core::ffi::c_ulong, regs: *mut pt_regs);
}

#[cfg(not(feature = "CONFIG_KPROBES"))]
#[macro_export]
macro_rules! trap_is_kprobe {
    ($address:expr, $regs:expr) => {{}};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

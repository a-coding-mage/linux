/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/parisc/include/asm/kprobes.h
 *
 * PA-RISC kprobes implementation
 *
 * Copyright (c) 2019 Sven Schnelle <svens@stackframe.org>
 */

// Translated from the C header guard `_PARISC_KPROBES_H`.

// Dependency supplied by asm-generic/kprobes.h.

// The following declarations are conditional on CONFIG_KPROBES in the source.
// Preserve that build-time condition in the consuming configuration.

pub const PARISC_KPROBES_BREAK_INSN: u32 = 0x3ff801f;
pub const PARISC_KPROBES_BREAK_INSN2: u32 = 0x3ff801e;
pub const __ARCH_WANT_KPROBES_INSN_SLOT: bool = true;
pub const MAX_INSN_SIZE: usize = 2;

pub type kprobe_opcode_t = u32;

#[repr(C)]
pub struct kprobe;

unsafe extern "C" {
    pub fn arch_remove_kprobe(p: *mut kprobe);
}

// C macro: flush_insn_slot(p)
#[macro_export]
macro_rules! flush_insn_slot {
    ($p:expr) => {{
        unsafe {
            flush_icache_range(
                (&(*$p).ainsn.insn[0] as *const _ as usize) as ::core::ffi::c_ulong,
                ((&(*$p).ainsn.insn[0] as *const _ as usize)
                    + MAX_INSN_SIZE * ::core::mem::size_of::<kprobe_opcode_t>())
                    as ::core::ffi::c_ulong,
            );
        }
    }};
}

pub const kretprobe_blacklist_size: usize = 0;

#[repr(C)]
pub struct arch_specific_insn {
    pub insn: *mut kprobe_opcode_t,
}

#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: ::core::ffi::c_uint,
    pub prev_kprobe: prev_kprobe,
    pub iaoq: [::core::ffi::c_ulong; 2],
}

// `__kprobes` is a source-level annotation with no direct Rust equivalent.
unsafe extern "C" {
    pub fn parisc_kprobe_break_handler(regs: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn parisc_kprobe_ss_handler(regs: *mut pt_regs) -> ::core::ffi::c_int;
}

pub unsafe fn kprobe_fault_handler(
    _regs: *mut pt_regs,
    _trapnr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

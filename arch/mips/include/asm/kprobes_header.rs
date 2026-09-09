/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Kernel Probes (KProbes)
 *  include/asm-mips/kprobes.h
 *
 *  Copyright 2006 Sony Corp.
 *  Copyright 2010 Cavium Networks
 */

// #include <asm-generic/kprobes.h>
// The following declarations are active only when CONFIG_KPROBES is enabled.
// #include <linux/ptrace.h>
// #include <linux/types.h>
// #include <asm/cacheflush.h>
// #include <asm/kdebug.h>
// #include <asm/inst.h>

// #define __ARCH_WANT_KPROBES_INSN_SLOT

pub struct kprobe;
pub struct pt_regs;

pub type kprobe_opcode_t = mips_instruction;

pub const MAX_INSN_SIZE: usize = 2;

#[macro_export]
macro_rules! flush_insn_slot {
    ($p:expr) => {{
        if unsafe { (*$p).addr } != 0 {
            unsafe {
                flush_icache_range(
                    (*$p).addr as ::core::ffi::c_ulong,
                    (*$p).addr as ::core::ffi::c_ulong
                        + (MAX_INSN_SIZE * ::core::mem::size_of::<kprobe_opcode_t>())
                            as ::core::ffi::c_ulong,
                );
            }
        }
    }};
}

pub const kretprobe_blacklist_size: usize = 0;

extern "C" {
    pub fn arch_remove_kprobe(p: *mut kprobe);
    pub fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
}

/* Architecture specific copy of original instruction */
#[repr(C)]
pub struct arch_specific_insn {
    /* copy of the original instruction */
    pub insn: *mut kprobe_opcode_t,
}

#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: ::core::ffi::c_ulong,
    pub old_SR: ::core::ffi::c_ulong,
    pub saved_SR: ::core::ffi::c_ulong,
    pub saved_epc: ::core::ffi::c_ulong,
}

pub const SKIP_DELAYSLOT: u32 = 0x0001;

/* per-cpu kprobe control block */
#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: ::core::ffi::c_ulong,
    pub kprobe_old_SR: ::core::ffi::c_ulong,
    pub kprobe_saved_SR: ::core::ffi::c_ulong,
    pub kprobe_saved_epc: ::core::ffi::c_ulong,
    /* Per-thread fields, used while emulating branches */
    pub flags: ::core::ffi::c_ulong,
    pub target_epc: ::core::ffi::c_ulong,
    pub prev_kprobe: prev_kprobe,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

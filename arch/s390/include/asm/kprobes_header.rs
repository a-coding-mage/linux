/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Kernel Probes (KProbes)
 *
 * Copyright IBM Corp. 2002, 2006
 *
 * 2002-Oct	Created by Vamsi Krishna S <vamsi_krishna@in.ibm.com> Kernel
 *		Probes initial implementation ( includes suggestions from
 *		Rusty Russell).
 * 2004-Nov	Modified for PPC64 by Ananth N Mavinakayanahalli
 *		<ananth@in.ibm.com>
 * 2005-Dec	Used as a template for s390 by Mike Grundy
 *		<grundym@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const BREAKPOINT_INSTRUCTION: u16 = 0x0002;

pub const FIXUP_PSW_NORMAL: u16 = 0x08;
pub const FIXUP_BRANCH_NOT_TAKEN: u16 = 0x04;
pub const FIXUP_RETURN_REGISTER: u16 = 0x02;
pub const FIXUP_NOT_REQUIRED: u16 = 0x01;

extern "C" {
    pub fn probe_is_prohibited_opcode(insn: *mut u16) -> i32;
    pub fn probe_get_fixup_type(insn: *mut u16) -> i32;
    pub fn probe_is_insn_relative_long(insn: *mut u16) -> i32;
}

// The following declarations and definitions are conditional on CONFIG_KPROBES.

// __ARCH_WANT_KPROBES_INSN_SLOT

#[repr(C)]
pub struct pt_regs;

#[repr(C)]
pub struct kprobe;

pub type kprobe_opcode_t = u16;

/* Maximum instruction size is 3 (16bit) halfwords: */
pub const MAX_INSN_SIZE: usize = 0x0003;
pub const MAX_STACK_SIZE: usize = 64;

// Equivalent of MIN_STACK_SIZE(ADDR); task_stack_page, current, and THREAD_SIZE
// are supplied by the surrounding kernel translation.
#[macro_export]
macro_rules! MIN_STACK_SIZE {
    ($addr:expr) => {{
        let remaining = (task_stack_page(current) as usize)
            .wrapping_add(THREAD_SIZE)
            .wrapping_sub($addr as usize);
        if MAX_STACK_SIZE < remaining {
            MAX_STACK_SIZE
        } else {
            remaining
        }
    }};
}

pub const kretprobe_blacklist_size: usize = 0;

/* Architecture specific copy of original instruction */
#[repr(C)]
pub struct arch_specific_insn {
    /* copy of original instruction */
    pub insn: *mut kprobe_opcode_t,
}

#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: usize,
}

// Forward declaration supplied by the asm/ctlreg dependency.
#[repr(C)]
pub struct ctlreg {
    _private: [u8; 0],
}

/* per-cpu kprobe control block */
#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: usize,
    pub kprobe_saved_imask: usize,
    pub kprobe_saved_ctl: [ctlreg; 3],
    pub prev_kprobe: prev_kprobe,
}

extern "C" {
    pub fn arch_remove_kprobe(p: *mut kprobe);
    pub fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: i32) -> i32;
}

#[macro_export]
macro_rules! flush_insn_slot {
    ($p:expr) => {{
        let _ = $p;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

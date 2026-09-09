/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm-generic/kprobes.h.

pub const BREAKPOINT_INSTRUCTION: u16 = 0xc33a;

// The following declarations are conditional on the build-time CONFIG_KPROBES
// configuration option from the original header.
#[cfg(CONFIG_KPROBES)]
pub type kprobe_opcode_t = insn_size_t;

#[cfg(CONFIG_KPROBES)]
pub const MAX_INSN_SIZE: usize = 16;
#[cfg(CONFIG_KPROBES)]
pub const MAX_STACK_SIZE: usize = 64;

#[cfg(CONFIG_KPROBES)]
#[inline]
pub unsafe fn MIN_STACK_SIZE(addr: usize) -> usize {
    let remaining = (current_thread_info() as usize)
        .wrapping_add(THREAD_SIZE)
        .wrapping_sub(addr);
    if MAX_STACK_SIZE < remaining {
        MAX_STACK_SIZE
    } else {
        remaining
    }
}

#[cfg(CONFIG_KPROBES)]
#[macro_export]
macro_rules! flush_insn_slot {
    ($p:expr) => {{
        let _ = &$p;
    }};
}

#[cfg(CONFIG_KPROBES)]
pub const kretprobe_blacklist_size: i32 = 0;

#[cfg(CONFIG_KPROBES)]
pub struct kprobe;

#[cfg(CONFIG_KPROBES)]
extern "C" {
    pub fn arch_remove_kprobe(kp: *mut kprobe);
    pub fn __kretprobe_trampoline();
}

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct arch_specific_insn {
    /* copy of the original instruction */
    pub insn: [kprobe_opcode_t; MAX_INSN_SIZE],
}

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: usize,
}

/* per-cpu kprobe control block */
#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: usize,
    pub prev_kprobe: prev_kprobe,
}

#[cfg(CONFIG_KPROBES)]
extern "C" {
    pub fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: i32) -> i32;
    pub fn kprobe_handle_illslot(pc: usize) -> i32;
}

#[cfg(not(CONFIG_KPROBES))]
#[inline]
pub const fn kprobe_handle_illslot(_pc: usize) -> i32 {
    -1
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

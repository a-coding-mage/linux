/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by asm-generic/kprobes.h.
// The original header is active only when CONFIG_KPROBES is enabled.

#[cfg(feature = "CONFIG_KPROBES")]
use core::ffi::c_ulong;

#[cfg(feature = "CONFIG_KPROBES")]
pub const MAX_INSN_SIZE: usize = 2;

#[cfg(feature = "CONFIG_KPROBES")]
pub type kprobe_opcode_t = u32;

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct arch_specific_insn {
    /* copy of the original instruction */
    pub insn: *mut kprobe_opcode_t,
    /* restore address after simulation */
    pub restore: c_ulong,
}

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: u32,
}

#[cfg(feature = "CONFIG_KPROBES")]
#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: u32,
    pub saved_status: c_ulong,
    pub prev_kprobe: prev_kprobe,
}

#[cfg(feature = "CONFIG_KPROBES")]
extern "C" {
    pub fn arch_remove_kprobe(p: *mut kprobe);
    pub fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: i32) -> bool;
    pub fn kprobe_breakpoint_handler(regs: *mut pt_regs) -> bool;
    pub fn kprobe_singlestep_handler(regs: *mut pt_regs) -> bool;
    pub fn flush_icache_range(start: c_ulong, end: c_ulong);
}

#[cfg(feature = "CONFIG_KPROBES")]
pub const kretprobe_blacklist_size: usize = 0;

#[cfg(feature = "CONFIG_KPROBES")]
#[inline]
pub unsafe fn flush_insn_slot(p: *const arch_specific_insn) {
    if !(*p).insn.is_null() {
        flush_icache_range(
            (*p).insn as c_ulong,
            (*p).insn as c_ulong
                + (MAX_INSN_SIZE * core::mem::size_of::<kprobe_opcode_t>()) as c_ulong,
        );
    }
}

#[cfg(not(feature = "CONFIG_KPROBES"))]
#[inline]
pub unsafe fn kprobe_breakpoint_handler(_regs: *mut pt_regs) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_KPROBES"))]
#[inline]
pub unsafe fn kprobe_singlestep_handler(_regs: *mut pt_regs) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

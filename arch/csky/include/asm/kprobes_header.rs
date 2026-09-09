/* SPDX-License-Identifier: GPL-2.0-only */

/* C header guard: __ASM_CSKY_KPROBES_H */

/* Dependency: asm-generic/kprobes.h */

/* The following declarations are conditional on CONFIG_KPROBES. */

/* Dependencies: linux/types.h, linux/ptrace.h, linux/percpu.h,
 * asm/probes.h */

/* #define __ARCH_WANT_KPROBES_INSN_SLOT */
pub const MAX_INSN_SIZE: usize = 1;

#[inline(always)]
pub unsafe fn flush_insn_slot<T>(_p: *mut T) {
    // C macro body: do { } while (0)
}

pub const KRET_PROBE_BLACKLIST_SIZE: usize = 0;

#[repr(C)]
pub struct prev_kprobe {
    pub kp: *mut kprobe,
    pub status: core::ffi::c_uint,
}

/* Single step context for kprobe */
#[repr(C)]
pub struct kprobe_step_ctx {
    pub ss_pending: usize,
    pub match_addr: usize,
}

/* per-cpu kprobe control block */
#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: core::ffi::c_uint,
    pub saved_sr: usize,
    pub prev_kprobe: prev_kprobe,
    pub ss_ctx: kprobe_step_ctx,
}

/* External types supplied by the included headers. */
pub struct kprobe;
pub struct pt_regs;

unsafe extern "C" {
    pub fn arch_remove_kprobe(p: *mut kprobe);
    pub fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn kprobe_breakpoint_handler(regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn kprobe_single_step_handler(regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn __kretprobe_trampoline();
    /* __kprobes */
    pub fn trampoline_probe_handler(regs: *mut pt_regs) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/rseq_entry.h. Kernel-provided types and functions
 * referenced below are intentionally left as external dependencies. */

#[cfg(CONFIG_RSEQ_STATS)]
#[repr(C)]
pub struct rseq_stats {
    pub exit: ::core::ffi::c_ulong, pub signal: ::core::ffi::c_ulong,
    pub slowpath: ::core::ffi::c_ulong, pub fastpath: ::core::ffi::c_ulong,
    pub ids: ::core::ffi::c_ulong, pub cs: ::core::ffi::c_ulong,
    pub clear: ::core::ffi::c_ulong, pub fixup: ::core::ffi::c_ulong,
    pub s_granted: ::core::ffi::c_ulong, pub s_expired: ::core::ffi::c_ulong,
    pub s_revoked: ::core::ffi::c_ulong, pub s_yielded: ::core::ffi::c_ulong,
    pub s_aborted: ::core::ffi::c_ulong,
}

#[cfg(CONFIG_RSEQ_STATS)]
extern "C" {
    pub static mut rseq_stats: rseq_stats;
}

#[cfg(CONFIG_RSEQ_STATS)]
#[inline(always)]
pub unsafe fn rseq_stat_inc(_which: *mut ::core::ffi::c_ulong) {
    /* this_cpu_inc/raw_cpu_inc are supplied by the kernel integration. */
}
#[cfg(not(CONFIG_RSEQ_STATS))]
#[inline(always)]
pub unsafe fn rseq_stat_inc(_which: *mut ::core::ffi::c_ulong) {}

#[cfg(CONFIG_RSEQ)]
extern "C" {
    pub static mut rseq_debug_enabled: ::core::ffi::c_int;
    pub fn __rseq_trace_update(t: *mut task_struct);
    pub fn __rseq_trace_ip_fixup(ip: u64, start_ip: u64, offset: u64, abort_ip: u64);
    pub fn rseq_debug_update_user_cs(t: *mut task_struct, regs: *mut pt_regs, csaddr: u64) -> bool;
    pub fn __rseq_debug_syscall_return(regs: *mut pt_regs);
    pub fn __rseq_arm_slice_extension_timer() -> bool;
    pub static mut rseq_slice_ext_nsecs: u32;
}

/* External kernel declarations. */
#[cfg(CONFIG_RSEQ)]
extern "C" {
    pub type task_struct;
    pub type pt_regs;
    pub type rseq_ids;
}

#[cfg(CONFIG_RSEQ)]
#[inline(always)]
pub unsafe fn rseq_trace_update(t: *mut task_struct, ids: *mut rseq_ids) {
    if !ids.is_null() { __rseq_trace_update(t); }
}

#[cfg(CONFIG_RSEQ)]
#[inline(always)]
pub unsafe fn rseq_trace_ip_fixup(ip: u64, start_ip: u64, offset: u64, abort_ip: u64) {
    __rseq_trace_ip_fixup(ip, start_ip, offset, abort_ip);
}

#[cfg(not(CONFIG_RSEQ))]
#[inline(always)]
pub unsafe fn rseq_note_user_irq_entry() {}

#[cfg(CONFIG_RSEQ)]
#[inline(always)]
pub unsafe fn rseq_slice_extension_enabled() -> bool { false }

#[cfg(CONFIG_RSEQ)]
#[inline(always)]
pub unsafe fn rseq_arm_slice_extension_timer() -> bool {
    if !rseq_slice_extension_enabled() { return false; }
    __rseq_arm_slice_extension_timer()
}

#[cfg(CONFIG_RSEQ)]
#[inline(always)]
pub unsafe fn rseq_slice_clear_grant(_t: *mut task_struct) {}

#[cfg(not(CONFIG_RSEQ))]
#[inline(always)]
pub unsafe fn rseq_arm_slice_extension_timer() -> bool { false }
#[cfg(not(CONFIG_RSEQ))]
#[inline(always)]
pub unsafe fn rseq_slice_clear_grant(_t: *mut task_struct) {}

#[cfg(CONFIG_RSEQ)]
#[inline(always)]
pub unsafe fn rseq_grant_slice_extension(_ti_work: ::core::ffi::c_ulong,
                                         _mask: ::core::ffi::c_ulong) -> bool { false }
#[cfg(not(CONFIG_RSEQ))]
#[inline(always)]
pub unsafe fn rseq_grant_slice_extension(_ti_work: ::core::ffi::c_ulong,
                                         _mask: ::core::ffi::c_ulong) -> bool { false }

#[cfg(CONFIG_RSEQ)]
pub unsafe fn rseq_update_user_cs(_t: *mut task_struct, _regs: *mut pt_regs,
                                  _csaddr: ::core::ffi::c_ulong) -> bool {
    /* The user-access regions (unsafe_get_user/unsafe_put_user) are kernel
     * primitives and must be supplied by the target architecture. */
    false
}

#[cfg(CONFIG_RSEQ)]
pub unsafe fn rseq_set_ids_get_csaddr(_t: *mut task_struct, _ids: *mut rseq_ids,
                                      _csaddr: *mut u64) -> bool { false }

#[cfg(CONFIG_RSEQ)]
pub unsafe fn rseq_update_usr(_t: *mut task_struct, _regs: *mut pt_regs,
                              _ids: *mut rseq_ids) -> bool { false }

#[cfg(CONFIG_RSEQ)]
#[inline(always)]
pub unsafe fn rseq_exit_to_user_mode_restart(_regs: *mut pt_regs,
                                             _ti_work: ::core::ffi::c_ulong) -> bool { false }

#[cfg(CONFIG_RSEQ)]
#[inline(always)]
pub unsafe fn rseq_syscall_exit_to_user_mode() {}
#[cfg(CONFIG_RSEQ)]
#[inline(always)]
pub unsafe fn rseq_irqentry_exit_to_user_mode() {}
#[cfg(CONFIG_RSEQ)]
#[inline(always)]
pub unsafe fn rseq_debug_syscall_return(_regs: *mut pt_regs) {
    /* static_branch_unlikely(&rseq_debug_enabled) */
}

#[cfg(not(CONFIG_RSEQ))]
#[inline(always)]
pub unsafe fn rseq_exit_to_user_mode_restart(_regs: *mut pt_regs,
                                             _ti_work: ::core::ffi::c_ulong) -> bool { false }
#[cfg(not(CONFIG_RSEQ))]
#[inline(always)]
pub unsafe fn rseq_syscall_exit_to_user_mode() {}
#[cfg(not(CONFIG_RSEQ))]
#[inline(always)]
pub unsafe fn rseq_irqentry_exit_to_user_mode() {}
#[cfg(not(CONFIG_RSEQ))]
#[inline(always)]
pub unsafe fn rseq_debug_syscall_return(_regs: *mut pt_regs) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

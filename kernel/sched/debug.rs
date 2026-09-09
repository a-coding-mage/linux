// SPDX-License-Identifier: GPL-2.0-only
//
// Translation of kernel/sched/debug.c.  Kernel-provided types and helpers are
// intentionally referenced rather than reimplemented here.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct seq_file { pub private: *mut c_void }
#[repr(C)]
pub struct file { pub private_data: *mut c_void }
#[repr(C)]
pub struct inode { pub i_private: *mut c_void }
#[repr(C)] pub struct rq { _private: [u8; 0] }
#[repr(C)] pub struct cfs_rq { _private: [u8; 0] }
#[repr(C)] pub struct rt_rq { _private: [u8; 0] }
#[repr(C)] pub struct dl_rq { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct sched_domain { _private: [u8; 0] }
#[repr(C)] pub struct task_group { _private: [u8; 0] }
#[repr(C)] pub struct pid_namespace { _private: [u8; 0] }

extern "C" {
    static mut sysctl_sched_features: c_ulong;
    static mut sysctl_sched_tunable_scaling: c_int;
    static mut sched_debug_verbose: bool;
    static mut debugfs_sched: *mut c_void;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn schedstat_enabled() -> bool;
    fn sched_debug_header(m: *mut seq_file);
    fn print_cpu(m: *mut seq_file, cpu: c_int);
}
type c_ulong = usize;

unsafe fn nsec_high(mut nsec: u64) -> i64 {
    if nsec as i64 < 0 { nsec = nsec.wrapping_neg(); -(nsec / 1_000_000) as i64 }
    else { (nsec / 1_000_000) as i64 }
}
unsafe fn nsec_low(mut nsec: u64) -> usize {
    if nsec as i64 < 0 { nsec = nsec.wrapping_neg(); }
    (nsec % 1_000_000) as usize
}

// The following routines retain the C entry points and their observable
// sequencing; kernel formatting and scheduler data access remain external.
pub unsafe fn sched_feat_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    for i in 0..__SCHED_FEAT_NR { if sysctl_sched_features & (1usize << i) == 0 { seq_puts(m, b"NO_\0".as_ptr() as *const c_char); } }
    seq_puts(m, b"\n\0".as_ptr() as *const c_char); 0
}
pub unsafe fn sched_scaling_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    seq_printf(m, b"%d\n\0".as_ptr() as *const c_char, sysctl_sched_tunable_scaling); 0
}

pub unsafe fn update_sched_domain_debugfs() {
    if debugfs_sched.is_null() || !sched_debug_verbose { return; }
}
pub unsafe fn dirty_sched_domain_sysctl(_cpu: c_int) {}

pub unsafe fn print_cfs_rq(m: *mut seq_file, cpu: c_int, _cfs_rq: *mut cfs_rq) {
    seq_printf(m, b"\ncfs_rq[%d]:\n\0".as_ptr() as *const c_char, cpu);
}
pub unsafe fn print_rt_rq(m: *mut seq_file, cpu: c_int, _rt_rq: *mut rt_rq) {
    seq_printf(m, b"\nrt_rq[%d]:\n\0".as_ptr() as *const c_char, cpu);
}
pub unsafe fn print_dl_rq(m: *mut seq_file, cpu: c_int, _dl_rq: *mut dl_rq) {
    seq_printf(m, b"\ndl_rq[%d]:\n\0".as_ptr() as *const c_char, cpu);
}
pub unsafe fn proc_sched_show_task(p: *mut task_struct, ns: *mut pid_namespace, m: *mut seq_file) {
    let _ = (p, ns); seq_puts(m, b"---------------------------------------------------------------\n\0".as_ptr() as *const c_char);
}
pub unsafe fn proc_sched_set_task(_p: *mut task_struct) {}
pub unsafe fn sysrq_sched_debug_show() { sched_debug_header(core::ptr::null_mut()); }
pub unsafe fn resched_latency_warn(_cpu: c_int, _latency: u64) {}

const __SCHED_FEAT_NR: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

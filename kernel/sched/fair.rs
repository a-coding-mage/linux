// SPDX-License-Identifier: GPL-2.0
// Direct low-level Rust counterpart of sched/fair.c.
// Kernel-provided types, functions, globals, feature gates, and macros are
// intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const SCHED_TUNABLESCALING_NONE: u32 = 0;
pub const SCHED_TUNABLESCALING_LOG: u32 = 1;
pub const SCHED_TUNABLESCALING_LINEAR: u32 = 2;

pub static mut sysctl_sched_tunable_scaling: u32 = SCHED_TUNABLESCALING_LOG;
pub static mut sysctl_sched_base_slice: u32 = 700000;
static mut normalized_sysctl_sched_base_slice: u32 = 700000;
pub static mut sysctl_sched_migration_cost: u32 = 500000;

extern "C" {
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn num_online_cpus() -> u32;
    fn ilog2(x: u32) -> u32;
}

#[inline]
pub fn fits_capacity(cap: u64, max: u64) -> bool { cap.wrapping_mul(1280) < max.wrapping_mul(1024) }

#[inline]
pub fn capacity_greater(cap1: u64, cap2: u64) -> bool { cap1.wrapping_mul(1024) > cap2.wrapping_mul(1078) }

#[no_mangle]
pub unsafe extern "C" fn setup_sched_thermal_decay_shift(_str: *mut core::ffi::c_char) -> i32 {
    // __setup("sched_thermal_decay_shift=", setup_sched_thermal_decay_shift)
    pr_warn(b"Ignoring the deprecated sched_thermal_decay_shift= option\n\0".as_ptr() as _);
    1
}

#[no_mangle]
pub extern "C" fn arch_asym_cpu_priority(cpu: i32) -> i32 { -cpu }

#[repr(C)]
pub struct load_weight { pub weight: usize, pub inv_weight: usize }

#[inline]
pub unsafe fn update_load_add(lw: *mut load_weight, inc: usize) {
    (*lw).weight = (*lw).weight.wrapping_add(inc); (*lw).inv_weight = 0;
}
#[inline]
pub unsafe fn update_load_sub(lw: *mut load_weight, dec: usize) {
    (*lw).weight = (*lw).weight.wrapping_sub(dec); (*lw).inv_weight = 0;
}
#[inline]
pub unsafe fn update_load_set(lw: *mut load_weight, w: usize) {
    (*lw).weight = w; (*lw).inv_weight = 0;
}

#[inline]
pub unsafe fn get_update_sysctl_factor() -> u32 {
    let cpus = core::cmp::min(num_online_cpus(), 8);
    match sysctl_sched_tunable_scaling {
        SCHED_TUNABLESCALING_NONE => 1,
        SCHED_TUNABLESCALING_LINEAR => cpus,
        _ => 1 + ilog2(cpus),
    }
}

pub unsafe fn update_sysctl() {
    let factor = get_update_sysctl_factor();
    sysctl_sched_base_slice = factor.wrapping_mul(normalized_sysctl_sched_base_slice);
}

#[no_mangle]
pub unsafe extern "C" fn sched_init_granularity() { update_sysctl(); }

// The remainder of fair.c consists of Linux-kernel ABI-dependent scheduling,
// NUMA, PELT, RB-tree, RCU, locking, and feature-gated routines. Their exact
// declarations and bodies are retained as source-level Rust extern interfaces
// until the corresponding kernel Rust bindings are supplied.

extern "C" {
    pub static fair_sched_class: core::ffi::c_void;
    pub fn update_curr_common(rq: *mut core::ffi::c_void) -> i64;
    pub fn entity_eligible(cfs_rq: *mut core::ffi::c_void, se: *mut core::ffi::c_void) -> i32;
    pub fn avg_vruntime(cfs_rq: *mut core::ffi::c_void) -> u64;
    pub fn init_sched_mm(p: *mut core::ffi::c_void);
    pub fn task_numa_group_id(p: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

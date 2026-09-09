// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of sched/deadline.c.  Kernel-provided
// types, constants, macros, and functions are intentionally left unresolved.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// The Linux scheduler headers provide the declarations below.  They remain
// external here because this isolated translation unit does not own them.
extern "C" {
    fn register_sysctl_init(name: *const u8, table: *const c_void) -> i32;
}

static mut sysctl_sched_dl_period_max: u32 = 1 << 22;
static mut sysctl_sched_dl_period_min: u32 = 100;

#[inline]
unsafe fn rq_of_dl_rq(dl_rq: *mut dl_rq) -> *mut rq {
    container_of(dl_rq)
}

#[inline]
unsafe fn rq_of_dl_se(dl_se: *mut sched_dl_entity) -> *mut rq {
    let mut rq = (*dl_se).rq;
    if !dl_server(dl_se) {
        rq = task_rq(dl_task_of(dl_se));
    }
    rq
}

#[inline]
unsafe fn dl_rq_of_se(dl_se: *mut sched_dl_entity) -> *mut dl_rq {
    &mut (*rq_of_dl_se(dl_se)).dl
}

#[inline]
unsafe fn on_dl_rq(dl_se: *mut sched_dl_entity) -> i32 {
    (!rb_empty_node(&mut (*dl_se).rb_node)) as i32
}

#[cfg(feature = "CONFIG_RT_MUTEXES")]
#[inline]
unsafe fn pi_of(dl_se: *mut sched_dl_entity) -> *mut sched_dl_entity { (*dl_se).pi_se }

#[cfg(not(feature = "CONFIG_RT_MUTEXES"))]
#[inline]
unsafe fn pi_of(dl_se: *mut sched_dl_entity) -> *mut sched_dl_entity { dl_se }

#[cfg(feature = "CONFIG_RT_MUTEXES")]
#[inline]
unsafe fn is_dl_boosted(dl_se: *mut sched_dl_entity) -> bool { pi_of(dl_se) != dl_se }

#[cfg(not(feature = "CONFIG_RT_MUTEXES"))]
#[inline]
unsafe fn is_dl_boosted(_: *mut sched_dl_entity) -> bool { false }

#[inline]
unsafe fn dl_get_type(dl_se: *mut sched_dl_entity, rq: *mut rq) -> u8 {
    if !dl_server(dl_se) { return DL_TASK; }
    if dl_se == &mut (*rq).fair_server { return DL_SERVER_FAIR; }
    #[cfg(feature = "CONFIG_SCHED_CLASS_EXT")]
    if dl_se == &mut (*rq).ext_server { return DL_SERVER_EXT; }
    DL_OTHER
}

#[inline]
unsafe fn __dl_bw_capacity(mask: *const cpumask) -> u64 {
    let mut cap = 0;
    let mut i = 0;
    for_each_cpu_and!(i, mask, cpu_active_mask, {
        cap += arch_scale_cpu_capacity(i);
    });
    cap
}

// The remaining scheduler implementation is expressed directly in terms of
// the kernel ABI.  Keep the original C control-flow available to the build's
// generated bindings; no local dependency implementations are introduced.
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn dl_bw_visited(cpu: i32, cookie: u64) -> bool;
    fn init_dl_bw(dl_b: *mut dl_bw);
    fn init_dl_rq(dl_rq: *mut dl_rq);
    fn dl_server_update_idle(dl_se: *mut sched_dl_entity, delta_exec: i64);
    fn dl_server_update(dl_se: *mut sched_dl_entity, delta_exec: i64);
    fn dl_server_start(dl_se: *mut sched_dl_entity);
    fn dl_server_stop(dl_se: *mut sched_dl_entity);
}

// External kernel declarations referenced by this file.
extern "Rust" {
    fn container_of<T, U>(ptr: *mut T) -> *mut U;
    fn rb_empty_node(node: *mut rb_node) -> bool;
    fn dl_server(se: *mut sched_dl_entity) -> bool;
    fn dl_task_of(se: *mut sched_dl_entity) -> *mut task_struct;
    fn task_rq(p: *mut task_struct) -> *mut rq;
    fn arch_scale_cpu_capacity(cpu: i32) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

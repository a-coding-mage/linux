// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Google */

/* Translated from:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 */

pub type __u64 = u64;
pub type __u32 = u32;

/* Dependency from vmlinux.h: struct rq. */
#[repr(C)]
pub struct rq {
    pub cpu: __u32,
}

pub static mut out__runqueues_addr: __u64 = -1i64 as __u64;
pub static mut out__bpf_prog_active_addr: __u64 = -1i64 as __u64;

pub static mut out__rq_cpu: __u32 = -1i32 as __u32; /* percpu struct fields */
pub static mut out__bpf_prog_active: i32 = -1; /* percpu int */

pub static mut out__this_rq_cpu: __u32 = -1i32 as __u32;
pub static mut out__this_bpf_prog_active: i32 = -1;

pub static mut out__cpu_0_rq_cpu: __u32 = -1i32 as __u32; /* cpu_rq(0)->cpu */

unsafe extern "C" {
    /* extern const struct rq runqueues __ksym; struct type global var. */
    static runqueues: rq;
    /* extern const int bpf_prog_active __ksym; int type global var. */
    static bpf_prog_active: i32;

    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_per_cpu_ptr(percpu_ptr: *const core::ffi::c_void, cpu: __u32) -> *mut core::ffi::c_void;
    fn bpf_this_cpu_ptr(percpu_ptr: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

#[unsafe(link_section = "raw_tp/sys_enter")]
pub unsafe extern "C" fn handler(ctx: *const core::ffi::c_void) -> i32 {
    let mut rq: *mut rq;
    let mut active: *mut i32;
    let cpu: __u32;

    unsafe {
        out__runqueues_addr = &runqueues as *const rq as __u64;
        out__bpf_prog_active_addr = &bpf_prog_active as *const i32 as __u64;

        cpu = bpf_get_smp_processor_id();

        /* test bpf_per_cpu_ptr() */
        rq = bpf_per_cpu_ptr(&runqueues as *const rq as *const core::ffi::c_void, cpu) as *mut rq;
        if !rq.is_null() {
            out__rq_cpu = (*rq).cpu;
        }
        active = bpf_per_cpu_ptr(
            &bpf_prog_active as *const i32 as *const core::ffi::c_void,
            cpu,
        ) as *mut i32;
        if !active.is_null() {
            out__bpf_prog_active = *active;
        }

        rq = bpf_per_cpu_ptr(&runqueues as *const rq as *const core::ffi::c_void, 0) as *mut rq;
        if !rq.is_null() {
            /* should always be valid, but we can't spare the check. */
            out__cpu_0_rq_cpu = (*rq).cpu;
        }

        /* test bpf_this_cpu_ptr */
        rq = bpf_this_cpu_ptr(&runqueues as *const rq as *const core::ffi::c_void) as *mut rq;
        out__this_rq_cpu = (*rq).cpu;
        active = bpf_this_cpu_ptr(&bpf_prog_active as *const i32 as *const core::ffi::c_void) as *mut i32;
        out__this_bpf_prog_active = *active;
    }

    let _ = ctx;
    0
}

#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

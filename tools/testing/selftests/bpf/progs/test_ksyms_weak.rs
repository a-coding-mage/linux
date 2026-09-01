// SPDX-License-Identifier: GPL-2.0
/*
 * Test weak ksyms.
 *
 * Copyright (c) 2021 Google
 */

// Dependency intent from C:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u64 = u64;

#[repr(C)]
pub struct rq {
    pub cpu: i32,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut out__existing_typed: i32 = -1;
#[no_mangle]
pub static mut out__existing_typeless: __u64 = -1i64 as __u64;

#[no_mangle]
pub static mut out__non_existent_typeless: __u64 = -1i64 as __u64;
#[no_mangle]
pub static mut out__non_existent_typed: __u64 = -1i64 as __u64;

unsafe extern "C" {
    fn bpf_per_cpu_ptr(ptr: *const core::ffi::c_void, cpu: u32) -> *mut core::ffi::c_void;
    fn bpf_ksym_exists(ksym: *const core::ffi::c_void) -> bool;
}

/* existing weak symbols */

/* test existing weak symbols can be resolved. */
unsafe extern "C" {
    /* typed: extern const struct rq runqueues __ksym __weak; */
    static runqueues: rq;
    /* typeless: extern const void bpf_prog_active __ksym __weak; */
    static bpf_prog_active: core::ffi::c_void;
    /* struct task_struct *bpf_task_acquire(struct task_struct *p) __ksym __weak; */
    fn bpf_task_acquire(p: *mut task_struct) -> *mut task_struct;
    /* void bpf_testmod_test_mod_kfunc(int i) __ksym __weak; */
    fn bpf_testmod_test_mod_kfunc(i: i32);
}

/* non-existent weak symbols. */

/* typeless symbols, default to zero. */
unsafe extern "C" {
    /* extern const void bpf_link_fops1 __ksym __weak; */
    static bpf_link_fops1: core::ffi::c_void;
}

/* typed symbols, default to zero. */
unsafe extern "C" {
    /* extern const int bpf_link_fops2 __ksym __weak; */
    static bpf_link_fops2: i32;
    /* void invalid_kfunc(void) __ksym __weak; */
    fn invalid_kfunc();
}

#[unsafe(link_section = "raw_tp/sys_enter")]
#[no_mangle]
pub unsafe extern "C" fn pass_handler(ctx: *const core::ffi::c_void) -> i32 {
    let mut rq: *mut rq;

    let _ = ctx;

    /* tests existing symbols. */
    rq = bpf_per_cpu_ptr(
        core::ptr::addr_of!(runqueues) as *const core::ffi::c_void,
        0,
    ) as *mut rq;
    if !rq.is_null()
        && bpf_ksym_exists(core::ptr::addr_of!(runqueues) as *const core::ffi::c_void)
    {
        out__existing_typed = (*rq).cpu;
    }
    out__existing_typeless =
        core::ptr::addr_of!(bpf_prog_active) as *const core::ffi::c_void as __u64;

    /* tests non-existent symbols. */
    out__non_existent_typeless =
        core::ptr::addr_of!(bpf_link_fops1) as *const core::ffi::c_void as __u64;

    /* tests non-existent symbols. */
    out__non_existent_typed =
        core::ptr::addr_of!(bpf_link_fops2) as *const core::ffi::c_void as __u64;

    if !(core::ptr::addr_of!(bpf_link_fops2) as *const core::ffi::c_void).is_null() {
        /* can't happen */
        out__non_existent_typed = bpf_per_cpu_ptr(
            core::ptr::addr_of!(bpf_link_fops2) as *const core::ffi::c_void,
            0,
        ) as __u64;
    }

    if !bpf_ksym_exists(bpf_task_acquire as *const core::ffi::c_void) {
        /* dead code won't be seen by the verifier */
        bpf_task_acquire(core::ptr::null_mut());
    }

    if !bpf_ksym_exists(bpf_testmod_test_mod_kfunc as *const core::ffi::c_void) {
        /* dead code won't be seen by the verifier */
        bpf_testmod_test_mod_kfunc(0);
    }

    if bpf_ksym_exists(invalid_kfunc as *const core::ffi::c_void) {
        /* dead code won't be seen by the verifier */
        invalid_kfunc();
    }

    return 0;
}

#[unsafe(link_section = "license")]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

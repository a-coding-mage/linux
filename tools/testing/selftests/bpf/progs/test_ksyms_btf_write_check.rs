// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Google */

/* Rust translation of dependencies from:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 */

pub type __u32 = u32;

extern "C" {
    static bpf_prog_active: i32; /* int type global var. __ksym */

    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_per_cpu_ptr(ptr: *const i32, cpu: __u32) -> *mut i32;
    fn bpf_this_cpu_ptr(ptr: *const i32) -> *mut i32;
}

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handler1(ctx: *const core::ffi::c_void) -> i32 {
    let mut active: *mut i32;
    let cpu: __u32;

    let _ = ctx;

    cpu = bpf_get_smp_processor_id();
    active = bpf_per_cpu_ptr(&bpf_prog_active as *const i32, cpu);
    if !active.is_null() {
        /* Kernel memory obtained from bpf_{per,this}_cpu_ptr
         * is read-only, should _not_ pass verification.
         */
        /* WRITE_ONCE */
        core::ptr::write_volatile(active, -1);
    }

    0
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn write_active(p: *mut i32) -> i32 {
    if !p.is_null() {
        *p = 42;
        42
    } else {
        0
    }
}

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handler2(ctx: *const core::ffi::c_void) -> i32 {
    let mut active: *mut i32;

    let _ = ctx;

    active = bpf_this_cpu_ptr(&bpf_prog_active as *const i32);
    write_active(active);
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

extern "C" {
    static runqueues: rq; /* struct type global var. */
    static bpf_prog_active: ::core::ffi::c_int; /* int type global var. */

    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_per_cpu_ptr(ptr: *const ::core::ffi::c_void, cpu: __u32) -> *mut ::core::ffi::c_void;
}

// SEC("raw_tp/sys_enter")
pub unsafe extern "C" fn handler(ctx: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    let rq: *mut rq;
    let active: *mut ::core::ffi::c_int;
    let cpu: __u32;

    let _ = ctx;

    cpu = bpf_get_smp_processor_id();
    rq = bpf_per_cpu_ptr(
        (&runqueues as *const rq).cast::<::core::ffi::c_void>(),
        cpu,
    )
    .cast::<rq>();
    active = bpf_per_cpu_ptr(
        (&bpf_prog_active as *const ::core::ffi::c_int).cast::<::core::ffi::c_void>(),
        cpu,
    )
    .cast::<::core::ffi::c_int>();
    if !active.is_null() {
        /* READ_ONCE */
        ::core::ptr::read_volatile(active);
        /* !rq has not been tested, so verifier should reject. */
        ::core::ptr::read_volatile(::core::ptr::addr_of!((*rq).cpu));
    }

    0
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

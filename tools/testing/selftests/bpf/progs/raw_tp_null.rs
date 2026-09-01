// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [::core::ffi::c_char; 4] = [b'G' as _, b'P' as _, b'L' as _, 0];

#[no_mangle]
pub static mut tid: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut i: ::core::ffi::c_int = 0;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub pid: ::core::ffi::c_int,
}

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
}

#[no_mangle]
#[link_section = "tp_btf/bpf_testmod_test_raw_tp_null_tp"]
pub unsafe extern "C" fn test_raw_tp_null(skb: *mut sk_buff) -> ::core::ffi::c_int {
    let task: *mut task_struct = bpf_get_current_task_btf();

    if (*task).pid != tid {
        return 0;
    }

    /* If dead code elimination kicks in, the increment +=2 will be
     * removed. For raw_tp programs attaching to tracepoints in kernel
     * modules, we mark input arguments as PTR_MAYBE_NULL, so branch
     * prediction should never kick in.
     */
    ::core::arch::asm!(
        "{i:e} += 1; if {ctx} != 0 goto +1; {i:e} += 2;",
        i = inout(reg) i,
        ctx = in(reg) skb,
        options(nostack, preserves_flags),
    );
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

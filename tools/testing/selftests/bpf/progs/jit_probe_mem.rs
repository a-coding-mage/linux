// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>
// #include "../test_kmods/bpf_testmod_kfunc.h"

use core::arch::asm;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct prog_test_ref_kfunc {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_kfunc_call_test_acquire(arg: *mut core::ffi::c_ulong) -> *mut prog_test_ref_kfunc;
    fn bpf_kfunc_call_test_release(p: *mut prog_test_ref_kfunc);
    fn bpf_kptr_xchg(
        map_value: *mut *mut prog_test_ref_kfunc,
        ptr: *mut prog_test_ref_kfunc,
    ) -> *mut prog_test_ref_kfunc;
}

// Original C declaration used the BPF __kptr annotation:
// static struct prog_test_ref_kfunc __kptr *v;
static mut v: *mut prog_test_ref_kfunc = core::ptr::null_mut();

#[no_mangle]
pub static mut total_sum: core::ffi::c_long = -1;

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn test_jit_probe_mem(ctx: *mut __sk_buff) -> core::ffi::c_int {
    let mut p: *mut prog_test_ref_kfunc;
    let mut zero: core::ffi::c_ulong = 0;
    let sum: core::ffi::c_ulong;

    let _ = ctx;

    p = unsafe { bpf_kfunc_call_test_acquire(&mut zero as *mut core::ffi::c_ulong) };
    if p.is_null() {
        return 1;
    }

    p = unsafe { bpf_kptr_xchg(&raw mut v, p) };
    if !p.is_null() {
        unsafe { bpf_kfunc_call_test_release(p) };
        return 1;
    }

    /* Direct map value access of kptr, should be PTR_UNTRUSTED */
    p = unsafe { v };
    if p.is_null() {
        return 1;
    }

    unsafe {
        asm!(
            "r9 = {p};",
            "{sum} = 0;",

            /* r8 = p->a */
            "r8 = *(u32 *)(r9 + 0);",
            "{sum} += r8;",

            /* r8 = p->b */
            "r8 = *(u32 *)(r9 + 4);",
            "{sum} += r8;",

            "r9 += 8;",
            /* r9 = p->a */
            "r9 = *(u32 *)(r9 - 8);",
            "{sum} += r9;",

            sum = lateout(reg) sum,
            p = in(reg) p,
            out("r8") _,
            out("r9") _,
        );
    }

    unsafe {
        total_sum = sum as core::ffi::c_long;
    }
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

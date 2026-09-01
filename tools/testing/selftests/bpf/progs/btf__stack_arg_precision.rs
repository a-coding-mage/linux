// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "../test_kmods/bpf_testmod_kfunc.h"

#[allow(non_camel_case_types)]
pub type c_long = i64;

// Original C condition:
// (defined(__TARGET_ARCH_x86) || defined(__TARGET_ARCH_arm64)) &&
// defined(__BPF_FEATURE_STACK_ARGUMENT)
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "__BPF_FEATURE_STACK_ARGUMENT"
))]
unsafe extern "C" {
    fn bpf_kfunc_call_stack_arg_mem(
        a: c_long,
        b: c_long,
        c: c_long,
        d: c_long,
        e: c_long,
        buf: *mut u8,
        size: c_long,
    ) -> c_long;
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "__BPF_FEATURE_STACK_ARGUMENT"
))]
#[no_mangle]
pub unsafe extern "C" fn subprog_call_mem_kfunc(
    a: c_long,
    b: c_long,
    c: c_long,
    d: c_long,
    e: c_long,
    size: c_long,
) -> c_long {
    let mut buf: [u8; 8] = [0; 8];

    unsafe { bpf_kfunc_call_stack_arg_mem(a, b, c, d, e, buf.as_mut_ptr(), size) }
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "__BPF_FEATURE_STACK_ARGUMENT"
)))]
#[no_mangle]
pub extern "C" fn subprog_call_mem_kfunc() -> c_long {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

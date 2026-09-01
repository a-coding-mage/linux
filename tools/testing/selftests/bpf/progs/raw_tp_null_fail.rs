// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Ensure module parameter has PTR_MAYBE_NULL */
#[no_mangle]
#[link_section = "tp_btf/bpf_testmod_test_raw_tp_null_tp"]
// __failure __msg("R1 invalid mem access 'trusted_ptr_or_null_'")
pub unsafe extern "C" fn test_raw_tp_null_bpf_testmod_test_raw_tp_null_arg_1(
    ctx: *mut core::ffi::c_void,
) -> i32 {
    let _ = ctx;
    core::arch::asm!(
        "r1 = *(u64 *)(r1 +0); r1 = *(u64 *)(r1 +0);",
        options(nostack)
    );
    return 0;
}

/* Check NULL marking */
#[no_mangle]
#[link_section = "tp_btf/sched_pi_setprio"]
// __failure __msg("R1 invalid mem access 'trusted_ptr_or_null_'")
pub unsafe extern "C" fn test_raw_tp_null_sched_pi_setprio_arg_2(
    ctx: *mut core::ffi::c_void,
) -> i32 {
    let _ = ctx;
    core::arch::asm!(
        "r1 = *(u64 *)(r1 +8); r1 = *(u64 *)(r1 +0);",
        options(nostack)
    );
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

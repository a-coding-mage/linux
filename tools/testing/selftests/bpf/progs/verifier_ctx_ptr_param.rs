// SPDX-License-Identifier: GPL-2.0
/*
 * Verifier tests for single- and multi-level pointer parameter handling
 * Copyright (c) 2026 CrowdStrike, Inc.
 */

// C includes translated as external dependency intent:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_misc.h"

use core::arch::asm;

#[unsafe(link_section = "fentry/bpf_fentry_test_ppvoid")]
// __description("fentry/void**: void ** inferred as scalar")
// __success __retval(0)
// __log_level(2)
// __msg("R1=ctx() R2=scalar()")
#[unsafe(naked)]
pub unsafe extern "C" fn fentry_ppvoid_as_scalar() {
    unsafe {
        asm!(
            "r2 = *(u64 *)(r1 + 0);",
            "r0 = 0;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "fentry/bpf_fentry_test_pppvoid")]
// __description("fentry/void***: void *** inferred as scalar")
// __success __retval(0)
// __log_level(2)
// __msg("R1=ctx() R2=scalar()")
#[unsafe(naked)]
pub unsafe extern "C" fn fentry_pppvoid_as_scalar() {
    unsafe {
        asm!(
            "r2 = *(u64 *)(r1 + 0);",
            "r0 = 0;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "fentry/bpf_fentry_test_ppfile")]
// __description("fentry/struct file**: struct file ** inferred as scalar")
// __success __retval(0)
// __log_level(2)
// __msg("R1=ctx() R2=scalar()")
#[unsafe(naked)]
pub unsafe extern "C" fn fentry_ppfile_as_scalar() {
    unsafe {
        asm!(
            "r2 = *(u64 *)(r1 + 0);",
            "r0 = 0;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "fexit/bpf_fexit_test_ret_ppfile")]
// __description("fexit/return struct file**: returned struct file ** inferred as scalar")
// __success __retval(0)
// __log_level(2)
// __msg("R1=ctx() R2=scalar()")
#[unsafe(naked)]
pub unsafe extern "C" fn fexit_ppfile_as_scalar() {
    unsafe {
        asm!(
            "r2 = *(u64 *)(r1 + 0);",
            "r0 = 0;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

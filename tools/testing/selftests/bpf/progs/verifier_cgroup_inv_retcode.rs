// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/cgroup_inv_retcode.c */

use core::arch::asm;

// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"
// SEC, __description, __success, __failure, __msg, __naked, and __clobber_all
// are provided by the BPF selftest harness in the original C environment.

#[unsafe(link_section = "cgroup/sock")]
#[unsafe(no_mangle)]
// __description("bpf_exit with invalid return code. test1")
// __failure __msg("smin=0 smax=4294967295 should have been in [0, 1]")
pub unsafe extern "C" fn with_invalid_return_code_test1() {
    unsafe {
        asm!(
            "r0 = *(u32*)(r1 + 0);",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "cgroup/sock")]
#[unsafe(no_mangle)]
// __description("bpf_exit with invalid return code. test2")
// __success
pub unsafe extern "C" fn with_invalid_return_code_test2() {
    unsafe {
        asm!(
            "r0 = *(u32*)(r1 + 0);",
            "r0 &= 1;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "cgroup/sock")]
#[unsafe(no_mangle)]
// __description("bpf_exit with invalid return code. test3")
// __failure __msg("smin=0 smax=3 should have been in [0, 1]")
pub unsafe extern "C" fn with_invalid_return_code_test3() {
    unsafe {
        asm!(
            "r0 = *(u32*)(r1 + 0);",
            "r0 &= 3;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "cgroup/sock")]
#[unsafe(no_mangle)]
// __description("bpf_exit with invalid return code. test4")
// __success
pub unsafe extern "C" fn with_invalid_return_code_test4() {
    unsafe {
        asm!(
            "r0 = 1;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "cgroup/sock")]
#[unsafe(no_mangle)]
// __description("bpf_exit with invalid return code. test5")
// __failure __msg("smin=2 smax=2 should have been in [0, 1]")
pub unsafe extern "C" fn with_invalid_return_code_test5() {
    unsafe {
        asm!(
            "r0 = 2;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "cgroup/sock")]
#[unsafe(no_mangle)]
// __description("bpf_exit with invalid return code. test6")
// __failure __msg("R0 is not a known value (ctx)")
pub unsafe extern "C" fn with_invalid_return_code_test6() {
    unsafe {
        asm!(
            "r0 = r1;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "cgroup/sock")]
#[unsafe(no_mangle)]
// __description("bpf_exit with invalid return code. test7")
// __failure __msg("R0 has unknown scalar value should have been in [0, 1]")
pub unsafe extern "C" fn with_invalid_return_code_test7() {
    unsafe {
        asm!(
            "r0 = *(u32*)(r1 + 0);",
            "r2 = *(u32*)(r1 + 4);",
            "r0 *= r2;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

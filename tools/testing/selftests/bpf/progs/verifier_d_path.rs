// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/d_path.c */

// Dependencies in the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

unsafe extern "C" {
    fn bpf_d_path() -> i64;
}

// SEC("fentry/dentry_open")
// __description("d_path accept")
// __success __retval(0)
// __naked
#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/dentry_open")]
pub unsafe extern "C" fn d_path_accept() {
    core::arch::asm!(
        "r1 = *(u64 *)(r1 + 0);",
        "r2 = r10;",
        "r2 += -8;",
        "r6 = 0;",
        "*(u64*)(r2 + 0) = r6;",
        "r3 = 8 ll;",
        "call {bpf_d_path};",
        "r0 = 0;",
        "exit;",
        bpf_d_path = sym bpf_d_path,
        options(noreturn)
    );
}

// SEC("fentry/d_path")
// __description("d_path reject")
// __failure __msg("helper call is not allowed in probe")
// __naked
#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/d_path")]
pub unsafe extern "C" fn d_path_reject() {
    core::arch::asm!(
        "r1 = *(u64 *)(r1 + 0);",
        "r2 = r10;",
        "r2 += -8;",
        "r6 = 0;",
        "*(u64*)(r2 + 0) = r6;",
        "r3 = 8 ll;",
        "call {bpf_d_path};",
        "r0 = 0;",
        "exit;",
        bpf_d_path = sym bpf_d_path,
        options(noreturn)
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

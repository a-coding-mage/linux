// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/btf_ctx_access.c */

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::arch::asm;

// SEC("fentry/bpf_modify_return_test")
// __description("btf_ctx_access accept")
// __success __retval(0)
#[no_mangle]
#[link_section = "fentry/bpf_modify_return_test"]
pub unsafe extern "C" fn btf_ctx_access_accept() {
    asm!(
        "r2 = *(u64 *)(r1 + 8);		/* load 2nd argument value (int pointer) */",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

// SEC("fentry/bpf_fentry_test9")
// __description("btf_ctx_access u32 pointer accept")
// __success __retval(0)
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test9"]
pub unsafe extern "C" fn ctx_access_u32_pointer_accept() {
    asm!(
        "r2 = *(u64 *)(r1 + 0);		/* load 1nd argument value (u32 pointer) */",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

// SEC("fentry/bpf_fentry_test9")
// __description("btf_ctx_access u32 pointer reject u32")
// __failure __msg("size 4 must be 8")
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test9"]
pub unsafe extern "C" fn ctx_access_u32_pointer_reject_32() {
    asm!(
        "r2 = *(u32 *)(r1 + 0);		/* load 1st argument with narrow load */",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

// SEC("fentry/bpf_fentry_test9")
// __description("btf_ctx_access u32 pointer reject u16")
// __failure __msg("size 2 must be 8")
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test9"]
pub unsafe extern "C" fn ctx_access_u32_pointer_reject_16() {
    asm!(
        "r2 = *(u16 *)(r1 + 0);		/* load 1st argument with narrow load */",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

// SEC("fentry/bpf_fentry_test9")
// __description("btf_ctx_access u32 pointer reject u8")
// __failure __msg("size 1 must be 8")
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test9"]
pub unsafe extern "C" fn ctx_access_u32_pointer_reject_8() {
    asm!(
        "r2 = *(u8 *)(r1 + 0);		/* load 1st argument with narrow load */",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

// SEC("fentry/bpf_fentry_test10")
// __description("btf_ctx_access const void pointer accept")
// __success __retval(0)
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test10"]
pub unsafe extern "C" fn ctx_access_const_void_pointer_accept() {
    asm!(
        "r2 = *(u64 *)(r1 + 0);		/* load 1st argument value (const void pointer) */",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

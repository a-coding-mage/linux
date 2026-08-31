// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/*
 * Dependencies from the original C source:
 * - <vmlinux.h>
 * - <bpf/bpf_helpers.h>
 * - ../test_kmods/bpf_testmod_kfunc.h
 * - bpf_misc.h
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct prog_test_big_arg {
    pub a: i32,
    pub b: i32,
}

extern "C" {
    fn bpf_kfunc_call_stack_arg_big(
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: prog_test_big_arg,
    ) -> i32;
}

/*
 * Original condition:
 * #if defined(__BPF_FEATURE_STACK_ARGUMENT)
 */

#[no_mangle]
#[link_section = "tc"]
/* __failure __msg("Unrecognized *(R11-8) type STRUCT") */
pub unsafe extern "C" fn test_stack_arg_big(skb: *mut __sk_buff) -> i32 {
    let s: prog_test_big_arg = prog_test_big_arg { a: 1, b: 2 };

    bpf_kfunc_call_stack_arg_big(1, 2, 3, 4, 5, s)
}

#[no_mangle]
#[link_section = "socket"]
/* __description("r11 in ALU instruction") */
/* __failure __msg("R11 is invalid") */
pub unsafe extern "C" fn r11_alu_reject() {
    asm!(
        "r11 += 1;",
        "r0 = 0;",
        "exit;",
        options(noreturn),
    );
}

#[no_mangle]
#[link_section = "socket"]
/* __description("r11 store with non-DW size") */
/* __failure __msg("R11 is invalid") */
pub unsafe extern "C" fn r11_store_non_dw() {
    asm!(
        "*(u32 *)(r11 - 8) = r1;",
        "r0 = 0;",
        "exit;",
        options(noreturn),
    );
}

#[no_mangle]
#[link_section = "socket"]
/* __description("r11 store with unaligned offset") */
/* __failure __msg("R11 is invalid") */
pub unsafe extern "C" fn r11_store_unaligned() {
    asm!(
        "*(u64 *)(r11 - 4) = r1;",
        "r0 = 0;",
        "exit;",
        options(noreturn),
    );
}

#[no_mangle]
#[link_section = "socket"]
/* __description("r11 store with positive offset") */
/* __failure __msg("R11 is invalid") */
pub unsafe extern "C" fn r11_store_positive_off() {
    asm!(
        "*(u64 *)(r11 + 8) = r1;",
        "r0 = 0;",
        "exit;",
        options(noreturn),
    );
}

#[no_mangle]
#[link_section = "socket"]
/* __description("r11 load with negative offset") */
/* __failure __msg("R11 is invalid") */
pub unsafe extern "C" fn r11_load_negative_off() {
    asm!(
        "r0 = *(u64 *)(r11 - 8);",
        "exit;",
        options(noreturn),
    );
}

#[no_mangle]
#[link_section = "socket"]
/* __description("r11 load with non-DW size") */
/* __failure __msg("R11 is invalid") */
pub unsafe extern "C" fn r11_load_non_dw() {
    asm!(
        "r0 = *(u32 *)(r11 + 8);",
        "exit;",
        options(noreturn),
    );
}

#[no_mangle]
#[link_section = "socket"]
/* __description("r11 store with zero offset") */
/* __failure __msg("R11 is invalid") */
pub unsafe extern "C" fn r11_store_zero_off() {
    asm!(
        "*(u64 *)(r11 + 0) = r1;",
        "r0 = 0;",
        "exit;",
        options(noreturn),
    );
}

/*
 * Original #else branch:
 *
 * SEC("tc")
 * __description("stack_arg_fail: not supported, dummy test")
 * __success
 * int test_stack_arg_big(struct __sk_buff *skb)
 * {
 *     return 0;
 * }
 */

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

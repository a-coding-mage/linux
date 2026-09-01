// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

unsafe extern "C" {
    fn __bpf_trap();
}

// Original C condition: #if __clang_major__ >= 21 && 0
// SEC("socket")
// __description("__builtin_trap with simple c code")
// __failure __msg("unexpected __bpf_trap() due to uninitialized variable?")
// void bpf_builtin_trap_with_simple_c(void)
// {
//     __builtin_trap();
// }
// #endif

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
// __description("__bpf_trap with simple c code")
// __failure __msg("unexpected __bpf_trap() due to uninitialized variable?")
pub unsafe extern "C" fn bpf_trap_with_simple_c() {
    unsafe {
        __bpf_trap();
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
// __description("__bpf_trap as the second-from-last insn")
// __failure __msg("unexpected __bpf_trap() due to uninitialized variable?")
// __naked
pub unsafe extern "C" fn bpf_trap_at_func_end() {
    unsafe {
        core::arch::asm!(
            "r0 = 0;",
            "call {__bpf_trap};",
            "exit;",
            __bpf_trap = sym __bpf_trap,
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
// __description("dead code __bpf_trap in the middle of code")
// __success
// __naked
pub unsafe extern "C" fn dead_bpf_trap_in_middle() {
    unsafe {
        core::arch::asm!(
            "r0 = 0;",
            "if r0 == 0 goto +1;",
            "call {__bpf_trap};",
            "r0 = 2;",
            "exit;",
            __bpf_trap = sym __bpf_trap,
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
// __description("reachable __bpf_trap in the middle of code")
// __failure __msg("unexpected __bpf_trap() due to uninitialized variable?")
// __naked
pub unsafe extern "C" fn live_bpf_trap_in_middle() {
    unsafe {
        core::arch::asm!(
            "r0 = 0;",
            "if r0 == 1 goto +1;",
            "call {__bpf_trap};",
            "r0 = 2;",
            "exit;",
            __bpf_trap = sym __bpf_trap,
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

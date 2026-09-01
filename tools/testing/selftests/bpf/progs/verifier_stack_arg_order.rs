// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C includes translated as dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

// Original C conditional:
// #if (defined(__TARGET_ARCH_x86) || defined(__TARGET_ARCH_arm64)) && \
//     defined(__BPF_FEATURE_STACK_ARGUMENT)
// The BPF stack argument tests below require x86/arm64 target support and
// __BPF_FEATURE_STACK_ARGUMENT.

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
#[used]
#[unsafe(no_mangle)]
pub static subprog_bad_order_6args: unsafe extern "C" fn(
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int = {
    unsafe extern "C" fn subprog_bad_order_6args_impl(
        _a: ::core::ffi::c_int,
        _b: ::core::ffi::c_int,
        _c: ::core::ffi::c_int,
        _d: ::core::ffi::c_int,
        _e: ::core::ffi::c_int,
        _f: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            ::core::arch::asm!(
                "*(u64 *)(r11 - 8) = r1;",
                "r0 = *(u64 *)(r11 + 8);",
                "exit;",
                options(noreturn)
            );
        }
    }
    subprog_bad_order_6args_impl
};

// SEC("tc")
// __description("stack_arg: r11 load after r11 store")
// __failure
// __msg("r11 load must be before any r11 store or call insn")
// __btf_func_path("btf__verifier_stack_arg_order.bpf.o")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_arg_load_after_store() {
    unsafe {
        ::core::arch::asm!(
            "r1 = 1;",
            "r2 = 2;",
            "r3 = 3;",
            "r4 = 4;",
            "r5 = 5;",
            "*(u64 *)(r11 - 8) = 6;",
            "call subprog_bad_order_6args;",
            "exit;",
            options(noreturn)
        );
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
#[used]
#[unsafe(no_mangle)]
pub static subprog_call_before_load_6args: unsafe extern "C" fn(
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int = {
    unsafe extern "C" fn subprog_call_before_load_6args_impl(
        _a: ::core::ffi::c_int,
        _b: ::core::ffi::c_int,
        _c: ::core::ffi::c_int,
        _d: ::core::ffi::c_int,
        _e: ::core::ffi::c_int,
        _f: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            ::core::arch::asm!(
                "call {bpf_get_prandom_u32};",
                "r0 = *(u64 *)(r11 + 8);",
                "exit;",
                bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
                options(noreturn)
            );
        }
    }
    subprog_call_before_load_6args_impl
};

// SEC("tc")
// __description("stack_arg: r11 load after a call")
// __failure
// __msg("r11 load must be before any r11 store or call insn")
// __btf_func_path("btf__verifier_stack_arg_order.bpf.o")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_arg_load_after_call() {
    unsafe {
        ::core::arch::asm!(
            "r1 = 1;",
            "r2 = 2;",
            "r3 = 3;",
            "r4 = 4;",
            "r5 = 5;",
            "*(u64 *)(r11 - 8) = 6;",
            "call subprog_call_before_load_6args;",
            "exit;",
            options(noreturn)
        );
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
#[used]
#[unsafe(no_mangle)]
pub static subprog_pruning_call_before_load_6args: unsafe extern "C" fn(
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) -> ::core::ffi::c_int = {
    unsafe extern "C" fn subprog_pruning_call_before_load_6args_impl(
        _a: ::core::ffi::c_int,
        _b: ::core::ffi::c_int,
        _c: ::core::ffi::c_int,
        _d: ::core::ffi::c_int,
        _e: ::core::ffi::c_int,
        _f: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            ::core::arch::asm!(
                "if r1 s> 0 goto 0f;",
                "goto 1f;",
                "0:",
                "call {bpf_get_prandom_u32};",
                "1:",
                "r0 = *(u64 *)(r11 + 8);",
                "exit;",
                bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
                options(noreturn)
            );
        }
    }
    subprog_pruning_call_before_load_6args_impl
};

// SEC("tc")
// __description("stack_arg: pruning keeps r11 load ordering")
// __failure
// __flag(BPF_F_TEST_STATE_FREQ)
// __msg("r11 load must be before any r11 store or call insn")
// __btf_func_path("btf__verifier_stack_arg_order.bpf.o")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_arg_pruning_load_after_call() {
    unsafe {
        ::core::arch::asm!(
            "call {bpf_get_prandom_u32};",
            "r1 = r0;",
            "r2 = 2;",
            "r3 = 3;",
            "r4 = 4;",
            "r5 = 5;",
            "*(u64 *)(r11 - 8) = 6;",
            "call subprog_pruning_call_before_load_6args;",
            "exit;",
            bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
            options(noreturn)
        );
    }
}

/*
 * "bad_ptr": the first arg is 'long *', which is not a recognized pointer
 * type for static subprogs (not ctx, dynptr, or tagged).  btf_prepare_func_args()
 * sets arg_cnt = 7 / stack_arg_cnt = 2, then fails with -EINVAL.  The subprog
 * is marked unreliable but the call still proceeds for static subprogs.
 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
#[used]
#[unsafe(no_mangle)]
pub static subprog_bad_ptr_7args: unsafe extern "C" fn(
    *mut ::core::ffi::c_long,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) = {
    unsafe extern "C" fn subprog_bad_ptr_7args_impl(
        _a: *mut ::core::ffi::c_long,
        _b: ::core::ffi::c_int,
        _c: ::core::ffi::c_int,
        _d: ::core::ffi::c_int,
        _e: ::core::ffi::c_int,
        _f: ::core::ffi::c_int,
        _g: ::core::ffi::c_int,
    ) {
        unsafe {
            ::core::arch::asm!(
                "r0 = *(u64 *)(r11 + 8);",
                "r1 = *(u64 *)(r11 + 16);",
                "exit;",
                options(noreturn)
            );
        }
    }
    subprog_bad_ptr_7args_impl
};

// SEC("tc")
// __description("stack_arg: read without caller write")
// __failure
// __msg("callee expects 7 args, stack arg1 is not initialized")
// __btf_func_path("btf__verifier_stack_arg_order.bpf.o")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_arg_read_without_write_1() {
    unsafe {
        ::core::arch::asm!(
            "r1 = 0;",
            "r2 = 0;",
            "r3 = 0;",
            "r4 = 0;",
            "r5 = 0;",
            "call subprog_bad_ptr_7args;",
            "exit;",
            options(noreturn)
        );
    }
}

// SEC("tc")
// __description("stack_arg: read with not-initialized caller write")
// __failure
// __msg("R0 !read_ok")
// __btf_func_path("btf__verifier_stack_arg_order.bpf.o")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_arg_read_without_write_2() {
    unsafe {
        ::core::arch::asm!(
            "r1 = 0;",
            "r2 = 0;",
            "r3 = 0;",
            "r4 = 0;",
            "r5 = 0;",
            "*(u64 *)(r11 - 8) = 0;",
            "*(u64 *)(r11 - 16) = 0;",
            "call subprog_bad_ptr_7args;",
            "call subprog_bad_ptr_7args;",
            "exit;",
            options(noreturn)
        );
    }
}

// #else

// SEC("socket")
// __description("stack_arg order is not supported by compiler or jit, use a dummy test")
// __success
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
pub extern "C" fn dummy_test() -> ::core::ffi::c_int {
    0
}

// #endif

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

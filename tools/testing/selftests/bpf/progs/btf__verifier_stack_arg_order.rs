// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependency intent: <vmlinux.h> and <bpf/bpf_helpers.h>.

// Original C condition:
// (defined(__TARGET_ARCH_x86) || defined(__TARGET_ARCH_arm64)) &&
// defined(__BPF_FEATURE_STACK_ARGUMENT)
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub extern "C" fn subprog_bad_order_6args(
    a: ::core::ffi::c_int,
    b: ::core::ffi::c_int,
    c: ::core::ffi::c_int,
    d: ::core::ffi::c_int,
    e: ::core::ffi::c_int,
    f: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    a + b + c + d + e + f
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub extern "C" fn subprog_call_before_load_6args(
    a: ::core::ffi::c_int,
    b: ::core::ffi::c_int,
    c: ::core::ffi::c_int,
    d: ::core::ffi::c_int,
    e: ::core::ffi::c_int,
    f: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    a + b + c + d + e + f
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub extern "C" fn subprog_pruning_call_before_load_6args(
    a: ::core::ffi::c_int,
    b: ::core::ffi::c_int,
    c: ::core::ffi::c_int,
    d: ::core::ffi::c_int,
    e: ::core::ffi::c_int,
    f: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    a + b + c + d + e + f
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
))]
#[no_mangle]
pub extern "C" fn subprog_bad_ptr_7args(
    a: *mut ::core::ffi::c_long,
    b: ::core::ffi::c_int,
    c: ::core::ffi::c_int,
    d: ::core::ffi::c_int,
    e: ::core::ffi::c_int,
    f: ::core::ffi::c_int,
    g: ::core::ffi::c_int,
) {
    let _ = (a, b, c, d, e, f, g);
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
)))]
#[no_mangle]
pub extern "C" fn subprog_bad_order_6args() -> ::core::ffi::c_int {
    0
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
)))]
#[no_mangle]
pub extern "C" fn subprog_call_before_load_6args() -> ::core::ffi::c_int {
    0
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
)))]
#[no_mangle]
pub extern "C" fn subprog_pruning_call_before_load_6args() -> ::core::ffi::c_int {
    0
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    feature = "bpf_feature_stack_argument"
)))]
#[no_mangle]
pub extern "C" fn subprog_bad_ptr_7args() {}

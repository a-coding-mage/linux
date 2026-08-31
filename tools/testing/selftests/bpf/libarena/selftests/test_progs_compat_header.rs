// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C source conditional: #ifdef __BPF__

/* Selftests use these tags for compatibility with test_progs. */
// C macro:
// #define __test_tag(tag) __attribute__((btf_decl_tag("comment:" XSTR(__COUNTER__) ":" tag)))
// Rust has no file-local equivalent for C's __COUNTER__ inside an attribute macro
// without defining a procedural macro. Preserve the attribute payload intent for
// call sites through declarative macros.
#[cfg(target_arch = "bpf")]
macro_rules! __test_tag {
    ($tag:expr) => {
        concat!("comment:", stringify!(__COUNTER__), ":", $tag)
    };
}

#[cfg(target_arch = "bpf")]
macro_rules! __stderr {
    ($msg:expr) => {
        __test_tag!(concat!("test_expect_stderr=", $msg))
    };
}

#[cfg(target_arch = "bpf")]
macro_rules! __stderr_unpriv {
    ($msg:expr) => {
        __test_tag!(concat!("test_expect_stderr_unpriv=", $msg))
    };
}

#[cfg(target_arch = "bpf")]
macro_rules! XSTR {
    ($s:tt) => {
        STR!($s)
    };
}

#[cfg(target_arch = "bpf")]
macro_rules! STR {
    ($s:tt) => {
        stringify!($s)
    };
}

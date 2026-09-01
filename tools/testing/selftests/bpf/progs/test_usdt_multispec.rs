// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// Rust translation of dependencies originally provided by:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/usdt.bpf.h>

/* this file is linked together with test_usdt.c to validate that usdt.bpf.h
 * can be included in multiple .bpf.c files forming single final BPF object
 * file
 */

extern "C" {
    static mut my_pid: ::core::ffi::c_int;

    fn bpf_get_current_pid_tgid() -> u64;
}

#[no_mangle]
pub static mut usdt_100_called: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut usdt_100_sum: ::core::ffi::c_int = 0;

#[no_mangle]
#[link_section = "usdt//proc/self/exe:test:usdt_100"]
pub unsafe extern "C" fn usdt_100(x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if my_pid != (bpf_get_current_pid_tgid() >> 32) as ::core::ffi::c_int {
        return 0;
    }

    core::intrinsics::atomic_xadd_seqcst(&mut usdt_100_called, 1);
    core::intrinsics::atomic_xadd_seqcst(&mut usdt_100_sum, x);

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

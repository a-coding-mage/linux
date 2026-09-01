// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// Dependencies in the C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
#[unsafe(link_section = "?fentry/bpf_spin_lock")]
pub unsafe extern "C" fn test_spin_lock(lock: *mut bpf_spin_lock) -> ::std::os::raw::c_int {
    let _ = lock;
    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?fentry/bpf_spin_unlock")]
pub unsafe extern "C" fn test_spin_unlock(lock: *mut bpf_spin_lock) -> ::std::os::raw::c_int {
    let _ = lock;
    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?fentry/__rcu_read_lock")]
pub unsafe extern "C" fn tracing_deny() -> ::std::os::raw::c_int {
    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?fexit/do_exit")]
pub unsafe extern "C" fn fexit_noreturns() -> ::std::os::raw::c_int {
    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?fexit/bpf_testmod_test_int128_ret")]
pub unsafe extern "C" fn fexit_int128_ret() -> ::std::os::raw::c_int {
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

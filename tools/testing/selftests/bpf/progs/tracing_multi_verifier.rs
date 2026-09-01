// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_misc.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "fentry.multi/bpf_fentry_test1"]
// __failure
// __msg("func 'bpf_multi_func' doesn't have 1-th argument")
pub extern "C" fn fentry_direct_access(a: i32) -> i32 {
    return a;
}

#[no_mangle]
#[link_section = "fexit.multi/bpf_fentry_test3"]
// __failure
// __msg("invalid bpf_context access off=24 size=8")
pub extern "C" fn fexit_direct_access(a: i8, b: i32, c: u64, ret: i32) -> i32 {
    return ret;
}

#[no_mangle]
#[link_section = "fsession.multi/bpf_fentry_test4"]
// __failure
// __msg("invalid bpf_context access off=16 size=8")
pub extern "C" fn fsession_direct_access(
    a: *mut core::ffi::c_void,
    b: i8,
    c: i32,
    d: u64,
    ret: i32,
) -> i32 {
    return c;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0
// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod_kfunc.h"

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_kfunc_common_test();
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

/*
 * No tests in here, just to trigger 'bpf_fentry_test*'
 * through tracing test_run
 */
#[no_mangle]
#[link_section = "fentry/bpf_modify_return_test"]
pub unsafe extern "C" fn trigger() -> i32 {
    return 0;
}

#[no_mangle]
#[link_section = "kprobe/bpf_fentry_test1"]
pub unsafe extern "C" fn test1(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    bpf_kfunc_common_test();
    return 0;
}

#[no_mangle]
#[link_section = "kprobe/bpf_kfunc_common_test"]
pub unsafe extern "C" fn test2(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    return 0;
}

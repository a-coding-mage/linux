// SPDX-License-Identifier: GPL-2.0
// Dependencies from C includes:
// - "vmlinux.h"
// - <bpf/bpf_helpers.h>
// - <bpf/bpf_tracing.h>
// - "../test_kmods/bpf_testmod_kfunc.h"

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_kfunc_common_test();
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

/*
 * No tests in here, just to trigger 'bpf_fentry_test*'
 * through tracing test_run
 */
#[unsafe(link_section = "fentry/bpf_modify_return_test")]
#[unsafe(no_mangle)]
pub extern "C" fn trigger() -> i32 {
    return 0;
}

#[unsafe(link_section = "kprobe.multi/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test1(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_kfunc_common_test();
    }
    return 0;
}

#[unsafe(link_section = "kprobe/bpf_kfunc_common_test")]
#[unsafe(no_mangle)]
pub extern "C" fn test2(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "kprobe/bpf_kfunc_common_test")]
#[unsafe(no_mangle)]
pub extern "C" fn test3(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "kprobe/bpf_kfunc_common_test")]
#[unsafe(no_mangle)]
pub extern "C" fn test4(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "kprobe.multi/bpf_kfunc_common_test")]
#[unsafe(no_mangle)]
pub extern "C" fn test5(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "kprobe.session/bpf_kfunc_common_test")]
#[unsafe(no_mangle)]
pub extern "C" fn test6(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

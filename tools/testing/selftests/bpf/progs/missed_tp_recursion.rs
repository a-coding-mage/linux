// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn bpf_printk(fmt: *const core::ffi::c_char, ...) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as _, b'P' as _, b'L' as _, 0];

/*
 * No tests in here, just to trigger 'bpf_fentry_test*'
 * through tracing test_run
 */
#[unsafe(link_section = "fentry/bpf_modify_return_test")]
#[unsafe(no_mangle)]
pub extern "C" fn trigger() -> i32 {
    return 0;
}

#[unsafe(link_section = "kprobe/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test1(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_printk(c"test".as_ptr());
    }
    return 0;
}

#[unsafe(link_section = "tp/bpf_trace/bpf_trace_printk")]
#[unsafe(no_mangle)]
pub extern "C" fn test2(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "tp/bpf_trace/bpf_trace_printk")]
#[unsafe(no_mangle)]
pub extern "C" fn test3(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "tp/bpf_trace/bpf_trace_printk")]
#[unsafe(no_mangle)]
pub extern "C" fn test4(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

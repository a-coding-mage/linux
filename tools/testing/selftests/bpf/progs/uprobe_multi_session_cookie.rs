// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source:
// includes: <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, <stdbool.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u64 = u64;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_session_cookie(ctx: *mut pt_regs) -> *mut __u64;
    fn bpf_session_is_return(ctx: *mut pt_regs) -> bool;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

#[unsafe(no_mangle)]
pub static mut test_uprobe_1_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut test_uprobe_2_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut test_uprobe_3_result: __u64 = 0;

unsafe fn check_cookie(ctx: *mut pt_regs, val: __u64, result: *mut __u64) -> i32 {
    let cookie: *mut __u64;

    if (unsafe { bpf_get_current_pid_tgid() } >> 32) != unsafe { pid } as __u64 {
        return 1;
    }

    cookie = unsafe { bpf_session_cookie(ctx) };

    if unsafe { bpf_session_is_return(ctx) } {
        unsafe {
            *result = if *cookie == val { val } else { 0 };
        }
    } else {
        unsafe {
            *cookie = val;
        }
    }
    0
}

// SEC("uprobe.session//proc/self/exe:uprobe_multi_func_1")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_1(ctx: *mut pt_regs) -> i32 {
    unsafe { check_cookie(ctx, 1, &raw mut test_uprobe_1_result) }
}

// SEC("uprobe.session//proc/self/exe:uprobe_multi_func_2")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_2(ctx: *mut pt_regs) -> i32 {
    unsafe { check_cookie(ctx, 2, &raw mut test_uprobe_2_result) }
}

// SEC("uprobe.session//proc/self/exe:uprobe_multi_func_3")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_3(ctx: *mut pt_regs) -> i32 {
    unsafe { check_cookie(ctx, 3, &raw mut test_uprobe_3_result) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

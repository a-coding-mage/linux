// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u64 = u64;

unsafe extern "C" {
    #[link_name = "tracing_multi_arg_check"]
    fn tracing_multi_arg_check(ctx: *mut __u64, test_result: *mut __u64, is_return: bool) -> i32;

    fn bpf_session_cookie(ctx: *mut __u64) -> *mut __u64;
    fn bpf_session_is_return(ctx: *mut __u64) -> bool;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut test_result_fentry: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut test_result_fexit: __u64 = 0;

#[unsafe(link_section = "fsession.multi/bpf_fentry_test*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_session_1(ctx: *mut __u64) -> i32 {
    let cookie: *mut __u64 = unsafe { bpf_session_cookie(ctx) };

    if unsafe { bpf_session_is_return(ctx) } {
        if unsafe { tracing_multi_arg_check(ctx, &raw mut test_result_fexit, true) } != 0 {
            return 0;
        }
        /* extra count for test_result_fexit cookie */
        unsafe {
            test_result_fexit = test_result_fexit.wrapping_add(
                (core::ptr::read_volatile(cookie) == 0xbeafbeafbeafbeaf) as __u64,
            );
        }
    } else {
        if unsafe { tracing_multi_arg_check(ctx, &raw mut test_result_fentry, false) } != 0 {
            return 0;
        }
        unsafe {
            core::ptr::write_volatile(cookie, 0xbeafbeafbeafbeaf);
        }
    }
    0
}

#[unsafe(link_section = "fsession.multi.s/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_fsession_s(ctx: *mut __u64) -> i32 {
    let cookie: *mut __u64 = unsafe { bpf_session_cookie(ctx) };

    if unsafe { bpf_session_is_return(ctx) } {
        if unsafe { tracing_multi_arg_check(ctx, &raw mut test_result_fexit, true) } != 0 {
            return 0;
        }
        /* extra count for test_result_fexit cookie */
        unsafe {
            test_result_fexit = test_result_fexit.wrapping_add(
                (core::ptr::read_volatile(cookie) == 0xbeafbeafbeafbeaf) as __u64,
            );
        }
    } else {
        if unsafe { tracing_multi_arg_check(ctx, &raw mut test_result_fentry, false) } != 0 {
            return 0;
        }
        unsafe {
            core::ptr::write_volatile(cookie, 0xbeafbeafbeafbeaf);
        }
    }
    0
}

#[unsafe(link_section = "fsession.multi/bpf_testmod:bpf_testmod_fentry_test*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_session_2(ctx: *mut __u64) -> i32 {
    let cookie: *mut __u64 = unsafe { bpf_session_cookie(ctx) };

    if unsafe { bpf_session_is_return(ctx) } {
        if unsafe { tracing_multi_arg_check(ctx, &raw mut test_result_fexit, true) } != 0 {
            return 0;
        }
        /* extra count for test_result_fexit cookie */
        unsafe {
            test_result_fexit = test_result_fexit.wrapping_add(
                (core::ptr::read_volatile(cookie) == 0xbeafbeafbeafbeaf) as __u64,
            );
        }
    } else {
        if unsafe { tracing_multi_arg_check(ctx, &raw mut test_result_fentry, false) } != 0 {
            return 0;
        }
        unsafe {
            core::ptr::write_volatile(cookie, 0xbeafbeafbeafbeaf);
        }
    }
    0
}

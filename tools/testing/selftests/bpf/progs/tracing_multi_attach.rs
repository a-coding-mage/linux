// SPDX-License-Identifier: GPL-2.0
// Translated from C source using vmlinux.h, bpf_helpers.h, and bpf_tracing.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u64 = u64;

unsafe extern "C" {
    #[link_name = "tracing_multi_arg_check"]
    fn tracing_multi_arg_check(ctx: *mut __u64, test_result: *mut __u64, is_return: bool) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut test_result_fentry: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut test_result_fexit: __u64 = 0;

#[unsafe(link_section = "fentry.multi/bpf_fentry_test*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_fentry(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(ctx, &raw mut test_result_fentry, false);
    }
    0
}

#[unsafe(link_section = "fexit.multi/bpf_fentry_test*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_fexit(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(ctx, &raw mut test_result_fexit, true);
    }
    0
}

#[unsafe(link_section = "fentry.multi.s/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_fentry_s(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(ctx, &raw mut test_result_fentry, false);
    }
    0
}

#[unsafe(link_section = "fexit.multi.s/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_fexit_s(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(ctx, &raw mut test_result_fexit, true);
    }
    0
}

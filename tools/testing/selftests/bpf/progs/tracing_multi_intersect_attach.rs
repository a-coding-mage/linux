// SPDX-License-Identifier: GPL-2.0
//
// C dependencies removed from executable Rust:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]

pub type __u64 = u64;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    #[link_name = "tracing_multi_arg_check"]
    fn tracing_multi_arg_check(ctx: *mut __u64, test_result: *mut __u64, is_return: bool) -> i32;
}

#[unsafe(no_mangle)]
pub static mut test_result_fentry_1: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut test_result_fentry_2: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut test_result_fexit_1: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut test_result_fexit_2: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut test_result_fentry: __u64 = 0;

#[unsafe(link_section = "fentry/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fentry(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(
            ctx,
            core::ptr::addr_of_mut!(test_result_fentry),
            false,
        );
    }
    0
}

#[unsafe(link_section = "fentry.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fentry_1(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(
            ctx,
            core::ptr::addr_of_mut!(test_result_fentry_1),
            false,
        );
    }
    0
}

#[unsafe(link_section = "fentry.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fentry_2(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(
            ctx,
            core::ptr::addr_of_mut!(test_result_fentry_2),
            false,
        );
    }
    0
}

#[unsafe(link_section = "fexit.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fexit_1(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(
            ctx,
            core::ptr::addr_of_mut!(test_result_fexit_1),
            true,
        );
    }
    0
}

#[unsafe(link_section = "fexit.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fexit_2(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(
            ctx,
            core::ptr::addr_of_mut!(test_result_fexit_2),
            true,
        );
    }
    0
}

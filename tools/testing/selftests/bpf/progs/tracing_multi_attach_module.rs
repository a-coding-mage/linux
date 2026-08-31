// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]

type __u64 = u64;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    #[link_name = "tracing_multi_arg_check"]
    fn tracing_multi_arg_check(ctx: *mut __u64, test_result: *mut __u64, is_return: bool) -> i32;
}

#[no_mangle]
pub static mut test_result_fentry: __u64 = 0;

#[no_mangle]
pub static mut test_result_fexit: __u64 = 0;

#[no_mangle]
#[link_section = "fentry.multi/bpf_testmod:bpf_testmod_fentry_test*"]
pub unsafe extern "C" fn test_fentry(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(ctx, &raw mut test_result_fentry, false);
    }
    0
}

#[no_mangle]
#[link_section = "fexit.multi/bpf_testmod:bpf_testmod_fentry_test*"]
pub unsafe extern "C" fn test_fexit(ctx: *mut __u64) -> i32 {
    unsafe {
        tracing_multi_arg_check(ctx, &raw mut test_result_fexit, true);
    }
    0
}

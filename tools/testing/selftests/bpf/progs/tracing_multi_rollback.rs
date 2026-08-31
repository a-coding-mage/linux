// SPDX-License-Identifier: GPL-2.0
// Translated from C:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_upper_case_globals)]

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

#[unsafe(no_mangle)]
pub static mut test_result_fentry: u64 = 0;
#[unsafe(no_mangle)]
pub static mut test_result_fexit: u64 = 0;

#[unsafe(link_section = "?fentry.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_fentry() -> i32 {
    if (unsafe { bpf_get_current_pid_tgid() } >> 32) != unsafe { pid as u64 } {
        return 0;
    }

    unsafe {
        test_result_fentry = test_result_fentry.wrapping_add(1);
    }
    0
}

#[unsafe(link_section = "?fexit.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_fexit() -> i32 {
    if (unsafe { bpf_get_current_pid_tgid() } >> 32) != unsafe { pid as u64 } {
        return 0;
    }

    unsafe {
        test_result_fexit = test_result_fexit.wrapping_add(1);
    }
    0
}

#[unsafe(link_section = "?fentry/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn extra() -> i32 {
    0
}

#[unsafe(link_section = "?fentry/bpf_fentry_test10")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filler() -> i32 {
    0
}

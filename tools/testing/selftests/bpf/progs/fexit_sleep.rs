// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut fentry_cnt: i32 = 0;
#[unsafe(no_mangle)]
pub static mut fexit_cnt: i32 = 0;

// SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nanosleep_fentry(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    if (unsafe { bpf_get_current_pid_tgid() } >> 32) != unsafe { pid } as u64 {
        return 0;
    }

    unsafe {
        fentry_cnt += 1;
    }
    0
}

// SEC("fexit/" SYS_PREFIX "sys_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nanosleep_fexit(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    if (unsafe { bpf_get_current_pid_tgid() } >> 32) != unsafe { pid } as u64 {
        return 0;
    }

    unsafe {
        fexit_cnt += 1;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

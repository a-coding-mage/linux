// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

pub static mut fentry_hit: i32 = 0;
pub static mut fexit_hit: i32 = 0;
pub static mut my_pid: i32 = 0;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[link_section = "fentry/cmdline_proc_show"]
#[no_mangle]
pub unsafe extern "C" fn fentry_cmdline() -> i32 {
    if my_pid != (bpf_get_current_pid_tgid() >> 32) as i32 {
        return 0;
    }

    fentry_hit = 1;
    return 0;
}

#[link_section = "fexit/cmdline_proc_show"]
#[no_mangle]
pub unsafe extern "C" fn fexit_cmdline() -> i32 {
    if my_pid != (bpf_get_current_pid_tgid() >> 32) as i32 {
        return 0;
    }

    fexit_hit = 1;
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

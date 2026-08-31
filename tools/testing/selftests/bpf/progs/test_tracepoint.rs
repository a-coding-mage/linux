// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017 Facebook

// C dependencies removed from executable Rust:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

extern "C" {
    static TASK_COMM_LEN: usize;
}

/* taken from /sys/kernel/tracing/events/sched/sched_switch/format */
#[repr(C)]
pub struct sched_switch_args {
    pub pad: ::core::ffi::c_ulonglong,
    pub prev_comm: [::core::ffi::c_char; TASK_COMM_LEN],
    pub prev_pid: ::core::ffi::c_int,
    pub prev_prio: ::core::ffi::c_int,
    pub prev_state: ::core::ffi::c_longlong,
    pub next_comm: [::core::ffi::c_char; TASK_COMM_LEN],
    pub next_pid: ::core::ffi::c_int,
    pub next_prio: ::core::ffi::c_int,
}

#[no_mangle]
#[link_section = "tracepoint/sched/sched_switch"]
pub unsafe extern "C" fn oncpu(ctx: *mut sched_switch_args) -> ::core::ffi::c_int {
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];

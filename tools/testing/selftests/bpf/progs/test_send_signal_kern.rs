// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// Dependencies from C includes:
// <vmlinux.h>, <linux/version.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub enum pid_type {
    PIDTYPE_PID = 0,
    PIDTYPE_TGID = 1,
}

pub type u64 = ::core::ffi::c_ulonglong;
pub type __u32 = ::core::ffi::c_uint;

unsafe extern "C" {
    #[link_name = "bpf_task_from_pid"]
    fn bpf_task_from_pid(pid: ::core::ffi::c_int) -> *mut task_struct;
    #[link_name = "bpf_task_release"]
    fn bpf_task_release(p: *mut task_struct);
    #[link_name = "bpf_send_signal_task"]
    fn bpf_send_signal_task(
        task: *mut task_struct,
        sig: ::core::ffi::c_int,
        type_: pid_type,
        value: u64,
    ) -> ::core::ffi::c_int;

    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_send_signal_thread(sig: __u32) -> ::core::ffi::c_int;
    fn bpf_send_signal(sig: __u32) -> ::core::ffi::c_int;
}

#[unsafe(no_mangle)]
pub static mut sig: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut pid: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut status: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut signal_thread: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut target_pid: __u32 = 0;

#[inline(always)]
unsafe fn bpf_send_signal_test(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut target_task: *mut task_struct = ::core::ptr::null_mut();
    let ret: ::core::ffi::c_int;
    let mut value: u64 = 0;

    let _ = ctx;

    if status != 0 || pid == 0 {
        return 0;
    }

    if (bpf_get_current_pid_tgid() >> 32) as __u32 == pid {
        if target_pid != 0 {
            target_task = bpf_task_from_pid(target_pid as ::core::ffi::c_int);
            if target_task.is_null() {
                return 0;
            }
            value = 8;
        }

        if signal_thread != 0 {
            if target_pid != 0 {
                ret = bpf_send_signal_task(target_task, sig as ::core::ffi::c_int, pid_type::PIDTYPE_PID, value);
            } else {
                ret = bpf_send_signal_thread(sig);
            }
        } else {
            if target_pid != 0 {
                ret = bpf_send_signal_task(target_task, sig as ::core::ffi::c_int, pid_type::PIDTYPE_TGID, value);
            } else {
                ret = bpf_send_signal(sig);
            }
        }
        if ret == 0 {
            status = 1;
        }
    }

    if !target_task.is_null() {
        bpf_task_release(target_task);
    }

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tracepoint/syscalls/sys_enter_nanosleep")]
pub unsafe extern "C" fn send_signal_tp(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    bpf_send_signal_test(ctx)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tracepoint/sched/sched_switch")]
pub unsafe extern "C" fn send_signal_tp_sched(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    bpf_send_signal_test(ctx)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "perf_event")]
pub unsafe extern "C" fn send_signal_perf(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    bpf_send_signal_test(ctx)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static __license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

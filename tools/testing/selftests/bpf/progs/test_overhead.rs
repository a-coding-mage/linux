// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_raw_tracepoint_args {
    pub args: [u64; 0],
}

#[unsafe(link_section = "kprobe/__set_task_comm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog1(tsk: *mut task_struct, _buf: *const core::ffi::c_char, _exec: bool) -> i32 {
    tsk.is_null() as i32
}

#[unsafe(link_section = "kretprobe/__set_task_comm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog2(ret: i32) -> i32 {
    ret
}

#[unsafe(link_section = "raw_tp/task_rename")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog3(ctx: *mut bpf_raw_tracepoint_args) -> i32 {
    ((*(*ctx).args.as_ptr().add(0)) == 0) as i32
}

#[unsafe(link_section = "fentry/__set_task_comm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog4(
    _tsk: *mut task_struct,
    _buf: *const core::ffi::c_char,
    _exec: bool,
) -> i32 {
    0
}

#[unsafe(link_section = "fexit/__set_task_comm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog5(
    _tsk: *mut task_struct,
    _buf: *const core::ffi::c_char,
    _exec: bool,
) -> i32 {
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];

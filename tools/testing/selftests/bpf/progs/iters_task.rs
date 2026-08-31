// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Chuyi Zhou <zhouchuyi@bytedance.com> */

/* Dependencies from the original C source:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 * #include "bpf_misc.h"
 * #include "bpf_experimental.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type pid_t = i32;

#[repr(C)]
pub struct task_struct {
    pub pid: pid_t,
    pub tgid: pid_t,
}

pub const BPF_TASK_ITER_PROC_THREADS: u32 = 1;
pub const BPF_TASK_ITER_ALL_PROCS: u32 = 2;
pub const BPF_TASK_ITER_ALL_THREADS: u32 = 3;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut target_pid: pid_t = 0;
#[no_mangle]
pub static mut procs_cnt: i32 = 0;
#[no_mangle]
pub static mut threads_cnt: i32 = 0;
#[no_mangle]
pub static mut proc_threads_cnt: i32 = 0;
#[no_mangle]
pub static mut invalid_cnt: i32 = 0;

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;

    #[link_name = "bpf_rcu_read_lock"]
    fn bpf_rcu_read_lock();
    #[link_name = "bpf_rcu_read_unlock"]
    fn bpf_rcu_read_unlock();
}

/* Rust translation of the BPF task iterator macro usage. The actual iterator
 * facility is provided externally by BPF helper infrastructure, matching the
 * original bpf_for_each(task, pos, task__nullable, flags) macro.
 */
extern "C" {
    fn bpf_for_each_task(
        task__nullable: *mut task_struct,
        flags: u32,
        callback: unsafe extern "C" fn(*mut task_struct, *mut core::ffi::c_void),
        callback_ctx: *mut core::ffi::c_void,
    );
}

unsafe extern "C" fn invalid_cnt_callback(
    _pos: *mut task_struct,
    _callback_ctx: *mut core::ffi::c_void,
) {
    /* Below instructions shouldn't be executed for invalid flags */
    invalid_cnt += 1;
}

unsafe extern "C" fn invalid_task_nullable_callback(
    _pos: *mut task_struct,
    _callback_ctx: *mut core::ffi::c_void,
) {
    /* Below instructions shouldn't be executed for invalid task__nullable */
    invalid_cnt += 1;
}

unsafe extern "C" fn procs_cnt_callback(
    pos: *mut task_struct,
    _callback_ctx: *mut core::ffi::c_void,
) {
    if (*pos).pid == target_pid {
        procs_cnt += 1;
    }
}

unsafe extern "C" fn proc_threads_cnt_callback(
    _pos: *mut task_struct,
    _callback_ctx: *mut core::ffi::c_void,
) {
    proc_threads_cnt += 1;
}

unsafe extern "C" fn threads_cnt_callback(
    pos: *mut task_struct,
    _callback_ctx: *mut core::ffi::c_void,
) {
    if (*pos).tgid == target_pid {
        threads_cnt += 1;
    }
}

#[no_mangle]
#[link_section = concat!("fentry.s/", SYS_PREFIX, "sys_getpgid")]
pub unsafe extern "C" fn iter_task_for_each_sleep(ctx: *mut core::ffi::c_void) -> i32 {
    let cur_task: *mut task_struct = bpf_get_current_task_btf();
    let mut _pos: *mut task_struct;

    if (*cur_task).pid != target_pid {
        return 0;
    }
    proc_threads_cnt = 0;
    threads_cnt = proc_threads_cnt;
    procs_cnt = threads_cnt;

    bpf_rcu_read_lock();
    bpf_for_each_task(
        core::ptr::null_mut(),
        !0u32,
        invalid_cnt_callback,
        core::ptr::null_mut(),
    );

    bpf_for_each_task(
        core::ptr::null_mut(),
        BPF_TASK_ITER_PROC_THREADS,
        invalid_task_nullable_callback,
        core::ptr::null_mut(),
    );

    bpf_for_each_task(
        core::ptr::null_mut(),
        BPF_TASK_ITER_ALL_PROCS,
        procs_cnt_callback,
        core::ptr::null_mut(),
    );

    bpf_for_each_task(
        cur_task,
        BPF_TASK_ITER_PROC_THREADS,
        proc_threads_cnt_callback,
        core::ptr::null_mut(),
    );

    bpf_for_each_task(
        core::ptr::null_mut(),
        BPF_TASK_ITER_ALL_THREADS,
        threads_cnt_callback,
        core::ptr::null_mut(),
    );
    bpf_rcu_read_unlock();
    0
}

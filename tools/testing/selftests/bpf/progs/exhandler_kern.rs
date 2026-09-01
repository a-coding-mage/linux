// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021, Oracle and/or its affiliates. */

/* Dependencies from the original C source:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 * #include <bpf/bpf_core_read.h>
 */

#[repr(C)]
pub struct callback_head {
    pub func: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct task_struct {
    pub task_works: *mut callback_head,
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut exception_triggered: u32 = 0;

#[unsafe(no_mangle)]
pub static mut test_pid: i32 = 0;

#[inline(always)]
fn barrier_var<T>(var: &mut T) {
    unsafe {
        core::arch::asm!("", inout(reg) var => _, options(nostack, preserves_flags));
    }
}

/* TRACE_EVENT(task_newtask,
 *         TP_PROTO(struct task_struct *p, u64 clone_flags)
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn trace_task_newtask(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let pid: i32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32;
    let mut work: *mut callback_head;
    let mut func: *mut core::ffi::c_void;

    let _ = clone_flags;

    if unsafe { test_pid } != pid {
        return 0;
    }

    /* To verify we hit an exception we dereference task->task_works->func.
     * If task work has been added,
     * - task->task_works is non-NULL; and
     * - task->task_works->func is non-NULL also (the callback function
     *   must be specified for the task work.
     *
     * However, for a newly-created task, task->task_works is NULLed,
     * so we know the exception handler triggered if task_works is
     * NULL and func is NULL.
     */
    work = unsafe { (*task).task_works };
    func = unsafe { (*work).func };
    /* Currently verifier will fail for `btf_ptr |= btf_ptr` * instruction.
     * To workaround the issue, use barrier_var() and rewrite as below to
     * prevent compiler from generating verifier-unfriendly code.
     */
    barrier_var(&mut work);
    if !work.is_null() {
        return 0;
    }
    barrier_var(&mut func);
    if !func.is_null() {
        return 0;
    }
    unsafe {
        exception_triggered = exception_triggered.wrapping_add(1);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

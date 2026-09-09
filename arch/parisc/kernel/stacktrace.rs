// SPDX-License-Identifier: GPL-2.0-only
/*
 * Stack trace management functions
 *
 *  Copyright (C) 2009-2021 Helge Deller <deller@gmx.de>
 *  based on arch/x86/kernel/stacktrace.c by Ingo Molnar <mingo@redhat.com>
 *  and parisc unwind functions by Randolph Chung <tausq@debian.org>
 *
 *  TODO: Userspace stacktrace (CONFIG_USER_STACKTRACE_SUPPORT)
 */

use core::ffi::c_void;

// Supplied by the kernel headers and architecture-specific unwind code.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unwind_frame_info {
    pub ip: usize,
}

pub type stack_trace_consume_fn = unsafe extern "C" fn(*mut c_void, usize) -> bool;

unsafe extern "C" {
    fn unwind_frame_init_task(
        info: *mut unwind_frame_info,
        task: *mut task_struct,
        arg: *mut c_void,
    );
    fn unwind_once(info: *mut unwind_frame_info) -> i32;
    fn __kernel_text_address(addr: usize) -> bool;
}

unsafe fn walk_stackframe(
    task: *mut task_struct,
    _regs: *mut pt_regs,
    func: stack_trace_consume_fn,
    cookie: *mut c_void,
) {
    let mut info: unwind_frame_info = core::mem::zeroed();

    unwind_frame_init_task(&mut info, task, core::ptr::null_mut());
    loop {
        if unwind_once(&mut info) < 0 || info.ip == 0 {
            break;
        }

        if __kernel_text_address(info.ip) && !func(cookie, info.ip) {
            break;
        }
    }
}

pub unsafe extern "C" fn arch_stack_walk(
    consume_entry: stack_trace_consume_fn,
    cookie: *mut c_void,
    task: *mut task_struct,
    regs: *mut pt_regs,
) {
    walk_stackframe(task, regs, consume_entry, cookie);
}

pub unsafe extern "C" fn arch_stack_walk_reliable(
    consume_entry: stack_trace_consume_fn,
    cookie: *mut c_void,
    task: *mut task_struct,
) -> i32 {
    walk_stackframe(task, core::ptr::null_mut(), consume_entry, cookie);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

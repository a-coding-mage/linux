// SPDX-License-Identifier: GPL-2.0
/*
 * Stack trace management functions
 *
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 */

use core::ffi::c_void;

// Declarations supplied by the kernel's scheduler, stacktrace, uaccess, and
// LoongArch unwind interfaces.
use crate::{pt_regs, stack_frame, task_struct, unwind_state};

pub type stack_trace_consume_fn = unsafe extern "C" fn(*mut c_void, usize) -> bool;

extern "C" {
    static mut current: *mut task_struct;

    fn thread_saved_fp(task: *mut task_struct) -> usize;
    fn thread_saved_ra(task: *mut task_struct) -> usize;
    fn unwind_start(state: *mut unwind_state, task: *mut task_struct, regs: *mut pt_regs);
    fn unwind_done(state: *mut unwind_state) -> bool;
    fn unwind_next_frame(state: *mut unwind_state);
    fn unwind_get_return_address(state: *mut unwind_state) -> usize;
    fn unwind_error(state: *mut unwind_state) -> bool;
    fn access_ok(addr: *const c_void, size: usize) -> bool;
    fn pagefault_disable();
    fn pagefault_enable();
    fn __copy_from_user_inatomic(to: *mut c_void, from: *const c_void, size: usize) -> usize;
    fn __builtin_frame_address(level: u32) -> *mut c_void;
    fn __builtin_return_address(level: u32) -> *mut c_void;
}

pub unsafe fn arch_stack_walk(
    consume_entry: stack_trace_consume_fn,
    cookie: *mut c_void,
    task: *mut task_struct,
    mut regs: *mut pt_regs,
) {
    let mut addr: usize;
    let mut dummyregs: pt_regs;

    if regs.is_null() {
        dummyregs = core::mem::zeroed();
        regs = &mut dummyregs;

        if task == current {
            (*regs).regs[3] = __builtin_frame_address(0) as usize;
            (*regs).csr_era = __builtin_return_address(0) as usize;
        } else {
            (*regs).regs[3] = thread_saved_fp(task);
            (*regs).csr_era = thread_saved_ra(task);
        }
        (*regs).regs[1] = 0;
        (*regs).regs[22] = 0;
    }

    let mut state: unwind_state = core::mem::zeroed();
    unwind_start(&mut state, task, regs);
    while !unwind_done(&mut state) {
        addr = unwind_get_return_address(&mut state);
        if addr == 0 || !consume_entry(cookie, addr) {
            break;
        }
        unwind_next_frame(&mut state);
    }
}

pub unsafe fn arch_stack_walk_reliable(
    consume_entry: stack_trace_consume_fn,
    cookie: *mut c_void,
    task: *mut task_struct,
) -> i32 {
    let mut dummyregs: pt_regs = core::mem::zeroed();
    let regs: *mut pt_regs = &mut dummyregs;

    if task == current {
        (*regs).regs[3] = __builtin_frame_address(0) as usize;
        (*regs).csr_era = __builtin_return_address(0) as usize;
        (*regs).regs[22] = 0;
    } else {
        (*regs).regs[3] = thread_saved_fp(task);
        (*regs).csr_era = thread_saved_ra(task);
        (*regs).regs[22] = (*task).thread.reg22;
    }
    (*regs).regs[1] = 0;

    let mut state: unwind_state = core::mem::zeroed();
    unwind_start(&mut state, task, regs);
    while !unwind_done(&mut state) && !unwind_error(&mut state) {
        let addr = unwind_get_return_address(&mut state);

        /*
         * A NULL or invalid return address probably means there's some
         * generated code which __kernel_text_address() doesn't know about.
         */
        if addr == 0 {
            return -22;
        }
        if !consume_entry(cookie, addr) {
            return -22;
        }
        unwind_next_frame(&mut state);
    }

    /* Check for stack corruption */
    if unwind_error(&mut state) {
        return -22;
    }
    0
}

unsafe fn copy_stack_frame(fp: usize, frame: *mut stack_frame) -> i32 {
    let mut ret: i32 = 1;
    let user_frame_tail = (fp - core::mem::size_of::<stack_frame>()) as *mut usize;

    if !access_ok(user_frame_tail as *const c_void, core::mem::size_of::<stack_frame>()) {
        return 0;
    }

    pagefault_disable();
    let err = __copy_from_user_inatomic(
        frame as *mut c_void,
        user_frame_tail as *const c_void,
        core::mem::size_of::<stack_frame>(),
    );
    if err != 0 || user_frame_tail as usize >= (*frame).fp {
        ret = 0;
    }
    pagefault_enable();
    ret
}

pub unsafe fn arch_stack_walk_user(
    consume_entry: stack_trace_consume_fn,
    cookie: *mut c_void,
    regs: *const pt_regs,
) {
    let mut fp = (*regs).regs[22];

    while fp != 0 && (fp & 0xf) == 0 {
        let mut frame: stack_frame = core::mem::zeroed();
        frame.fp = 0;
        frame.ra = 0;
        if copy_stack_frame(fp, &mut frame) == 0 {
            break;
        }
        if frame.ra == 0 {
            break;
        }
        if !consume_entry(cookie, frame.ra) {
            break;
        }
        fp = frame.fp;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

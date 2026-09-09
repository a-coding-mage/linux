/*
 * Stack trace management functions
 *
 *  Copyright (C) 2006-2009 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external Rust items.

use core::ffi::c_void;

extern "C" {
    fn unwind_start(state: *mut unwind_state, task: *mut task_struct,
                    regs: *mut pt_regs, stack: *mut c_void);
    fn unwind_done(state: *mut unwind_state) -> bool;
    fn unwind_next_frame(state: *mut unwind_state);
    fn unwind_get_return_address(state: *mut unwind_state) -> c_ulong;
    fn unwind_error(state: *mut unwind_state) -> bool;
    fn unwind_get_entry_regs(state: *mut unwind_state,
                             stack: *mut c_void) -> *mut pt_regs;
    fn user_mode(regs: *const pt_regs) -> bool;
    fn pagefault_disable();
    fn pagefault_enable();
    fn __access_ok(addr: *const c_void, size: usize) -> bool;
    fn __get_user<T>(value: *mut T, addr: *const T) -> c_int;
}

#[repr(C)]
pub struct stack_frame_user {
    next_fp: *const c_void,
    ret_addr: c_ulong,
}

unsafe fn copy_stack_frame(fp: *const stack_frame_user,
                            frame: *mut stack_frame_user) -> c_int {
    let mut ret: c_int;

    if !__access_ok(fp.cast::<c_void>(), core::mem::size_of::<stack_frame_user>()) {
        return 0;
    }

    ret = 1;
    pagefault_disable();
    if __get_user(&mut (*frame).next_fp, &(*fp).next_fp) != 0
        || __get_user(&mut (*frame).ret_addr, &(*fp).ret_addr) != 0
    {
        ret = 0;
    }
    pagefault_enable();

    ret
}

pub unsafe fn arch_stack_walk(consume_entry: stack_trace_consume_fn,
                              cookie: *mut c_void,
                              task: *mut task_struct,
                              regs: *mut pt_regs) {
    let mut state: unwind_state = core::mem::zeroed();
    let mut addr: c_ulong;

    if !regs.is_null() && consume_entry(cookie, (*regs).ip) == 0 {
        return;
    }

    unwind_start(&mut state, task, regs, core::ptr::null_mut());
    while !unwind_done(&mut state) {
        addr = unwind_get_return_address(&mut state);
        if addr == 0 || consume_entry(cookie, addr) == 0 {
            break;
        }
        unwind_next_frame(&mut state);
    }
}

pub unsafe fn arch_stack_walk_reliable(consume_entry: stack_trace_consume_fn,
                                       cookie: *mut c_void,
                                       task: *mut task_struct) -> c_int {
    let mut state: unwind_state = core::mem::zeroed();
    let mut regs: *mut pt_regs;
    let mut addr: c_ulong;

    unwind_start(&mut state, task, core::ptr::null_mut(), core::ptr::null_mut());
    while !unwind_done(&mut state) && !unwind_error(&mut state) {
        regs = unwind_get_entry_regs(&mut state, core::ptr::null_mut());
        if !regs.is_null() {
            // Success path for user tasks
            if user_mode(regs) {
                return 0;
            }

            /*
             * Kernel mode registers on the stack indicate an
             * in-kernel interrupt or exception (e.g., preemption
             * or a page fault), which can make frame pointers
             * unreliable.
             */
            // CONFIG_FRAME_POINTER is a build-time configuration condition.
            if cfg!(feature = "CONFIG_FRAME_POINTER") {
                return -22;
            }
        }

        addr = unwind_get_return_address(&mut state);

        /*
         * A NULL or invalid return address probably means there's some
         * generated code which __kernel_text_address() doesn't know
         * about.
         */
        if addr == 0 {
            return -22;
        }

        if consume_entry(cookie, addr) == 0 {
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

pub unsafe fn arch_stack_walk_user(consume_entry: stack_trace_consume_fn,
                                   cookie: *mut c_void,
                                   regs: *const pt_regs) {
    let mut fp: *const c_void = (*regs).bp as *const c_void;

    if consume_entry(cookie, (*regs).ip) == 0 {
        return;
    }

    loop {
        let mut frame: stack_frame_user = core::mem::zeroed();

        frame.next_fp = core::ptr::null();
        frame.ret_addr = 0;
        if copy_stack_frame(fp.cast::<stack_frame_user>(), &mut frame) == 0 {
            break;
        }
        if (fp as c_ulong) < (*regs).sp {
            break;
        }
        if frame.ret_addr == 0 {
            break;
        }
        if consume_entry(cookie, frame.ret_addr) == 0 {
            break;
        }
        fp = frame.next_fp;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Copyright (C) 2013 Richard Weinberger <richrd@nod.at>
 */

// Dependencies supplied by the surrounding kernel/UML environment are
// intentionally left external here.

use core::ffi::c_void;

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread_struct {
    pub segv_regs: *mut pt_regs,
}

#[repr(C)]
pub struct stacktrace_ops {
    pub address: Option<unsafe extern "C" fn(*mut c_void, usize, i32)>,
}

extern "C" {
    static mut current: *mut task_struct;
    static mut STACKSLOTS_PER_LINE: i32;

    fn printk(fmt: *const u8, ...);
    fn pr_cont(fmt: *const u8, ...);
    fn get_stack_pointer(task: *mut task_struct, regs: *mut pt_regs) -> *mut usize;
    fn kstack_end(stack: *mut usize) -> bool;
    fn dump_trace(task: *mut task_struct, ops: *const stacktrace_ops, data: *mut c_void);
    fn read_once_nocheck(value: *const usize) -> usize;
}

unsafe extern "C" fn _print_addr(data: *mut c_void, address: usize, reliable: i32) {
    let loglvl = data as *const u8;
    let empty = b"\0".as_ptr();
    let uncertain = b"? \0".as_ptr();

    printk(
        b"%s [<%08lx>] %s%pS\n\0".as_ptr(),
        loglvl,
        address,
        if reliable != 0 { empty } else { uncertain },
        address as *mut c_void,
    );
}

static stackops: stacktrace_ops = stacktrace_ops {
    address: Some(_print_addr),
};

pub unsafe extern "C" fn show_stack(
    task: *mut task_struct,
    mut stack: *mut usize,
    loglvl: *const u8,
) {
    let segv_regs = (*current).thread.segv_regs;
    let mut i: i32;

    if stack.is_null() {
        stack = get_stack_pointer(task, segv_regs);
    }

    printk(b"%sStack:\n\0".as_ptr(), loglvl);
    i = 0;
    while i < 3 * STACKSLOTS_PER_LINE {
        if kstack_end(stack) {
            break;
        }
        if i != 0 && (i % STACKSLOTS_PER_LINE) == 0 {
            pr_cont(b"\n\0".as_ptr());
        }
        pr_cont(b" %08lx\0".as_ptr(), read_once_nocheck(stack));
        stack = stack.add(1);
        i += 1;
    }

    printk(b"%sCall Trace:\n\0".as_ptr(), loglvl);
    dump_trace(
        if !task.is_null() { task } else { current },
        &stackops,
        loglvl as *mut c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

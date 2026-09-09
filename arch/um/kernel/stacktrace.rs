// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Copyright (C) 2013 Richard Weinberger <richard@nod.at>
 * Copyright (C) 2014 Google Inc., Author: Daniel Walter <dwalter@google.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stack_frame {
    pub next_frame: *mut stack_frame,
}

#[repr(C)]
pub struct thread_struct {
    pub segv_regs: *mut pt_regs,
}

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

#[repr(C)]
pub struct stack_trace {
    pub entries: *mut usize,
    pub nr_entries: usize,
    pub max_entries: usize,
}

#[repr(C)]
pub struct stacktrace_ops {
    pub address: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, i32)>,
}

extern "C" {
    fn get_frame_pointer(tsk: *mut task_struct, regs: *mut pt_regs) -> usize;
    fn get_stack_pointer(tsk: *mut task_struct, regs: *mut pt_regs) -> *mut usize;
    fn __kernel_text_address(addr: usize) -> i32;
    static mut current: *mut task_struct;
}

const THREAD_SIZE: usize = 0; // Supplied by the target architecture.

pub unsafe fn dump_trace(
    tsk: *mut task_struct,
    ops: *const stacktrace_ops,
    data: *mut core::ffi::c_void,
) {
    let mut reliable: i32 = 0;
    let mut sp: *mut usize;
    let mut bp: usize;
    let mut addr: usize;
    let segv_regs = (*tsk).thread.segv_regs;
    let mut frame: *mut stack_frame;

    bp = get_frame_pointer(tsk, segv_regs);
    sp = get_stack_pointer(tsk, segv_regs);

    frame = bp as *mut stack_frame;
    while ((sp as usize as isize) & ((THREAD_SIZE.wrapping_sub(1)) as isize)) != 0 {
        addr = core::ptr::read_volatile(sp);
        if __kernel_text_address(addr) != 0 {
            reliable = 0;
            if sp as usize == bp.wrapping_add(core::mem::size_of::<usize>()) {
                frame = if !frame.is_null() {
                    (*frame).next_frame
                } else {
                    core::ptr::null_mut()
                };
                bp = frame as usize;
                reliable = 1;
            }
            if let Some(address) = (*ops).address {
                address(data, addr, reliable);
            }
        }
        sp = sp.add(1);
    }
}

unsafe extern "C" fn save_addr(data: *mut core::ffi::c_void, address: usize, reliable: i32) {
    let trace = data as *mut stack_trace;

    if reliable == 0 {
        return;
    }
    if (*trace).nr_entries >= (*trace).max_entries {
        return;
    }

    *(*trace).entries.add((*trace).nr_entries) = address;
    (*trace).nr_entries += 1;
}

static DUMP_OPS: stacktrace_ops = stacktrace_ops {
    address: Some(save_addr),
};

unsafe fn __save_stack_trace(tsk: *mut task_struct, trace: *mut stack_trace) {
    dump_trace(tsk, &DUMP_OPS, trace as *mut core::ffi::c_void);
}

pub unsafe fn save_stack_trace(trace: *mut stack_trace) {
    __save_stack_trace(current, trace);
}

pub unsafe fn save_stack_trace_tsk(tsk: *mut task_struct, trace: *mut stack_trace) {
    __save_stack_trace(tsk, trace);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

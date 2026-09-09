// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *  Copyright (C) 2000, 2001, 2002 Andi Kleen, SuSE Labs
 *  Copyright (C) 2009  Matt Fleming
 *  Copyright (C) 2002 - 2012  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn pr_cont(fmt: *const core::ffi::c_char, ...);
    fn __get_user(val: *mut u32, ptr: *const u32) -> i32;
    fn __kernel_text_address(addr: usize) -> bool;
    fn kstack_end(sp: *mut usize) -> bool;
    fn user_mode(regs: *mut pt_regs) -> bool;
    fn unwind_stack(
        tsk: *mut task_struct,
        regs: *mut pt_regs,
        sp: *mut usize,
        ops: *const stacktrace_ops,
        data: *mut core::ffi::c_void,
    );
    fn debug_show_held_locks(tsk: *mut task_struct);
    fn task_stack_page(tsk: *mut task_struct) -> *mut core::ffi::c_void;
    fn ftrace_graph_get_ret_stack(
        task: *mut task_struct,
        graph: i32,
    ) -> *mut ftrace_ret_stack;
    static mut current: *mut task_struct;
    static mut current_stack_pointer: usize;
    static return_to_handler: unsafe extern "C" fn();
}

#[repr(C)]
pub struct stacktrace_ops {
    pub address: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, i32)>,
}

#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct,
}

#[repr(C)]
pub struct ftrace_ret_stack {
    pub ret: usize,
}

#[repr(C)]
pub struct task_struct {
    pub ret_stack: *mut ftrace_ret_stack,
    pub thread: thread_struct,
}

#[repr(C)]
pub struct thread_struct {
    pub sp: usize,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

pub const THREAD_SIZE: usize = 0; // Supplied by the architecture configuration.

pub unsafe fn dump_mem(
    str_: *const core::ffi::c_char,
    loglvl: *const core::ffi::c_char,
    bottom: usize,
    top: usize,
) {
    let mut p: usize;
    let mut i: i32;

    printk(
        b"%s%s(0x%08lx to 0x%08lx)\n\0".as_ptr() as *const _,
        loglvl,
        str_,
        bottom,
        top,
    );

    p = bottom & !31;
    while p < top {
        printk(
            b"%s%04lx: \0".as_ptr() as *const _,
            loglvl,
            p & 0xffff,
        );

        i = 0;
        while i < 8 {
            if p < bottom || p >= top {
                pr_cont(b"         \0".as_ptr() as *const _);
            } else {
                let mut val: u32 = 0;
                if __get_user(&mut val, p as *const u32) != 0 {
                    pr_cont(b"\n\0".as_ptr() as *const _);
                    return;
                }
                pr_cont(b"%08x \0".as_ptr() as *const _, val);
            }
            i += 1;
            p = p.wrapping_add(4);
        }
        pr_cont(b"\n\0".as_ptr() as *const _);
    }
}

pub unsafe fn printk_address(address: usize, reliable: i32) {
    pr_cont(
        b" [<%px>] %s%pS\n\0".as_ptr() as *const _,
        address as *mut core::ffi::c_void,
        if reliable != 0 {
            b"\0".as_ptr() as *const _
        } else {
            b"? \0".as_ptr() as *const _
        },
        address as *mut core::ffi::c_void,
    );
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
unsafe fn print_ftrace_graph_addr(
    addr: usize,
    data: *mut core::ffi::c_void,
    ops: *const stacktrace_ops,
    tinfo: *mut thread_info,
    graph: *mut i32,
) {
    let task = (*tinfo).task;
    let ret_stack: *mut ftrace_ret_stack;
    let ret_addr: usize;

    if addr != return_to_handler as usize || (*task).ret_stack.is_null() {
        return;
    }
    ret_stack = ftrace_graph_get_ret_stack(task, *graph);
    if ret_stack.is_null() {
        return;
    }
    ret_addr = (*ret_stack).ret;
    if let Some(address) = (*ops).address {
        address(data, ret_addr, 1);
    }
    *graph += 1;
}

#[cfg(not(CONFIG_FUNCTION_GRAPH_TRACER))]
unsafe fn print_ftrace_graph_addr(
    _addr: usize,
    _data: *mut core::ffi::c_void,
    _ops: *const stacktrace_ops,
    _tinfo: *mut thread_info,
    _graph: *mut i32,
) {
}

pub unsafe fn stack_reader_dump(
    task: *mut task_struct,
    regs: *mut pt_regs,
    sp: *mut usize,
    ops: *const stacktrace_ops,
    data: *mut core::ffi::c_void,
) {
    let context = ((sp as usize) & !(THREAD_SIZE - 1)) as *mut thread_info;
    let mut graph: i32 = 0;
    let mut cursor = sp;

    while !kstack_end(cursor) {
        let addr = *cursor;
        cursor = cursor.add(1);
        if __kernel_text_address(addr) {
            if let Some(address) = (*ops).address {
                address(data, addr, 1);
            }
            print_ftrace_graph_addr(addr, data, ops, context, &mut graph);
        }
    }
}

unsafe fn print_trace_address(data: *mut core::ffi::c_void, addr: usize, reliable: i32) {
    printk(b"%s\0".as_ptr() as *const _, data as *const core::ffi::c_char);
    printk_address(addr, reliable);
}

pub static print_trace_ops: stacktrace_ops = stacktrace_ops {
    address: Some(print_trace_address),
};

pub unsafe fn show_trace(
    tsk: *mut task_struct,
    sp: *mut usize,
    regs: *mut pt_regs,
    loglvl: *const core::ffi::c_char,
) {
    if !regs.is_null() && user_mode(regs) {
        return;
    }

    printk(b"%s\nCall trace:\n\0".as_ptr() as *const _, loglvl);
    unwind_stack(tsk, regs, sp, &print_trace_ops, loglvl as *mut _);
    pr_cont(b"\n\0".as_ptr() as *const _);

    if tsk.is_null() {
        tsk = current;
    }
    debug_show_held_locks(tsk);
}

pub unsafe fn show_stack(
    mut tsk: *mut task_struct,
    mut sp: *mut usize,
    loglvl: *const core::ffi::c_char,
) {
    if tsk.is_null() {
        tsk = current;
    }
    if tsk == current {
        sp = current_stack_pointer as *mut usize;
    } else {
        sp = (*tsk).thread.sp as *mut usize;
    }

    let stack = sp as usize;
    dump_mem(
        b"Stack: \0".as_ptr() as *const _,
        loglvl,
        stack,
        THREAD_SIZE.wrapping_add(task_stack_page(tsk) as usize),
    );
    show_trace(tsk, sp, core::ptr::null_mut(), loglvl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

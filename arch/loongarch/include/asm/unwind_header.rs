/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Most of this ideas comes from x86.
 *
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/sched.h, linux/ftrace.h, asm/ptrace.h, and asm/stacktrace.h.

#[repr(C)]
pub enum unwinder_type {
    UNWINDER_GUESS,
    UNWINDER_PROLOGUE,
    UNWINDER_ORC,
}

#[repr(C)]
pub struct unwind_state {
    pub r#type: i8, /* UNWINDER_XXX */
    pub stack_info: stack_info,
    pub task: *mut task_struct,
    pub first: bool,
    pub error: bool,
    pub reset: bool,
    pub graph_idx: i32,
    pub sp: c_ulong,
    pub fp: c_ulong,
    pub pc: c_ulong,
    pub ra: c_ulong,
}

extern "C" {
    pub fn default_next_frame(state: *mut unwind_state) -> bool;

    pub fn unwind_start(
        state: *mut unwind_state,
        task: *mut task_struct,
        regs: *mut pt_regs,
    );
    pub fn unwind_next_frame(state: *mut unwind_state) -> bool;
    pub fn unwind_get_return_address(state: *mut unwind_state) -> c_ulong;
}

#[inline]
pub unsafe fn unwind_done(state: *mut unwind_state) -> bool {
    (*state).stack_info.r#type == STACK_TYPE_UNKNOWN
}

#[inline]
pub unsafe fn unwind_error(state: *mut unwind_state) -> bool {
    (*state).error
}

pub const GRAPH_FAKE_OFFSET: usize =
    core::mem::size_of::<pt_regs>() - core::mem::offset_of!(pt_regs, regs[1]);

#[inline]
pub unsafe fn unwind_graph_addr(
    state: *mut unwind_state,
    pc: c_ulong,
    cfa: c_ulong,
) -> c_ulong {
    ftrace_graph_ret_addr(
        (*state).task,
        &mut (*state).graph_idx,
        pc,
        (cfa.wrapping_sub(GRAPH_FAKE_OFFSET as c_ulong)) as *mut c_ulong,
    )
}

#[inline(always)]
pub unsafe fn __unwind_start(
    state: *mut unwind_state,
    task: *mut task_struct,
    regs: *mut pt_regs,
) {
    core::ptr::write_bytes(state, 0, 1);
    if !regs.is_null() {
        (*state).sp = (*regs).regs[3];
        (*state).pc = (*regs).csr_era;
        (*state).ra = (*regs).regs[1];
        (*state).fp = (*regs).regs[22];
    } else if !task.is_null() && task != current {
        (*state).sp = thread_saved_fp(task);
        (*state).pc = thread_saved_ra(task);
        (*state).ra = 0;
        (*state).fp = 0;
    } else {
        (*state).sp = __builtin_frame_address(0) as c_ulong;
        (*state).pc = __builtin_return_address(0) as c_ulong;
        (*state).ra = 0;
        (*state).fp = 0;
    }
    (*state).task = task;
    get_stack_info((*state).sp, (*state).task, &mut (*state).stack_info);
    (*state).pc = unwind_graph_addr(state, (*state).pc, (*state).sp);
}

#[inline(always)]
pub unsafe fn __unwind_get_return_address(state: *mut unwind_state) -> c_ulong {
    if unwind_done(state) {
        return 0;
    }

    if __kernel_text_address((*state).pc) {
        (*state).pc
    } else {
        0
    }
}

#[cfg(CONFIG_UNWINDER_ORC)]
extern "C" {
    pub fn unwind_init();
    pub fn unwind_module_init(
        module: *mut module,
        orc_ip: *mut c_void,
        orc_ip_size: usize,
        orc: *mut c_void,
        orc_size: usize,
    );
}

#[cfg(not(CONFIG_UNWINDER_ORC))]
#[inline]
pub fn unwind_init() {}

#[cfg(not(CONFIG_UNWINDER_ORC))]
#[inline]
pub unsafe fn unwind_module_init(
    _module: *mut module,
    _orc_ip: *mut c_void,
    _orc_ip_size: usize,
    _orc: *mut c_void,
    _orc_size: usize,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

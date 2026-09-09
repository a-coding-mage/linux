/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/sched.h, linux/ftrace.h, linux/rethook.h, linux/llist.h,
// asm/ptrace.h, and asm/stacktrace.h.

/*
 * To use the stack unwinder it has to be initialized with unwind_start.
 * There four combinations for task and regs:
 * 1) task==NULL, regs==NULL: the unwind starts for the task that is currently
 *    running, sp/ip picked up from the CPU registers
 * 2) task==NULL, regs!=NULL: the unwind starts from the sp/ip found in
 *    the struct pt_regs of an interrupt frame for the current task
 * 3) task!=NULL, regs==NULL: the unwind starts for an inactive task with
 *    the sp picked up from task->thread.ksp and the ip picked up from the
 *    return address stored by __switch_to
 * 4) task!=NULL, regs!=NULL: the sp/ip are picked up from the interrupt
 *    frame 'regs' of a inactive task
 * If 'first_frame' is not zero unwind_start skips unwind frames until it
 * reaches the specified stack pointer.
 * The end of the unwinding is indicated with unwind_done, this can be true
 * right after unwind_start, e.g. with first_frame!=0 that can not be found.
 * unwind_next_frame skips to the next frame.
 * Once the unwind is completed unwind_error() can be used to check if there
 * has been a situation where the unwinder could not correctly understand the
 * tasks call chain.
 */
#[repr(C)]
pub struct unwind_state {
    pub stack_info: stack_info,
    pub stack_mask: ::core::ffi::c_ulong,
    pub task: *mut task_struct,
    pub regs: *mut pt_regs,
    pub sp: ::core::ffi::c_ulong,
    pub ip: ::core::ffi::c_ulong,
    pub graph_idx: ::core::ffi::c_int,
    pub kr_cur: *mut llist_node,
    pub reliable: bool,
    pub error: bool,
}

/* Recover the return address modified by rethook and ftrace_graph. */
#[inline]
pub unsafe fn unwind_recover_ret_addr(
    state: *mut unwind_state,
    mut ip: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    ip = ftrace_graph_ret_addr(
        (*state).task,
        &mut (*state).graph_idx,
        ip,
        (*state).sp as *mut ::core::ffi::c_void,
    );
    // CONFIG_RETHOOK controls this block in the C build.
    #[cfg(feature = "CONFIG_RETHOOK")]
    {
        if is_rethook_trampoline(ip) {
            ip = rethook_find_ret_addr((*state).task, (*state).sp, &mut (*state).kr_cur);
        }
    }
    ip
}

extern "C" {
    pub fn __unwind_start(
        state: *mut unwind_state,
        task: *mut task_struct,
        regs: *mut pt_regs,
        first_frame: ::core::ffi::c_ulong,
    );
    pub fn unwind_next_frame(state: *mut unwind_state) -> bool;
    pub fn unwind_get_return_address(state: *mut unwind_state) -> ::core::ffi::c_ulong;
}

#[inline]
pub unsafe fn unwind_done(state: *mut unwind_state) -> bool {
    (*state).stack_info.type_ == STACK_TYPE_UNKNOWN
}

#[inline]
pub unsafe fn unwind_error(state: *mut unwind_state) -> bool {
    (*state).error
}

#[inline(always)]
pub unsafe fn unwind_start(
    state: *mut unwind_state,
    mut task: *mut task_struct,
    regs: *mut pt_regs,
    mut first_frame: ::core::ffi::c_ulong,
) {
    if task.is_null() {
        task = current;
    }
    if first_frame == 0 {
        first_frame = get_stack_pointer(task, regs);
    }
    __unwind_start(state, task, regs, first_frame);
}

#[inline]
pub unsafe fn unwind_get_entry_regs(state: *mut unwind_state) -> *mut pt_regs {
    if unwind_done(state) {
        core::ptr::null_mut()
    } else {
        (*state).regs
    }
}

/*
 * C macro equivalent:
 * for (unwind_start(state, task, regs, first_frame);
 *      !unwind_done(state);
 *      unwind_next_frame(state))
 */

#[inline]
pub fn unwind_init() {}

#[inline]
pub fn unwind_module_init(
    _mod: *mut module,
    _orc_ip: *mut ::core::ffi::c_void,
    _orc_ip_size: usize,
    _orc: *mut ::core::ffi::c_void,
    _orc_size: usize,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

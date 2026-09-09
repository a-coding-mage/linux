// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel build are referenced here.

use core::ffi::c_int;

#[repr(C)]
pub struct task_struct {
    pub __state: u32,
}

#[repr(C)]
pub struct syscall_info_data {
    pub nr: i64,
    pub args: [u64; 6],
    pub instruction_pointer: u64,
}

#[repr(C)]
pub struct syscall_info {
    pub sp: u64,
    pub data: syscall_info_data,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut current: *mut task_struct;

    fn try_get_task_stack(task: *mut task_struct) -> bool;
    fn put_task_stack(task: *mut task_struct);
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn user_stack_pointer(regs: *mut pt_regs) -> u64;
    fn instruction_pointer(regs: *mut pt_regs) -> u64;
    fn syscall_get_nr(task: *mut task_struct, regs: *mut pt_regs) -> i64;
    fn syscall_get_arguments(task: *mut task_struct, regs: *mut pt_regs, args: *mut u64);
    fn wait_task_inactive(task: *mut task_struct, state: u32) -> u64;
}

unsafe fn collect_syscall(target: *mut task_struct, info: *mut syscall_info) -> c_int {
    let mut args = [0u64; 6];
    let regs: *mut pt_regs;

    if !try_get_task_stack(target) {
        // Task has no stack, so the task isn't in a syscall.
        core::ptr::write_bytes(info, 0, 1);
        (*info).data.nr = -1;
        return 0;
    }

    regs = task_pt_regs(target);
    if regs.is_null() {
        put_task_stack(target);
        return -11; // -EAGAIN
    }

    (*info).sp = user_stack_pointer(regs);
    (*info).data.instruction_pointer = instruction_pointer(regs);

    (*info).data.nr = syscall_get_nr(target, regs);
    if (*info).data.nr != -1 {
        syscall_get_arguments(target, regs, args.as_mut_ptr());
    }

    (*info).data.args[0] = args[0];
    (*info).data.args[1] = args[1];
    (*info).data.args[2] = args[2];
    (*info).data.args[3] = args[3];
    (*info).data.args[4] = args[4];
    (*info).data.args[5] = args[5];

    put_task_stack(target);
    0
}

/**
 * task_current_syscall - Discover what a blocked task is doing.
 * @target:       thread to examine
 * @info:         structure with the following fields:
 *                .sp        - filled with user stack pointer
 *                .data.nr   - filled with system call number or -1
 *                .data.args - filled with @maxargs system call arguments
 *                .data.instruction_pointer - filled with user PC
 *
 * If @target is blocked in a system call, returns zero with @info.data.nr
 * set to the call's number and @info.data.args filled in with its
 * arguments. Registers not used for system call arguments may not be available
 * and it is not kosher to use &struct user_regset calls while the system
 * call is still in progress. Note we may get this result if @target
 * has finished its system call but not yet returned to user mode, such
 * as when it's stopped for signal handling or syscall exit tracing.
 *
 * If @target is blocked in the kernel during a fault or exception,
 * returns zero with *@info.data.nr set to -1 and does not fill in
 * @info.data.args. If so, it's now safe to examine @target using
 * &struct user_regset get() calls as long as we're sure @target won't return
 * to user mode.
 *
 * Returns -%EAGAIN if @target does not remain blocked.
 */
pub unsafe fn task_current_syscall(
    target: *mut task_struct,
    info: *mut syscall_info,
) -> c_int {
    let ncsw: u64;
    let state: u32;

    if target == current {
        return collect_syscall(target, info);
    }

    state = core::ptr::read_volatile(&(*target).__state);
    if state == 0 {
        return -11; // -EAGAIN
    }

    ncsw = wait_task_inactive(target, state);
    if ncsw == 0
        || collect_syscall(target, info) != 0
        || wait_task_inactive(target, state) != ncsw
    {
        return -11; // -EAGAIN
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
